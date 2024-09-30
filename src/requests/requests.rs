use reqwest::{header::HeaderMap, Client};

#[allow(dead_code)]
pub fn generate_client() -> Result<Client, reqwest::Error> {
    let v: Vec<_> = std::env::vars()
        .into_iter()
        .filter(|k| k.0.find("x_rapid_api").is_some())
        .collect();

    let mut header = HeaderMap::new();

    let client_builder = Client::builder();

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
    client_builder.default_headers(header).build()
}
