use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Segment {
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub ext: Option<serde_json::Value>,
}
