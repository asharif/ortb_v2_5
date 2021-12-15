use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Native {
    pub request: String,
    pub ver: Option<String>,
    pub api: Option<Vec<i32>>,
    pub battr: Option<Vec<i32>>,
    pub ext: Option<serde_json::Value>,
}
