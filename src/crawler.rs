use select::document::Document;
use select::predicate::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use url::Url;

pub struct CrawlerConfig {
    keeper_url: Url,
}

impl CrawlerConfig {
    pub fn new(keeper_url: Url) -> CrawlerConfig {
        CrawlerConfig { keeper_url }
    }
}

pub struct Crawler {
    client: reqwest::blocking::Client,
    keeper_url: Url,
}

impl Crawler {
    pub fn new(config: CrawlerConfig) -> Crawler {
        Crawler {
            client: reqwest::blocking::Client::new(),
            keeper_url: config.keeper_url,
        }
    }

    pub fn run(&self, starting_url: Url) -> Result<(), Box<dyn Error>> {
        let mut map: HashMap<Url, HashSet<Url>> = HashMap::new();
        let mut queue: VecDeque<Url> = VecDeque::new();
        queue.push_back(starting_url);

        while !queue.is_empty() {
            let crawling_url: Url = queue.pop_front().unwrap();
            if !map.contains_key(&crawling_url) {
                if let Ok(crawling_results) = Crawler::crawl(crawling_url) {
                    for child in crawling_results.children.iter() {
                        queue.push_back(child.clone());
                    }
                    map.insert(
                        crawling_results.parent.clone(),
                        crawling_results.children.clone(),
                    );
                    self.send_to_keeper(CrawlerResultsRequestBody::from(crawling_results))?;
                }
            }
        }
        Ok(())
    }

    fn crawl(url: Url) -> Result<CrawlerResults, Box<dyn Error>> {
        println!("Crawling {}", url);
        let mut found_urls: HashSet<Url> = HashSet::new();
        let response = reqwest::blocking::get(url.clone())?;
        Document::from_read(response)?
            .find(Name("a"))
            .filter_map(|a| a.attr("href"))
            .for_each(|link| {
                if let Ok(url) = Url::parse(link) {
                    found_urls.insert(url);
                } else if let Ok(url) = url.join(link) {
                    found_urls.insert(url);
                }
            });
        Ok(CrawlerResults::from(url, found_urls))
    }

    fn send_to_keeper(
        &self,
        body: CrawlerResultsRequestBody,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .post(self.keeper_url.clone())
            .body(serde_json::to_string(&body).unwrap())
            .send()?;
        Ok(())
    }
}

struct CrawlerResults {
    parent: Url,
    children: HashSet<Url>,
}

impl CrawlerResults {
    fn from(parent: Url, children: HashSet<Url>) -> CrawlerResults {
        CrawlerResults { parent, children }
    }
}

#[derive(Deserialize, Serialize)]
struct CrawlerResultsRequestBody {
    parent: String,
    children: Vec<String>,
}

impl CrawlerResultsRequestBody {
    fn from(crawling_results: CrawlerResults) -> CrawlerResultsRequestBody {
        CrawlerResultsRequestBody {
            parent: crawling_results.parent.into_string(),
            children: crawling_results
                .children
                .iter()
                .map(|child| child.clone().into_string())
                .collect::<Vec<String>>(),
        }
    }
}
