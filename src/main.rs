use select::document::Document;
use select::predicate::Name;
use std::collections::HashMap;
use std::error::Error;

const STARTING_DOMAIN: &str = "https://www.lemonde.fr/";

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    map.entry(String::from(STARTING_DOMAIN))
        .or_insert(crawl(STARTING_DOMAIN).unwrap_or(Vec::new()));

    for key in map.keys() {
        let mut map_copy = map.clone();
        let crawling_urls = map_copy.get(key).unwrap().clone();
        for url in crawling_urls {
            map_copy
                .entry(url.clone())
                .or_insert(crawl(&url).unwrap_or(Vec::new()));
        }
    }
}

fn crawl(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Crawling {}", url);
    let response = reqwest::blocking::get(url)?;
    let urls = Document::from_read(response)?
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|url| String::from(url))
        .collect::<Vec<String>>();
    Ok(urls)
}
