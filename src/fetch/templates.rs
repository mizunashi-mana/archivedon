use std::{error::Error, collections::HashMap};

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

pub const TEMPLATE_KEY_PROFILE_HTML: &str = "PROFILE_HTML";
pub const TEMPLATE_KEY_OBJECT_HTML: &str = "OBJECT_HTML";

#[derive(Serialize, Deserialize)]
pub struct ProfileHtmlParams {
    pub typ: Option<String>,
    pub account: String,
    pub actor_url: String,
    pub name: Option<String>,
    pub name_map: HashMap<String, String>,
    pub summary: Option<String>,
    pub summary_map: HashMap<String, String>,
    pub url: Option<String>,
    pub moved_to: Option<String>,
    pub moved_profile_url: Option<String>,
    pub published: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectHtmlParams {
    pub typ: Option<String>,
    pub account: String,
    pub account_url: String,
    pub object_url: String,
    pub to: Option<String>,
    pub content: Option<String>,
    pub content_map: HashMap<String, String>,
    pub url: Option<String>,
    pub published: Option<String>,
}

pub struct Templates<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> Templates<'a> {
    pub fn create() -> Result<Self, Box<dyn Error>> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string(TEMPLATE_KEY_PROFILE_HTML, &[
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
            "{{#each name_map}}<dt>Name ({{@key}})</dt><dd>{{this}}</dd>{{/each}}",
            "{{#if summary}}<dt>Summary</dt><dd>{{{summary}}}</dd>{{/if}}",
            "{{#each summary_map}}<dt>Summary ({{@key}})</dt><dd>{{this}}</dd>{{/each}}",
            "{{#if url}}<dt>URL</dt><dd><a href=\"{{url}}\">{{url}}</a></dd>{{/if}}",
            "{{#if published}}<dt>Published</dt><dd>{{published}}</dd>{{/if}}",
            "</dl>",
            "</body>",
            "</html>",
        ].join(""))?;

        handlebars.register_template_string(TEMPLATE_KEY_OBJECT_HTML, &[
            "<!DOCTYPE html>",
            "<html>",
            "<head>",
            "<meta charset=\"utf-8\">",
            "<meta content=\"width=device-width, initial-scale=1\" name=\"viewport\">",
            "<title>Archived - {{url}}</title>",
            "<link href=\"{{object_url}}\" rel=\"alternate\" type=\"application/activity+json\">",
            "<meta content=\"{{account}}\" property=\"profile:username\">",
            "</head>",
            "<body>",
            "<h1>Archived {{typ}}</h1>",
            "<dl>",
            "{{#if content}}<dt>Content</dt><dd>{{{content}}}</dd>{{/if}}",
            "{{#each content_map}}<dt>Content ({{@key}})</dt><dd>{{{this}}}</dd>{{/each}}",
            "{{#if from}}<dt>From</dt><dd><a href=\"{{account_url}}\">{{account}}</a></dd>{{/if}}",
            "{{#if to}}<dt>To</dt><dd><a href=\"{{to}}\">{{to}}</a></dd>{{/if}}",
            "{{#if url}}<dt>URL</dt><dd><a href=\"{{url}}\">{{url}}</a></dd>{{/if}}",
            "{{#if published}}<dt>Published</dt><dd>{{published}}</dd>{{/if}}",
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
        Ok(self.handlebars.render(TEMPLATE_KEY_PROFILE_HTML, params)?)
    }

    pub fn render_object_html(
        &self,
        params: &ObjectHtmlParams,
    ) -> Result<String, Box<dyn Error>> {
        Ok(self.handlebars.render(TEMPLATE_KEY_OBJECT_HTML, params)?)
    }
}
