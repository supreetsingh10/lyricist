use core::future::Future;
use reqwest::{header::HeaderMap, Client, Error, Response};
use std::collections::HashMap;
pub mod response;

const URL: &str = "https://musixmatch-lyrics-songs.p.rapidapi.com/songs/lyrics?type=json";
pub trait Lyrics {
    type Out;
    fn get_lyrics(&self, query: String) -> impl Future<Output = Self::Out>;
}

impl Lyrics for Client {
    type Out = Result<Response, Error>;
    async fn get_lyrics(&self, query: String) -> Result<Response, Error> {
        let mut query_m: HashMap<&str, &str> = HashMap::new();
        let _ = query.split(',').map(|elems| {
            let split_req: Vec<&str> = elems.split(':').collect();
            assert_eq!(split_req.len(), 2);
            query_m.insert(split_req[0], split_req[1]);
        });

        self.get(URL)
            .query(&[
                ("t", query_m.get("t").unwrap()),
                ("a", query_m.get("a").unwrap()),
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
