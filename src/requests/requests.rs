use reqwest::{header::HeaderMap, Client};

#[allow(dead_code)]
pub fn generate_header() -> HeaderMap {
    let v: Vec<_> = std::env::vars()
        .into_iter()
        .filter(|k| k.0.find("genius").is_some())
        .collect();

    let mut header = HeaderMap::new();

    for (key, vals) in v.iter() {
        if key.eq("genius_api_key") {
            assert!(header
                .insert("x-rapidapi-key", vals.parse().unwrap())
                .is_none());
        } else {
            assert!(header
                .insert("x-rapidapi-host", vals.parse().unwrap())
                .is_none());
        }
    }

    header
}
