use std::error::Error;

use handlebars::Handlebars;
use serde::{Serialize, Deserialize};

pub const TEMPLATE_PROFILE_HTML_KEY: &str = "PROFILE_HTML";

#[derive(Serialize, Deserialize)]
pub struct ProfileHtmlParams {
    pub name: String,
    pub summary: String,
}

pub struct Templates<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> Templates<'a> {
    pub fn create() -> Result<Self, Box<dyn Error>> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string(TEMPLATE_PROFILE_HTML_KEY, &[
            "<!DOCTYPE html>",
            "<html>",
            "<head>",
            "<meta charset=\"utf-8\">",
            "<meta content=\"width=device-width, initial-scale=1\" name=\"viewport\">",
            "<title>Archivedon - {{name}}</title>",
            "</head>",
            "<body>",
            "<h1>{{name}}</h1>",
            "<div>{{{summary}}}</div>",
            "</body>",
            "</html>",
        ].join(""))?;

        Ok(Templates {
            handlebars,
        })
    }

    pub fn render_profile_html(&self, params: &ProfileHtmlParams) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars.render(TEMPLATE_PROFILE_HTML_KEY, params)?)
    }
}
