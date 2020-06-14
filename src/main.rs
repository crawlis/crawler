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
    let nats_subscriber_uri = env::var("NATS_URI")?;
    let nats_subscriber_uri = Url::parse(&nats_subscriber_uri)?;

    let nats_subscriber_subject = String::from("url");

    let nats_publisher_uri = env::var("NATS_URI")?;
    let nats_publisher_uri = Url::parse(&nats_publisher_uri)?;

    let nats_publisher_subject = String::from("node");

    let config = CrawlerConfig::new(
        nats_subscriber_uri.into_string(),
        nats_subscriber_subject,
        nats_publisher_uri.into_string(),
        nats_publisher_subject,
    );
    Ok(config)
}
