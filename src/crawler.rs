use futures::{stream, StreamExt};
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::{thread, time};
use url::Url;

pub struct CrawlerConfig {
    keeper_url: Url,
    starting_url: Url,
}

impl CrawlerConfig {
    pub fn new(keeper_url: Url, starting_url: Url) -> CrawlerConfig {
        CrawlerConfig {
            keeper_url,
            starting_url,
        }
    }
}

struct CrawlingResult {
    parent: Url,
    value: Url,
}

impl CrawlingResult {
    fn from(parent: Url, value: Url) -> CrawlingResult {
        CrawlingResult { parent, value }
    }
}

pub struct Crawler {
    config: CrawlerConfig,
}

impl Crawler {
    pub fn new(config: CrawlerConfig) -> Crawler {
        Crawler { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.wait_for_keeper_conn(time::Duration::from_secs(2), 10)
            .await?;
        let mut map: HashMap<Url, HashSet<Url>> = HashMap::new();
        let mut queue: VecDeque<Url> = VecDeque::new();
        queue.push_back(self.config.starting_url.clone());

        while !queue.is_empty() {
            let crawling_url = queue.pop_front().unwrap();
            if !map.contains_key(&crawling_url) {
                if let Ok(crawling_results) = crawl(&crawling_url).await {
                    crawling_results
                        .iter()
                        .for_each(|crawling_result| queue.push_back(crawling_result.value.clone()));
                    send(&crawling_results, &self.config.keeper_url, 10).await?;
                    map.insert(
                        crawling_url.clone(),
                        crawling_results
                            .iter()
                            .map(|crawling_result| crawling_result.value.clone())
                            .collect(),
                    );
                }
            }
        }
        Ok(())
    }

    async fn wait_for_keeper_conn(
        &self,
        refresh_time: time::Duration,
        max_retries: u32,
    ) -> Result<(), String> {
        let client = Client::new();
        for i in 0..max_retries {
            println!("Waiting for database connexion, attempt number: {}", i);
            match client.get(self.config.keeper_url.clone()).send().await {
                Ok(response) => {
                    println!("Keeper connexion is ready");
                    return Ok(());
                }
                Err(_) => println!("Keeper connexion is not ready yet"),
            }
            thread::sleep(refresh_time);
        }
        Err(format!(
            "Could not connect to keeper after {} attempts",
            max_retries
        ))
    }
}

async fn crawl(url: &Url) -> Result<Vec<CrawlingResult>, Box<dyn Error>> {
    println!("Crawling {}", url);
    let mut found_urls: HashSet<Url> = HashSet::new();
    let response = reqwest::get(url.clone()).await?.text().await?;
    Document::from(response.as_str())
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .for_each(|link| {
            if let Ok(url) = Url::parse(link) {
                found_urls.insert(url);
            } else if let Ok(url) = url.join(link) {
                found_urls.insert(url);
            }
        });
    let crawling_results = found_urls
        .iter()
        .map(|found_url| CrawlingResult::from(url.clone(), found_url.clone()))
        .collect();
    Ok(crawling_results)
}

#[derive(Serialize, Deserialize)]
struct NewNodeRequestMessage {
    parent: String,
    value: String,
}

impl NewNodeRequestMessage {
    fn from_crawling_result(crawling_result: &CrawlingResult) -> NewNodeRequestMessage {
        NewNodeRequestMessage {
            parent: crawling_result.parent.to_string(),
            value: crawling_result.value.to_string(),
        }
    }
}

async fn send(
    crawling_results: &Vec<CrawlingResult>,
    url: &Url,
    max_parrallel_requests: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let responses = stream::iter(crawling_results)
        .map(|crawling_result| {
            let client = &client;
            async move {
                client
                    .post(url.clone())
                    .json(&NewNodeRequestMessage::from_crawling_result(
                        crawling_result,
                    ))
                    .send()
                    .await
            }
        })
        .buffer_unordered(max_parrallel_requests);

    responses
        .for_each(|b| async {
            if let Err(e) = b {
                eprintln!("Got an error: {}", e)
            }
        })
        .await;
    Ok(())
}
