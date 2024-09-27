use reqwest::Client;
use std::io::Result;
mod requests;

fn main() -> Result<()> {
    let builder = Client::builder();
    println!("{:?}", builder);
    Ok(())
}

