use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root2 {
    pub text: String,
    pub time: Time,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
    pub total: f64,
    pub minutes: i64,
    pub seconds: i64,
    pub hundredths: i64,
}
