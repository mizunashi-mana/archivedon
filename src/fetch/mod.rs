use std::error::Error;

mod activitypub;
mod input;
mod output;
mod templates;
mod webfinger;

use archivedon::activitypub::actor::{self, Actor};
use archivedon::activitypub::collection as ap_collection;
use archivedon::activitypub::collection::Collection as ApCollection;
use archivedon::webfinger::resource::{Link as WebfingerLink, Resource as WebfingerResource};
use output::Output;
use serde_json::json;
use url::Url;

use self::templates::{ProfileHtmlParams, Templates};

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
        )
        .await?;
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

    save_webfinger_resource(output, &ap_resource_url, &profile_url, subject).await?;

    save_profile_resource(
        output,
        format!("{username}@{domain}"),
        &profile_path,
        &ap_resource_url,
        &account_actor,
        &None,
        templates,
    )
    .await?;

    save_actor_resource(
        output,
        &ap_resource_path,
        account_actor,
        predef_urls,
        &ap_resource_url,
        &profile_url,
    )
    .await?;

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
        aliases: Some(vec![ap_resource_url.to_string(), profile_url.to_string()]),
        properties: None,
        links: Some(vec![
            WebfingerLink {
                rel: "self".to_string(),
                typ: Some("application/activity+json".to_string()),
                href: Some(ap_resource_url.to_string()),
                titles: None,
                properties: None,
            },
            WebfingerLink {
                rel: "http://webfinger.net/rel/profile-page".to_string(),
                typ: Some("text/html".to_string()),
                href: Some(profile_url.to_string()),
                titles: None,
                properties: None,
            },
        ]),
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

    output
        .save_static_json_resource(inbox_path, &json!({"error": "Not Found"}))
        .await?;

    output
        .save_static_json_resource(
            empty_collection_path,
            &ApCollection {
                schema_context: ap_collection::default_context(),
                id: predef_urls.empty_collection_url.to_string(),
                typ: "Collection".to_string(),
                total_items: 0,
                first: None,
                last: None,
                items: Some(vec![]),
                ordered_items: None,
            },
        )
        .await?;

    output
        .save_static_json_resource(
            empty_ordered_collection_path,
            &ApCollection {
                schema_context: ap_collection::default_context(),
                id: predef_urls.empty_ordered_collection_url.to_string(),
                typ: "OrderedCollection".to_string(),
                total_items: 0,
                first: None,
                last: None,
                items: None,
                ordered_items: Some(vec![]),
            },
        )
        .await?;

    Ok(predef_urls)
}

async fn save_profile_resource<'a>(
    output: &Output,
    account: String,
    profile_path: &str,
    ap_resource_url: &Url,
    original_actor: &Actor,
    moved_profile_url: &Option<String>,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    output
        .save_static_text_resource(
            profile_path,
            &templates.render_profile_html(&ProfileHtmlParams {
                account,
                actor_url: ap_resource_url.to_string(),
                name: original_actor.name.to_owned(),
                summary: original_actor.summary.to_owned(),
                url: original_actor.url.to_owned(),
                moved_to: original_actor.moved_to.to_owned(),
                moved_profile_url: moved_profile_url.to_owned(),
            })?,
        )
        .await
}

async fn save_actor_resource(
    output: &Output,
    ap_resource_path: &str,
    original_actor: Actor,
    predef_urls: &PredefUrls,
    ap_resource_url: &Url,
    profile_url: &Url,
) -> Result<(), Box<dyn Error>> {
    output
        .save_static_json_resource(
            ap_resource_path,
            &Actor {
                schema_context: actor::default_context(),
                id: ap_resource_url.to_string(),
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
                url: Some(profile_url.to_string()),
                inbox: predef_urls.inbox_url.to_string(),
                outbox: predef_urls.empty_ordered_collection_url.to_string(),
                followers: predef_urls.empty_ordered_collection_url.to_string(),
                following: predef_urls.empty_ordered_collection_url.to_string(),
                featured: Some(predef_urls.empty_ordered_collection_url.to_string()),
                featured_tags: Some(predef_urls.empty_collection_url.to_string()),
                devices: Some(predef_urls.empty_collection_url.to_string()),
                attachment: original_actor.attachment,
                // TODO: fetch images
                image: original_actor.image,
                // TODO: fetch icons
                icon: original_actor.icon,
                tag: original_actor.tag,
                endpoints: None,
                public_key: original_actor.public_key,
            },
        )
        .await
}
