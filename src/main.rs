use crawler::crawler::{Crawler, CrawlerConfig};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::process;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = get_config().unwrap_or_else(|err| {
        eprintln!("Problem initializing crawler config: {}", err);
        process::exit(1);
    });
    let crawler = Crawler::new(config).unwrap_or_else(|err| {
        eprintln!("Problem initializing crawler config: {}", err);
        process::exit(1);
    });
    crawler.run().await.unwrap_or_else(|err| {
        eprintln!("Problem running the crawler: {}", err);
        process::exit(1);
    });
}

fn get_config() -> Result<CrawlerConfig, Box<dyn Error>> {
    let starting_url = env::var("STARTING_URL")?;
    let starting_url = Url::parse(&starting_url)?;

    let nats_uri = env::var("NATS_URI")?;
    let nats_uri = Url::parse(&nats_uri)?;

    let config = CrawlerConfig::new(nats_uri.into_string(), starting_url.into_string());
    Ok(config)
}
