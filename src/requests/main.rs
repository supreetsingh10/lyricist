mod response;

use core::panic;
use libreq::generate_client;
use libreq::Lyrics;

use std::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = match generate_client() {
        Ok(c) => c,
        Err(e) => panic!("Failed to generate the client {}", e.to_string()),
    };

    let _ = client.get_lyrics("Black Sabbath N.I.B.").await;

    Ok(())
}
