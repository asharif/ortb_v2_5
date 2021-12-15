use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Geo {
    pub lat: Option<f32>,
    pub lon: Option<f32>,
    #[serde(rename = "type")]
    pub _type: Option<f32>,
    pub accuracy: Option<i32>,
    pub lastfix: Option<i32>,
    pub ipservice: Option<i32>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub regionfips104: Option<String>,
    pub metro: Option<String>,
    pub city: Option<String>,
    pub zip: Option<String>,
    pub utcoffset: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
