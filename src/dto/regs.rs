use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Regs {
    pub coppa: Option<u8>,
    pub ext: Option<serde_json::Value>,
}
