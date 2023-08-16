use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::activitypub::context::Context;

pub fn default_context() -> Context {
    Context::from("https://www.w3.org/ns/activitystreams")
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Collection {
    #[serde(rename = "@context")]
    pub schema_context: Context,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "totalItems")]
    pub total_items: usize,
    pub first: Option<String>,
    pub last: Option<String>,
    pub items: Option<Vec<Value>>,
    pub ordered_items: Option<Vec<Value>>,
}
