use core::future::Future;
use reqwest::{header::HeaderMap, Client, Error, Response};
use std::collections::HashMap;

pub trait Lyrics {
    type Out;
    fn get_lyrics(&self, query: &HashMap<&str, &str>) -> impl Future<Output = Self::Out>;
}

impl Lyrics for Client {
    type Out = Result<Response, Error>;
    // query will be t="Saint And Sinner" a="Saint And Sinners"
    async fn get_lyrics(&self, query: &HashMap<&str, &str>) -> Result<Response, Error> {
        self.get("https://musixmatch-lyrics-songs.p.rapidapi.com/songs/lyrics?type=json")
            .query(&[
                ("t", query.get("t").unwrap()),
                ("a", query.get("a").unwrap()),
            ])
            .send()
            .await
    }
}

pub fn generate_client() -> Result<Client, reqwest::Error> {
    let v: Vec<_> = std::env::vars()
        .into_iter()
        .filter(|k| k.0.find("x_rapid_api").is_some())
        .collect();

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

    assert_ne!(header.len(), 0);
    Client::builder().default_headers(header).build()
}
