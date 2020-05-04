mod crawler;

use crawler::{Crawler, CrawlerConfig};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::process;
use url::Url;

fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing crawler config: {}", err);
        process::exit(1);
    });
    let crawler = Crawler::new(config);
    crawler.run().unwrap_or_else(|err| {
        eprintln!("Problem running the crawler: {}", err);
        process::exit(1);
    });
}

fn get_config() -> Result<CrawlerConfig, Box<dyn Error>> {
    let starting_url = env::var("STARTING_URL")?;
    let starting_url = Url::parse(&starting_url)?;

    let keeper_host = env::var("KEEPER_HOST")?;
    let keeper_port = env::var("KEEPER_PORT")?;
    let keeper_url = format!("http://{}:{}", keeper_host, keeper_port);
    let keeper_url = Url::parse(&keeper_url)?;

    let max_retries = env::var("MAX_RETRIES")?.parse::<u32>()?;

    let config = CrawlerConfig::new(keeper_url, max_retries, starting_url);
    Ok(config)
}
