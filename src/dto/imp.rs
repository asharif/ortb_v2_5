use super::audio::Audio;
use super::banner::Banner;
use super::common_defaults::default_curr;
use super::metric::Metric;
use super::native::Native;
use super::pmp::Pmp;
use super::video::Video;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Imp {
    pub id: String,
    pub metric: Option<Vec<Metric>>,
    pub banner: Option<Banner>,
    pub video: Option<Video>,
    pub audio: Option<Audio>,
    pub native: Option<Native>,
    pub pmp: Option<Pmp>,
    pub displaymanager: Option<String>,
    pub displaymanagerver: Option<String>,
    #[serde(default = "default_instl")]
    pub instl: u8,
    pub tagid: Option<String>,
    #[serde(default = "default_bidfloor")]
    pub bidfloor: f32,
    #[serde(default = "default_curr")]
    pub bidfloorcur: String,
    pub clickbrowser: Option<u8>,
    pub secure: Option<u8>,
    pub iframebuster: Option<Vec<String>>,
    pub exp: Option<u32>,
    pub ext: Option<serde_json::Value>,
}

fn default_instl() -> u8 {
    0
}

fn default_bidfloor() -> f32 {
    0.0
}
