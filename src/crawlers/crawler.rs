use reqwest::Result;
use std::time::Duration;
use reqwest::ClientBuilder;

#[tokio::main]
pub async fn run() -> Result<()> {
    let url = "";
    let request_url = format!("{}",url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send().await?;

    println!("{:?}",response);


    Ok(())
}