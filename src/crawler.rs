use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
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
    max_retries: u32,
    starting_url: Url,
}

impl CrawlerConfig {
    pub fn new(keeper_url: Url, max_retries: u32, starting_url: Url) -> CrawlerConfig {
        CrawlerConfig {
            keeper_url,
            max_retries,
            starting_url,
        }
    }
}

pub struct Crawler {
    client: Client,
    config: CrawlerConfig,
}

impl Crawler {
    pub fn new(config: CrawlerConfig) -> Crawler {
        Crawler {
            client: Client::new(),
            config,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut map: HashMap<Url, HashSet<Url>> = HashMap::new();
        let mut queue: VecDeque<Url> = VecDeque::new();
        queue.push_back(self.config.starting_url.clone());

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
                    self.send_to_keeper(CrawlerResultsMessage::from(crawling_results))?;
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
        body: CrawlerResultsMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut retries: u32 = 0;
        let mut response_status = StatusCode::CONTINUE;
        while response_status != StatusCode::OK && retries < self.config.max_retries {
            retries += 1;
            response_status = match self
                .client
                .post(self.config.keeper_url.clone())
                .body(serde_json::to_string(&body).unwrap())
                .send()
            {
                Ok(response) => response.status(),
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };
        }
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
struct CrawlerResultsMessage {
    parent: String,
    children: Vec<String>,
}

impl CrawlerResultsMessage {
    fn from(crawling_results: CrawlerResults) -> CrawlerResultsMessage {
        CrawlerResultsMessage {
            parent: crawling_results.parent.into_string(),
            children: crawling_results
                .children
                .iter()
                .map(|child| child.clone().into_string())
                .collect::<Vec<String>>(),
        }
    }
}
