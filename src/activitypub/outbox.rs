use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::activitypub::context::Context;

use super::typ::OrderedCollection;

pub fn default_context() -> Context {
    Context::from("https://www.w3.org/ns/activitystreams")
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Outbox {
    #[serde(rename = "@context")]
    pub schema_context: Context,
    pub id: String,
    #[serde(rename = "type")]
    pub typ: OrderedCollection,
    #[serde(rename = "totalItems")]
    pub total_items: usize,
    pub first: Option<String>,
    pub last: Option<String>,
    pub items: Option<Vec<Value>>,
}
