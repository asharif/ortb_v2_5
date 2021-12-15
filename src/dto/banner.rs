use super::format::Format;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Banner {
    pub format: Option<Vec<Format>>,
    pub w: Option<i32>,
    pub h: Option<i32>,
    //deprecated
    pub wmax: Option<i32>,
    //deprecated
    pub hmax: Option<i32>,
    //deprecated
    pub wmin: Option<i32>,
    //deprecated
    pub hmin: Option<i32>,
    pub btype: Option<Vec<i32>>,
    pub battr: Option<Vec<i32>>,
    pub pos: Option<i32>,
    pub mimes: Option<Vec<String>>,
    pub topframe: Option<u8>,
    pub expdir: Option<Vec<i32>>,
    pub api: Option<Vec<i32>>,
    pub id: Option<String>,
    pub vcm: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
