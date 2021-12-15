use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Format {
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub wratio: Option<i32>,
    pub hratio: Option<i32>,
    pub wmin: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
