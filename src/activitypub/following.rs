use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::activitypub::context::Context;

use super::typ::Collection;

pub fn default_context() -> Context {
    Context::from("https://www.w3.org/ns/activitystreams")
}

#[derive(Serialize, Deserialize)]
pub struct Following {
    #[serde(rename = "@context")]
    pub schema_context: Context,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: Collection,
    #[serde(rename = "totalItems")]
    pub total_items: usize,
    pub first: Option<String>,
    pub last: Option<String>,
    pub items: Option<Vec<Value>>,
}
