use reqwest::Client;
use std::io::Result;
mod requests;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let header_map = requests::generate_header();

    Ok(())
}
