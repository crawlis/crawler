use crate::nats::NatsPublisher;
use select::document::Document;
use select::predicate::Name;
use serde::Serialize;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::iter::FromIterator;
use url::Url;

pub struct CrawlerConfig {
    nats_publisher_uri: String,
    starting_url: String,
}

impl CrawlerConfig {
    pub fn new(nats_publisher_uri: String, starting_url: String) -> CrawlerConfig {
        CrawlerConfig {
            nats_publisher_uri,
            starting_url,
        }
    }
}

pub struct Crawler {
    config: CrawlerConfig,
    nats_publisher: NatsPublisher,
}

impl<'a> Crawler {
    pub fn new(config: CrawlerConfig) -> Result<Crawler, std::io::Error> {
        let nats_publisher = NatsPublisher::new(&config.nats_publisher_uri)?;
        Ok(Crawler {
            config,
            nats_publisher,
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut queue: VecDeque<String> = VecDeque::new();
        queue.push_back(self.config.starting_url.clone());

        while !queue.is_empty() {
            let crawling_url = queue.pop_front().unwrap();
            match crawl(&crawling_url).await {
                Ok(crawling_results) => {
                    crawling_results
                        .urls
                        .iter()
                        .for_each(|url| queue.push_back(url.clone()));
                    if let Err(err) = publish_to_nats(&self.nats_publisher, &crawling_results) {
                        eprintln!("Problem sending results: {}", err);
                    };
                }
                Err(err) => eprintln!("Problem crawling url {} : {}", crawling_url, err),
            }
        }
        Ok(())
    }
}

async fn crawl(crawling_url: &str) -> Result<CrawlingResults, Box<dyn Error>> {
    // We ensure that the url is valid
    let crawling_url = Url::parse(crawling_url)?;
    println!("Crawling {}", crawling_url);
    let mut found_urls: HashSet<Url> = HashSet::new();
    let response = reqwest::get(crawling_url.clone()).await?.text().await?;
    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .for_each(|link| {
            if let Ok(url) = Url::parse(link) {
                found_urls.insert(url);
            } else if let Ok(url) = crawling_url.join(link) {
                found_urls.insert(url);
            }
        });
    let crawling_results = CrawlingResults::from(crawling_url, Vec::from_iter(found_urls));
    Ok(crawling_results)
}

#[derive(Serialize)]
struct CrawlingResults {
    parent: String,
    urls: Vec<String>,
}

impl CrawlingResults {
    fn new(parent: String, urls: Vec<String>) -> CrawlingResults {
        CrawlingResults { parent, urls }
    }

    fn from(parent: Url, urls: Vec<Url>) -> CrawlingResults {
        CrawlingResults::new(
            parent.into_string(),
            urls.iter().map(|url| url.clone().into_string()).collect(),
        )
    }
}

fn publish_to_nats(
    publisher: &NatsPublisher,
    crawling_results: &CrawlingResults,
) -> Result<(), std::io::Error> {
    let key = format!("crawling.{}", crawling_results.parent);
    let value = serde_json::to_vec(crawling_results)?;
    publisher.publish(&key, value)
}
