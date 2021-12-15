use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Metric {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: f32,
    pub vendor: Option<String>,
    pub ext: Option<serde_json::Value>,
}
