use std::time::Duration;
use reqwest::header::{USER_AGENT};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/get";

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let resp = client
        .get(url)
        .header(USER_AGENT, "hello-there")
        .send()
        .await?;

    let body = resp.text().await?;

    println!("{:?}", body);

    Ok(())
}
