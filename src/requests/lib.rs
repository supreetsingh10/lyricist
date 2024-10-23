pub mod response;

use core::future::Future;
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE},
    Client, Error, Response,
};

// The trailing question mark was an issue lol
const URL: &str = "https://musixmatch-lyrics-songs.p.rapidapi.com/songs/lyrics";

pub trait Lyrics {
    type Out;
    fn get_lyrics(&self, query: String) -> impl Future<Output = Self::Out>;
}

impl Lyrics for Client {
    type Out = Result<Response, Error>;

    async fn get_lyrics(&self, query: String) -> Result<Response, Error> {
        let v: Vec<&str> = query
            .split(',')
            .into_iter()
            .flat_map(|s| s.trim().split(':'))
            .collect();

        let mut q_vec: Vec<(&str, &str)> = Vec::new();

        for (index, val) in v.iter().enumerate() {
            if val.eq_ignore_ascii_case("t") {
                let vals = v.get(index + 1).expect("VALUE FAILED");
                q_vec.push(("t", vals.trim()));
            } else if val.eq_ignore_ascii_case("a") {
                let vals = v.get(index + 1).expect("VALUE FAILED");
                q_vec.push(("a", vals.trim()));
            }
        }

        self.get(URL).query(&q_vec).send().await
    }
}

pub fn generate_client() -> Result<Client, reqwest::Error> {
    let v: Vec<_> = std::env::vars()
        .into_iter()
        .filter(|k| k.0.find("x_rapid_api").is_some())
        .collect();

    let mut header = HeaderMap::new();

    header.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    for (key, vals) in v.iter() {
        if key.eq("x_rapid_api_key") {
            assert!(header
                .insert("x-rapidapi-key", vals.parse().unwrap())
                .is_none());
        } else {
            assert!(header
                .insert("x-rapidapi-host", vals.parse().unwrap())
                .is_none());
        }
    }

    assert_ne!(header.len(), 0);
    Client::builder().default_headers(header).build()
}
