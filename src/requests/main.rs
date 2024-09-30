mod requests;
mod response;

use response::Root;

use core::panic;
use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = match requests::generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate the client {}", e.to_string()),
    };

    let req = client
        .get("https://genius-song-lyrics1.p.rapidapi.com/search/?q=Black%20Sabbath%20&per_page=5&page=1")
        .send()
        .await
        .unwrap()
        .json::<Root>()
        .await.unwrap();

    println!("{}", serde_json::to_string_pretty(&req).unwrap());
    Ok(())
}
