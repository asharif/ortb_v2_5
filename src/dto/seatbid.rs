use super::bid::Bid;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct SeatBid {
    pub bid: Vec<Bid>,
    pub seat: Option<String>,
    #[serde(default = "default_group")]
    pub group: u8,
    pub ext: Option<serde_json::Value>,
}

fn default_group() -> u8 {
    0
}
