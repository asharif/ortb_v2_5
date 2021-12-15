use super::banner::Banner;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Audio {
    pub mimes: Vec<String>,
    pub minduration: Option<i32>,
    pub maxduration: Option<i32>,
    pub protocols: Option<Vec<i32>>,
    pub startdelay: Option<i32>,
    pub sequence: Option<i32>,
    pub battr: Option<Vec<i32>>,
    pub maxextended: Option<i32>,
    pub minbitrate: Option<i32>,
    pub maxbitrate: Option<i32>,
    pub delivery: Option<Vec<i32>>,
    pub companionad: Option<Vec<Banner>>,
    pub api: Option<Vec<i32>>,
    pub companiontype: Option<Vec<i32>>,
    pub maxseq: Option<i32>,
    pub feed: Option<i32>,
    pub stitched: Option<i32>,
    pub nvol: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
