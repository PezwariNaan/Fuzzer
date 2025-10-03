use std::time::Duration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IpApi {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust-reqwest/boilerplate"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .build()?;
    
    let text = client
        .get("https://testing.com")
 //       .query(None)
        .send()
        .await?
        .text()
        .await?;
    
    let ip_info: IpApi = client
        .get("https://testing.com")
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
