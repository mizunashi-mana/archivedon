use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/**
 * ref: https://datatracker.ietf.org/doc/html/rfc7033
 */
#[derive(Serialize, Deserialize)]
pub struct Resource {
    subject: String,
    aliases: Vec<String>,
    properties: HashMap<String, Option<String>>,
    links: Option<Vec<Link>>,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    rel: String,
    #[serde(rename = "type")]
    typ: Option<String>,
    href: Option<String>,
    titles: Option<HashMap<String, String>>,
    properties: Option<HashMap<String, Option<String>>>,
}
