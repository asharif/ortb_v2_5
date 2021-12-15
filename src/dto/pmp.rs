use super::deal::Deal;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Pmp {
    #[serde(default = "default_private_auction")]
    pub private_auction: u8,
    pub deals: Option<Vec<Deal>>,
    pub ext: Option<serde_json::Value>,
}

fn default_private_auction() -> u8 {
    0
}
