mod response;
use response::Root;

use core::panic;
use libreq::generate_client;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = match generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate the client {}", e.to_string()),
    };

    let req = client.get("https://genius-song-lyrics1.p.rapidapi.com/search/?per_page=5&page=1");

    let t = req
        .query(&[("q", "n.i.b.")])
        .send()
        .await
        .unwrap()
        .json::<Root>()
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&t).unwrap());
    Ok(())
}
