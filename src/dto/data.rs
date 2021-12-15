use super::segment::Segment;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub id: Option<String>,
    pub name: Option<String>,
    pub segment: Option<Vec<Segment>>,
    pub ext: Option<serde_json::Value>,
}
