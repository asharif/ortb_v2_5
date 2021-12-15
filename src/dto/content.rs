use super::data::Data;
use super::producer::Producer;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Content {
    pub id: Option<String>,
    pub episode: Option<i32>,
    pub title: Option<String>,
    pub series: Option<String>,
    pub season: Option<String>,
    pub artist: Option<String>,
    pub genre: Option<String>,
    pub album: Option<String>,
    pub isrc: Option<String>,
    pub producer: Option<Producer>,
    pub url: Option<String>,
    pub cat: Option<Vec<String>>,
    pub prodq: Option<i32>,
    //deprecated
    pub videoquality: Option<i32>,
    pub context: Option<i32>,
    pub contentrating: Option<String>,
    pub userrating: Option<String>,
    pub qagmediarating: Option<i32>,
    pub keywords: Option<String>,
    pub livestream: Option<u8>,
    pub sourcerelationship: Option<u8>,
    pub len: Option<i32>,
    pub language: Option<String>,
    pub embeddable: Option<u8>,
    pub data: Option<Data>,
    pub ext: Option<serde_json::Value>,
}
