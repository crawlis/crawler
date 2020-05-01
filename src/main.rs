use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use url::Url;

const STARTING_DOMAIN: &str = "https://www.lemonde.fr/";

fn main() -> Result<(), Box<dyn Error>> {
    let domains = crawl(STARTING_DOMAIN)?;
    println!("{:#?}", domains);
    Ok(())
}

fn crawl(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Crawling {}", url);
    let response = reqwest::blocking::get(url)?;
    let links = Document::from_read(response)?
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|link| String::from(link))
        .collect::<Vec<String>>();
    let domains = extract_domains(&links);
    Ok(domains)
}

fn extract_domains(links: &Vec<String>) -> Vec<String> {
    links
        .iter()
        .filter_map(|link| Url::parse(link).map(|url| Some(url)).unwrap_or(None))
        .filter_map(|url| url.domain().map(|domain| domain.to_string()))
        .collect::<Vec<String>>()
}
