use std::error::Error;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

pub const TEMPLATE_PROFILE_HTML_KEY: &str = "PROFILE_HTML";

#[derive(Serialize, Deserialize)]
pub struct ProfileHtmlParams {
    pub typ: Option<String>,
    pub account: String,
    pub actor_url: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub url: Option<String>,
    pub moved_to: Option<String>,
    pub moved_profile_url: Option<String>,
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
            "<title>Archived - {{account}}</title>",
            "<link href=\"{{actor_url}}\" rel=\"alternate\" type=\"application/activity+json\">",
            "<meta content=\"{{account}}\" property=\"profile:username\">",
            "</head>",
            "<body>",
            "<h1>Archived {{typ}}: {{account}}</h1>",
            "<dl>",
            "{{#if moved_to}}{{#if moved_profile_url}}",
            "<dt>Moved To</dt><dd><a href=\"{{moved_profile_url}}\">{{moved_to}}</a></dd>",
            "{{else}}",
            "<dt>Moved To</dt><dd>{{moved_to}}</dd>",
            "{{/if}}{{/if}}",
            "{{#if name}}<dt>Name</dt><dd>{{name}}</dd>{{/if}}",
            "{{#if summary}}<dt>Summary</dt><dd>{{{summary}}}</dd>{{/if}}",
            "{{#if url}}<dt>URL</dt><dd><a href=\"{{url}}\">{{url}}</a></dd>{{/if}}",
            "</dl>",
            "</body>",
            "</html>",
        ].join(""))?;

        Ok(Templates { handlebars })
    }

    pub fn render_profile_html(
        &self,
        params: &ProfileHtmlParams,
    ) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars.render(TEMPLATE_PROFILE_HTML_KEY, params)?)
    }
}
