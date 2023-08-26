use url::Url;

use super::{output::Output, templates::Templates};

pub struct Env<'a> {
    pub client: reqwest::Client,
    pub output: Output,
    pub templates: Templates<'a>,

    pub default_max_pages: usize,
    pub static_base_url: Url,
    pub fetch_outbox: bool,
    pub page_items_count: usize,
}
