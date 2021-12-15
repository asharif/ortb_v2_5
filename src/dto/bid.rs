use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Bid {
    pub id: String,
    pub impid: String,
    //will probably need to use a crate for this because of ieee floating point spec
    pub price: f64,
    pub nurl: Option<String>,
    pub burl: Option<String>,
    pub lurl: Option<String>,
    pub adm: Option<String>,
    pub adid: Option<String>,
    pub adomain: Option<Vec<String>>,
    pub bundle: Option<String>,
    pub iurl: Option<String>,
    pub cid: Option<String>,
    pub crid: Option<String>,
    pub tactic: Option<String>,
    pub cat: Option<Vec<String>>,
    pub attr: Option<Vec<i32>>,
    pub api: Option<i32>,
    pub protocol: Option<i32>,
    pub qagmediarating: Option<i32>,
    pub language: Option<String>,
    pub dealid: Option<String>,
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub wratio: Option<i32>,
    pub hratio: Option<i32>,
    pub exp: Option<i32>,
    pub ext: Option<serde_json::Value>,
}
