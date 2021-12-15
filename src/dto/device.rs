use super::geo::Geo;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Device {
    pub ua: Option<String>,
    pub geo: Option<Geo>,
    pub dnt: Option<u8>,
    pub lmt: Option<u8>,
    pub ip: Option<String>,
    pub ipv6: Option<String>,
    pub devicetype: Option<i32>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub os: Option<String>,
    pub osv: Option<String>,
    pub hwv: Option<String>,
    pub h: Option<i32>,
    pub w: Option<i32>,
    pub ppi: Option<i32>,
    pub pxratio: Option<f32>,
    pub js: Option<u8>,
    pub geofetch: Option<u8>,
    pub flashver: Option<String>,
    pub language: Option<String>,
    pub carrier: Option<String>,
    pub mccmnc: Option<String>,
    pub connectiontype: Option<i32>,
    pub ifa: Option<String>,
    pub didsha1: Option<String>,
    pub didmd5: Option<String>,
    pub dpidsha1: Option<String>,
    pub dpidmd5: Option<String>,
    pub macsha1: Option<String>,
    pub macmd5: Option<String>,
    pub ext: Option<serde_json::Value>,
}
