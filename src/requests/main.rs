mod response;

use core::panic;
use libreq::generate_client;
use libreq::Lyrics;
use response::Root;
use std::collections::HashMap;

use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = match generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate the client {}", e.to_string()),
    };

    let mut hmap: HashMap<&str, &str> = HashMap::new();
    hmap.insert("t", "n.i.b.");
    hmap.insert("a", "Black Sabbath");
    let j = client
        .get_lyrics(&hmap)
        .await
        .unwrap()
        .json::<Root>()
        .await
        .unwrap();

    println!("{:?}", j);

    Ok(())
}
