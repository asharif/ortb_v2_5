use super::common_defaults::default_curr;
use super::seatbid::SeatBid;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BidResponse {
    pub id: String,
    pub seatbid: Option<Vec<SeatBid>>,
    pub bidid: Option<String>,
    #[serde(default = "default_curr")]
    pub curr: String,
    pub customdata: Option<String>,
    pub nbr: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
