use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct RedirectMap {
    type_to_url: HashMap<String, String>,
}

impl RedirectMap {
    pub fn new() -> RedirectMap {
        RedirectMap {
            type_to_url: HashMap::new(),
        }
    }

    pub fn insert_entry(&mut self, typ: String, new_url: &Url) -> () {
        let _: Option<String> = self.type_to_url.insert(typ, new_url.to_string());
    }

    pub fn get_entry(&self, typ: &str) -> Option<Url> {
        match self.type_to_url.get(typ) {
            None => None,
            Some(url) => match Url::parse(url) {
                Ok(url) => Some(url),
                Err(_) => None,
            },
        }
    }
}
