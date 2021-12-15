use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Producer {
    pub id: Option<String>,
    pub name: Option<String>,
    pub cat: Option<Vec<String>>,
    pub domain: Option<String>,
    pub ext: Option<serde_json::Value>,
}
