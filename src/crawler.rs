use select::document::Document;
use select::predicate::Name;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use url::Url;

#[derive(Deserialize, Serialize)]
struct CrawlingOutput {
    parent: String,
    childs: Vec<String>,
}

impl CrawlingOutput {
    fn from(parent: String, childs: Vec<String>) -> CrawlingOutput {
        CrawlingOutput { parent, childs }
    }
}

pub fn run(starting_url: &str, n_loops: u16) -> Result<(), Box<dyn Error>> {
    let keeper_client = reqwest::blocking::Client::new();
    let mut map: HashMap<Url, HashSet<Url>> = HashMap::new();
    let mut queue: VecDeque<Url> = VecDeque::new();
    queue.push_back(Url::parse(starting_url)?);

    let mut i: u16 = 0;
    while !queue.is_empty() && i < n_loops {
        let crawling_url: Url = queue.pop_front().unwrap();
        if !map.contains_key(&crawling_url) {
            let new_links: Vec<String> = crawl(&crawling_url).unwrap_or(Vec::new());
            let new_urls: HashSet<Url> = links_to_urls(&crawling_url, new_links);
            for new_url in new_urls.iter() {
                queue.push_back(new_url.clone());
            }
            let crawling_output = CrawlingOutput::from(
                crawling_url.clone().into_string(),
                new_urls
                    .clone()
                    .iter()
                    .map(|new_url| new_url.clone().into_string())
                    .collect::<Vec<String>>(),
            );
            map.insert(crawling_url, new_urls);
            send_to_keeper(&keeper_client, crawling_output)?;
        }
        i += 1;
    }
    println!("{:#?}", map);
    Ok(())
}

fn crawl(url: &Url) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Crawling {}", url);
    let response = reqwest::blocking::get(url.clone())?;
    let links = Document::from_read(response)?
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|link| String::from(link))
        .collect::<Vec<String>>();
    Ok(links)
}

fn links_to_urls(parent_url: &Url, links: Vec<String>) -> HashSet<Url> {
    let mut urls: HashSet<Url> = HashSet::new();
    for link in links.iter() {
        if let Ok(url) = Url::parse(link) {
            urls.insert(url);
        } else if let Ok(url) = parent_url.join(link) {
            urls.insert(url);
        }
    }
    return urls;
}

fn send_to_keeper(
    keeper_client: &reqwest::blocking::Client,
    crawling_output: CrawlingOutput,
) -> Result<(), Box<dyn std::error::Error>> {
    keeper_client
        .post("http://0.0.0.0:3030")
        .body(serde_json::to_string(&crawling_output).unwrap())
        .send()?;
    Ok(())
}
