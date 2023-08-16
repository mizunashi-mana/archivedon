use std::error::Error;

mod input;
mod output;
mod webfinger;
mod activitypub;
mod templates;

use archivedon::webfinger::resource::{Resource as WebfingerResource, Link as WebfingerLink};
use archivedon::activitypub::actor::{Actor, self};
use archivedon::activitypub::collection::Collection as ApCollection;
use archivedon::activitypub::collection as ap_collection;
use output::Output;
use serde_json::json;
use url::Url;

use self::templates::{Templates, ProfileHtmlParams};

pub async fn run(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    let input = input::load(input_path).await?;
    let client = reqwest::Client::new();
    let output_path_buf = tokio::fs::canonicalize(output_path).await?;
    let output = Output::create(output_path_buf).await?;
    let templates = Templates::create()?;

    let static_base_url = Url::parse(&input.static_base_url)?;
    let predef_urls = save_predefs(&output, &static_base_url).await?;

    for account in input.accounts {
        fetch_account(
            &client,
            &static_base_url,
            &output,
            &predef_urls,
            &account,
            &templates,
        ).await?;
    }

    Ok(())
}

pub struct PredefUrls {
    inbox_url: Url,
    empty_collection_url: Url,
    empty_ordered_collection_url: Url,
}

pub async fn fetch_account<'a>(
    client: &reqwest::Client,
    static_base_url: &Url,
    output: &Output,
    predef_urls: &PredefUrls,
    account: &str,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    let account_stripped = account.strip_prefix("@").unwrap_or(&account);
    let (username, domain) = match account_stripped.split_once("@") {
        None => return Err(format!("Illegal account: {account}").into()),
        Some(x) => x,
    };

    let subject = format!("acct:{username}@{domain}");
    let account_actor_url = webfinger::fetch_ap_account_actor_url(client, domain, &subject).await?;
    let account_actor = activitypub::fetch_actor(client, account_actor_url).await?;

    if !account_actor.suspended.is_some_and(|x| x) {
        println!("Warning: account={account} is not suspended.");
    }

    let ap_resource_path = format!("users/{domain}/{username}.json");
    let profile_path = format!("users/{domain}/{username}.html");

    let ap_resource_url = static_base_url.join(&ap_resource_path)?;
    let profile_url = static_base_url.join(&profile_path)?;

    save_webfinger_resource(
        output,
        &ap_resource_url,
        &profile_url,
        subject,
    ).await?;

    save_profile_resource(
        output,
        &profile_path,
        &account_actor,
        templates,
    ).await?;

    save_actor_resource(
        output,
        &ap_resource_path,
        account_actor,
        predef_urls,
        &ap_resource_url,
        &profile_url,
    ).await?;

    Ok(())
}

async fn save_webfinger_resource(
    output: &Output,
    ap_resource_url: &Url,
    profile_url: &Url,
    subject: String,
) -> Result<(), Box<dyn Error>> {
    let new_resource = WebfingerResource {
        subject,
        aliases: Some(vec![
            String::from(ap_resource_url.as_str()),
            String::from(profile_url.as_str()),
        ]),
        properties: None,
        links: Some(vec![
            WebfingerLink {
                rel: String::from("self"),
                typ: Some(String::from("application/activity+json")),
                href: Some(String::from(ap_resource_url.as_str())),
                titles: None,
                properties: None,
            },
            WebfingerLink {
                rel: String::from("http://webfinger.net/rel/profile-page"),
                typ: Some(String::from("text/html")),
                href: Some(String::from(profile_url.as_str())),
                titles: None,
                properties: None,
            },
        ])
    };
    output.save_webfinger_resource(&new_resource).await?;
    Ok(())
}

async fn save_predefs(
    output: &Output,
    static_base_url: &Url,
) -> Result<PredefUrls, Box<dyn Error>> {
    let inbox_path = "predef/inbox.json";
    let empty_collection_path = "predef/empty-collection.json";
    let empty_ordered_collection_path = "predef/empty-ordered-collection.json";
    let predef_urls = PredefUrls {
        inbox_url: static_base_url.join(inbox_path)?,
        empty_collection_url: static_base_url.join(empty_collection_path)?,
        empty_ordered_collection_url: static_base_url.join(empty_ordered_collection_path)?,
    };

    output.save_static_json_resource(inbox_path, &json!({"error": "Not Found"})).await?;

    output.save_static_json_resource(empty_collection_path, &ApCollection {
        schema_context: ap_collection::default_context(),
        id: String::from(predef_urls.empty_collection_url.as_str()),
        typ: String::from("Collection"),
        total_items: 0,
        first: None,
        last: None,
        items: Some(vec![]),
        ordered_items: None,
    }).await?;

    output.save_static_json_resource(empty_ordered_collection_path, &ApCollection {
        schema_context: ap_collection::default_context(),
        id: String::from(predef_urls.empty_ordered_collection_url.as_str()),
        typ: String::from("OrderedCollection"),
        total_items: 0,
        first: None,
        last: None,
        items: None,
        ordered_items: Some(vec![]),
    }).await?;

    Ok(predef_urls)
}

async fn save_profile_resource<'a>(
    output: &Output,
    profile_path: &str,
    original_actor: &Actor,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    output.save_static_text_resource(
        profile_path,
        &templates.render_profile_html(&ProfileHtmlParams {
            name: match &original_actor.name {
                Some(name) => String::from(name),
                None => String::from(""),
            },
            summary: match &original_actor.summary {
                Some(summary) => String::from(summary),
                None => String::from(""),
            },
        })?,
    ).await
}

async fn save_actor_resource(
    output: &Output,
    ap_resource_path: &str,
    original_actor: Actor,
    predef_urls: &PredefUrls,
    ap_resource_url: &Url,
    profile_url: &Url,
) -> Result<(), Box<dyn Error>> {
    output.save_static_json_resource(
        ap_resource_path,
        &Actor {
            schema_context: actor::default_context(),
            id: String::from(ap_resource_url.as_str()),
            typ: original_actor.typ,
            name: original_actor.name,
            summary: original_actor.summary,
            published: original_actor.published,
            preferred_username: original_actor.preferred_username,
            moved_to: original_actor.moved_to,
            also_known_as: original_actor.also_known_as,
            discoverable: original_actor.discoverable,
            manually_approves_followers: original_actor.manually_approves_followers,
            suspended: Some(true),
            url: Some(String::from(profile_url.as_str())),
            inbox: String::from(predef_urls.inbox_url.as_str()),
            outbox: String::from(predef_urls.empty_ordered_collection_url.as_str()),
            followers: String::from(predef_urls.empty_ordered_collection_url.as_str()),
            following: String::from(predef_urls.empty_ordered_collection_url.as_str()),
            featured: Some(String::from(predef_urls.empty_ordered_collection_url.as_str())),
            featured_tags: Some(String::from(predef_urls.empty_collection_url.as_str())),
            devices: Some(String::from(predef_urls.empty_collection_url.as_str())),
            attachment: None,
            image: None,
            icon: None,
            tag: None,
            endpoints: None,
            public_key: None,
        },
    ).await
}
