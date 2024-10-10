use serde_derive::Deserialize;
use serde_derive::Serialize;

pub type Root = Vec<Root2>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub text: String,
}

pub enum SEARCH {
    SUCCESS(Root),
    FAILURE(String),
}
