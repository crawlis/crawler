use crate::nats::{NatsPublisher, NatsSubscriber};
use select::document::Document;
use select::predicate::Name;
use serde::Serialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::error;
use std::hash::{Hash, Hasher};
use std::io;
use std::iter::FromIterator;
use url::Url;

pub struct CrawlerConfig {
    nats_subscriber_uri: String,
    nats_subscriber_subject: String,
    nats_publisher_uri: String,
    nats_publisher_subject: String,
}

impl CrawlerConfig {
    pub fn new(
        nats_subscriber_uri: String,
        nats_subscriber_subject: String,
        nats_publisher_uri: String,
        nats_publisher_subject: String,
    ) -> CrawlerConfig {
        CrawlerConfig {
            nats_subscriber_uri,
            nats_subscriber_subject,
            nats_publisher_uri,
            nats_publisher_subject,
        }
    }
}

pub struct Crawler {
    _config: CrawlerConfig,
    nats_publisher: NatsPublisher,
    nats_subscriber: NatsSubscriber,
}

impl<'a> Crawler {
    pub fn new(config: CrawlerConfig) -> io::Result<Crawler> {
        let nats_subscriber =
            NatsSubscriber::new(&config.nats_subscriber_uri, &config.nats_subscriber_subject)?;
        let nats_publisher =
            NatsPublisher::new(&config.nats_publisher_uri, &config.nats_publisher_subject)?;
        Ok(Crawler {
            _config: config,
            nats_subscriber,
            nats_publisher,
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn error::Error>> {
        loop {
            if let Some(message) = self.nats_subscriber.get_next_message() {
                match serde_json::from_slice::<String>(&message.data) {
                    Ok(crawling_url) => match crawl_url(&crawling_url).await {
                        Ok(crawling_results) => {
                            if let Err(err) = self.publish_results(&crawling_results) {
                                eprintln!("Problem sending results: {}", err);
                            };
                        }
                        Err(err) => eprintln!("Problem crawling url {} : {}", crawling_url, err),
                    },
                    Err(err) => eprintln!("Could not deserialize message: {}", err),
                }
            }
        }
    }

    fn publish_results(&self, crawling_results: &CrawlingResults) -> io::Result<()> {
        let key = format!("{}", calculate_hash(crawling_results));
        let message = serde_json::to_vec(crawling_results)?;
        self.nats_publisher.publish(&key, message)
    }
}

#[derive(Serialize, Hash)]
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

async fn crawl_url(crawling_url: &str) -> Result<CrawlingResults, Box<dyn error::Error>> {
    // We ensure that the url is valid
    let crawling_url = Url::parse(crawling_url)?;
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

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
