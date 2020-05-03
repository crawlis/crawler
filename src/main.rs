mod crawler;

use crawler::{Crawler, CrawlerConfig};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::process;
use url::Url;

const STARTING_URL: &str = "https://www.lemonde.fr/";

fn main() {
    let starting_url: Url = Url::parse(STARTING_URL).expect("Starting url is not valid");
    dotenv().expect("No .env file!");

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem parsing env values: {}", err);
        process::exit(1);
    });
    let crawler = Crawler::new(config);
    crawler.run(starting_url).unwrap_or_else(|err| {
        eprintln!("Problem running the crawler: {}", err);
        process::exit(1);
    });
}

fn get_config() -> Result<CrawlerConfig, Box<dyn Error>> {
    let keeper_host = env::var("KEEPER_HOST")?;
    let keeper_port = env::var("KEEPER_PORT")?;

    let keeper_url = format!("http://{}:{}", keeper_host, keeper_port);
    let keeper_url = Url::parse(&keeper_url)?;

    let config = CrawlerConfig::new(keeper_url);
    Ok(config)
}
