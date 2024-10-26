pub mod response;

use core::future::Future;
use reqwest::{header::HeaderMap, Client, Error, Response};

const URL: &str = "https://musixmatch-lyrics-songs.p.rapidapi.com/songs/lyrics";

pub trait Lyrics {
    type Out;
    fn get_lyrics(&self, query: String) -> impl Future<Output = Self::Out>;
}

impl Lyrics for Client {
    type Out = Result<Response, Error>;

    async fn get_lyrics(&self, query: String) -> Result<Response, Error> {
        let v: Vec<&str> = query.split(',').flat_map(|s| s.trim().split(':')).collect();

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

        q_vec.push(("type", "json"));

        assert_eq!(q_vec.len(), 3);
        self.get(URL).query(&q_vec).send().await
    }
}

pub fn generate_client() -> Result<Client, reqwest::Error> {
    let v: Vec<_> = std::env::vars()
        .filter(|k| k.0.contains("x_rapid_api"))
        .collect();

    assert_eq!(v.len(), 2);
    let mut header = HeaderMap::new();

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

    Client::builder().default_headers(header).build()
}
