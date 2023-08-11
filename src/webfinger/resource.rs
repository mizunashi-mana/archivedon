use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/**
 * ref: https://datatracker.ietf.org/doc/html/rfc7033
 */
#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub subject: String,
    pub aliases: Option<Vec<String>>,
    pub properties: Option<HashMap<String, Option<String>>>,
    pub links: Option<Vec<Link>>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    #[serde(rename = "type")]
    pub typ: Option<String>,
    pub href: Option<String>,
    pub titles: Option<HashMap<String, String>>,
    pub properties: Option<HashMap<String, Option<String>>>,
}
