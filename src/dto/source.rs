use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    pub fd: Option<u8>,
    pub tid: Option<String>,
    pub pchain: Option<String>,
    pub ext: Option<serde_json::Value>,
}
