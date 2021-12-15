use super::data::Data;
use super::geo::Geo;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<String>,
    pub buyeruid: Option<String>,
    pub yob: Option<i32>,
    pub gender: Option<String>,
    pub keywords: Option<String>,
    pub customdata: Option<String>,
    pub geo: Option<Geo>,
    pub data: Option<Vec<Data>>,
    pub ext: Option<serde_json::Value>,
}
