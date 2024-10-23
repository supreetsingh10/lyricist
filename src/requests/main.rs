mod response;

use core::panic;

use libreq::{generate_client, response::Root2, Lyrics};

#[tokio::main]
async fn main() {
    let c = generate_client().unwrap();

    let r = match c
        .get_lyrics(String::from("t: paranoid, a: blacksabbath"))
        .await
    {
        Ok(s) => {
            println!("{:?}", s);
            match s.json::<Vec<Root2>>().await {
                Ok(j) => j,
                Err(e) => panic!("JSON failed {}", e.to_string()),
            }
        }
        Err(e) => panic!("WHAT THE FUCK {}", e),
    };

    println!("{:?}", r);
}
