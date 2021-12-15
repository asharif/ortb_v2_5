use super::banner::Banner;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    pub mimes: Vec<String>,
    pub minduration: Option<i32>,
    pub maxduration: Option<i32>,
    pub protocols: Option<Vec<i32>>,
    //deprecated
    pub protocol: Option<i32>,
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub startdelay: Option<i32>,
    pub placement: Option<i32>,
    pub linearity: Option<i32>,
    pub skip: Option<i32>,
    #[serde(default = "default_skipmin")]
    pub skipmin: i32,
    #[serde(default = "default_skipafter")]
    pub skipafter: i32,
    pub sequence: Option<i32>,
    pub battr: Option<Vec<i32>>,
    pub maxextended: Option<i32>,
    pub minbitrate: Option<i32>,
    pub maxbitrate: Option<i32>,
    #[serde(default = "default_boxingallowed")]
    pub boxingallowed: u8,
    pub playbackend: Option<i32>,
    pub delivery: Option<Vec<i32>>,
    pub pos: Option<i32>,
    pub companionad: Option<Vec<Banner>>,
    pub api: Option<Vec<i32>>,
    pub companiontype: Option<Vec<i32>>,
    pub ext: Option<serde_json::Value>,
}

fn default_skipmin() -> i32 {
    0
}

fn default_skipafter() -> i32 {
    0
}

fn default_boxingallowed() -> u8 {
    1
}
