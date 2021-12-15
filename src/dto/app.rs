use super::content::Content;
use super::publisher::Publisher;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct App {
    pub id: Option<String>,
    pub name: Option<String>,
    pub bundle: Option<String>,
    pub domain: Option<String>,
    pub storeurl: Option<String>,
    pub cat: Option<Vec<String>>,
    pub sectioncat: Option<Vec<String>>,
    pub pagecat: Option<Vec<String>>,
    pub ver: Option<String>,
    pub privacypolicy: Option<u8>,
    pub paid: Option<u8>,
    pub publisher: Option<Publisher>,
    pub content: Option<Content>,
    pub keywords: Option<String>,
    pub ext: Option<serde_json::Value>,
}
