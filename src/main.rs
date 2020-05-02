use select::document::Document;
use select::predicate::Name;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use url::Url;

const STARTING_DOMAIN: &str = "https://www.lemonde.fr/";
const N_LOOPS: u16 = 1000;

fn main() {
    let mut map: HashMap<Url, HashSet<Url>> = HashMap::new();
    let mut queue: VecDeque<Url> = VecDeque::new();
    queue.push_back(Url::parse(STARTING_DOMAIN).unwrap());

    let mut i: u16 = 0;
    while !queue.is_empty() && i < N_LOOPS {
        let crawling_url: Url = queue.pop_front().unwrap();
        if !map.contains_key(&crawling_url) {
            let new_links: Vec<String> = crawl(crawling_url.clone()).unwrap_or(Vec::new());
            let new_urls: HashSet<Url> = links_to_urls(crawling_url.clone(), new_links);
            for new_url in new_urls.iter() {
                queue.push_back(new_url.clone());
            }
            map.insert(crawling_url, new_urls);
        }
        i += 1;
    }
    println!("{:#?}", map);
}

fn crawl(url: Url) -> Result<Vec<String>, Box<dyn Error>> {
    println!("Crawling {}", url);
    let response = reqwest::blocking::get(url)?;
    let links = Document::from_read(response)?
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|link| String::from(link))
        .collect::<Vec<String>>();
    Ok(links)
}

fn links_to_urls(parent_url: Url, links: Vec<String>) -> HashSet<Url> {
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
