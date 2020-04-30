use http::method::Method;
use http::Request;
use select::document::Document;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri("https://www.rust-lang.org/en-US/")
        .body(())?;

    Ok(())
}
