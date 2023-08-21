use std::collections::HashMap;
use std::error::Error;

mod activitypub;
mod input;
mod output;
mod templates;
mod webfinger;
mod env;

use archivedon::activitypub::json::ModelConv;
use archivedon::activitypub::model as ap_model;
use archivedon::webfinger::resource::{Link as WebfingerLink, Resource as WebfingerResource};
use chrono::Utc;
use output::Output;
use serde_json::json;
use url::Url;
use regex::Regex;

use self::env::Env;
use self::templates::{ProfileHtmlParams, Templates};

pub async fn run(input_path: &str, output_path: &str, default_max_pages: usize) -> Result<(), Box<dyn Error>> {
    let input = input::load(input_path).await?;
    let client = reqwest::Client::new();
    let output_path_buf = tokio::fs::canonicalize(output_path).await?;
    let output = Output::create(output_path_buf).await?;
    let templates = Templates::create()?;

    let env = Env {
        default_max_pages,
        static_base_url: Url::parse(&input.static_base_url)?,
    };

    let predef_urls = save_predefs(&output, &env.static_base_url).await?;

    for account in input.accounts {
        fetch_account(
            &client,
            &env,
            &output,
            &predef_urls,
            &account,
            &templates,
        )
        .await?;
    }

    Ok(())
}

struct PredefUrls {
    inbox_url: Url,
    empty_collection_url: Url,
    empty_ordered_collection_url: Url,
}

#[derive(Debug)]
struct Account {
    username: String,
    domain: String,
}

async fn fetch_account<'a>(
    client: &reqwest::Client,
    env: &Env,
    output: &Output,
    predef_urls: &PredefUrls,
    account: &str,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    let account_stripped = account.strip_prefix("@").unwrap_or(&account);
    let account = match account_stripped.split_once("@") {
        None => return Err(format!("Illegal account: {account}").into()),
        Some((username, domain)) => Account {
            username: username.to_string(),
            domain: domain.to_string(),
        },
    };

    let account_ident = format!("{}@{}", &account.username, &account.domain);
    let subject = format!("acct:{}", account_ident);

    let account_actor_url = webfinger::fetch_ap_account_actor_url(client, &account.domain, &subject).await?;
    let account_actor = activitypub::fetch_actor(client, account_actor_url).await?;

    if !account_actor
        .mastodon_ext_items
        .suspended
        .is_some_and(|x| x)
    {
        println!("Warning: account={account:?} is not suspended.");
    }

    let ap_resource_path = format!("users/{}/{}.json", account.domain, account.username);
    let profile_path = format!("users/{}/{}.html", account.domain, account.username);
    let user_resource_path_base = format!("users/{}/{}/", account.domain, account.username);

    let ap_resource_url = env.static_base_url.join(&ap_resource_path)?;
    let profile_url = env.static_base_url.join(&profile_path)?;

    save_webfinger_resource(output, &ap_resource_url, &profile_url, subject).await?;

    for actor_items in &account_actor.actor_items {
        fetch_outbox_collection_ref(
            env,
            client,
            output,
            &user_resource_path_base,
            &ap_model::ObjectOrLink::Link(ap_model::Link::from(actor_items.outbox.as_str())),
        ).await?;
    }

    save_profile_resource(
        output,
        account_ident,
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
            &ap_model::Object::new_collection(
                Some(predef_urls.empty_collection_url.to_string()),
                vec!["Collection".to_string()],
                Some(0),
                None,
                None,
                None,
                vec![],
                vec![],
            )
            .from_model()?,
        )
        .await?;

    output
        .save_static_json_resource(
            empty_ordered_collection_path,
            &ap_model::Object::new_collection(
                Some(predef_urls.empty_ordered_collection_url.to_string()),
                vec!["OrderedCollection".to_string()],
                Some(0),
                None,
                None,
                None,
                vec![],
                vec![],
            )
            .from_model()?,
        )
        .await?;

    Ok(predef_urls)
}

async fn save_profile_resource<'a>(
    output: &Output,
    account: String,
    profile_path: &str,
    ap_resource_url: &Url,
    original_actor: &ap_model::Object,
    moved_profile_url: &Option<String>,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    output
        .save_static_text_resource(
            profile_path,
            &templates.render_profile_html(&ProfileHtmlParams {
                typ: original_actor.typ.first().cloned(),
                account,
                actor_url: ap_resource_url.to_string(),
                name: original_actor.object_items.name.first().cloned(),
                summary: original_actor.object_items.summary.first().cloned(),
                url: match &original_actor.object_items.url {
                    None => None,
                    Some(item) => Some(item.href.to_string())
                },
                moved_to: original_actor
                    .activity_streams_ext_items
                    .moved_to
                    .to_owned(),
                moved_profile_url: moved_profile_url.to_owned(),
            })?,
        )
        .await
}

async fn save_actor_resource(
    output: &Output,
    ap_resource_path: &str,
    mut actor: ap_model::Object,
    predef_urls: &PredefUrls,
    ap_resource_url: &Url,
    profile_url: &Url,
) -> Result<(), Box<dyn Error>> {
    actor.id = Some(ap_resource_url.to_string());
    actor.object_items.url = Some(ap_model::Link {
        href: profile_url.to_string(),
        schema_context: None,
        id: None,
        typ: vec![],
        height: None,
        hreflang: None,
        media_type: vec![],
        rel: vec![],
        width: None,
    });
    actor.object_items.updated = Some(Utc::now());
    actor.mastodon_ext_items.suspended = Some(true);
    actor.mastodon_ext_items.featured = Some(predef_urls.empty_ordered_collection_url.to_string());
    actor.mastodon_ext_items.featured_tags = Some(predef_urls.empty_collection_url.to_string());
    actor.mastodon_ext_items.devices = Some(predef_urls.empty_collection_url.to_string());

    match &mut actor.actor_items {
        Some(actor_items) => {
            actor_items.inbox = predef_urls.inbox_url.to_string();
            actor_items.outbox = predef_urls.empty_ordered_collection_url.to_string();
            actor_items.following = predef_urls.empty_ordered_collection_url.to_string();
            actor_items.followers = predef_urls.empty_ordered_collection_url.to_string();
            actor_items.endpoints = HashMap::new();
        }
        None => return Err(format!("unreachable: actor_items should be available.").into()),
    };

    output
        .save_static_json_resource(ap_resource_path, &actor.from_model()?)
        .await
}

async fn fetch_outbox_collection_ref(
    env: &Env,
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    collection_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match collection_ref {
        ap_model::ObjectOrLink::Link(collection_ref) => {
            let collection = activitypub::fetch_object(client, collection_ref.href.to_string()).await?;
            fetch_outbox_collection(
                env,
                client,
                output,
                user_resource_path_base,
                &collection,
            ).await
        }
        ap_model::ObjectOrLink::Object(collection) => {
            fetch_outbox_collection(
                env,
                client,
                output,
                user_resource_path_base,
                collection,
            ).await
        }
    }
}

async fn fetch_outbox_collection(
    env: &Env,
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    collection: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    for item in &collection.collection_items.items {
        fetch_outbox_activity_ref(
            client,
            output,
            user_resource_path_base,
            item,
        ).await?;
    }

    for item in &collection.ordered_collection_items.ordered_items {
        fetch_outbox_activity_ref(
            client,
            output,
            user_resource_path_base,
            item,
        ).await?;
    }

    if {
        !collection.collection_items.items.is_empty() ||
        !collection.ordered_collection_items.ordered_items.is_empty() ||
        collection.collection_items.total_items == Some(0)
    } {
        return Ok(())
    }

    match &collection.collection_items.first {
        None => Ok(()),
        Some(init_collection_page_ref) => fetch_outbox_collection_pages(
            client,
            output,
            collection.collection_items.total_items.unwrap_or(env.default_max_pages),
            user_resource_path_base,
            init_collection_page_ref,
        ).await
    }
}

async fn fetch_outbox_collection_pages(
    client: &reqwest::Client,
    output: &Output,
    max_pages_count: usize,
    user_resource_path_base: &str,
    init_collection_page_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
        max_pages_count,
        client,
        output,
        user_resource_path_base,
        init_collection_page_ref,
    ).await?;

    let mut next_page_opt = result.next_page_opt;
    let mut max_pages_count = max_pages_count - result.fetched_pages_count;
    while let Some(next_page) = next_page_opt {
        let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
            max_pages_count,
            client,
            output,
            user_resource_path_base,
            &ap_model::ObjectOrLink::Object(next_page),
        ).await?;

        next_page_opt = result.next_page_opt;
        max_pages_count -= result.fetched_pages_count;
    }

    Ok(())
}

struct FetchNextCollectionPageResult {
    next_page_opt: Option<ap_model::Object>,
    fetched_pages_count: usize,
}
async fn fetch_outbox_collection_pages_in_object_and_fetch_next(
    max_pages_count: usize,
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    collection_page_ref: &ap_model::ObjectOrLink,
) -> Result<FetchNextCollectionPageResult, Box<dyn Error>> {
    let mut collection_page_ref = collection_page_ref;
    let mut fetched_pages_count: usize = 0;
    loop {
        if fetched_pages_count >= max_pages_count {
            return Ok(FetchNextCollectionPageResult {
                next_page_opt: None,
                fetched_pages_count,
            });
        }

        match collection_page_ref {
            ap_model::ObjectOrLink::Link(collection_page_ref) => {
                let next_page_uri = collection_page_ref.href.to_string();
                let next_page = activitypub::fetch_object(client, next_page_uri).await?;
                return Ok(FetchNextCollectionPageResult {
                    next_page_opt: Some(next_page),
                    fetched_pages_count: fetched_pages_count + 1,
                })
            }
            ap_model::ObjectOrLink::Object(collection_page) => {
                for item in &collection_page.collection_items.items {
                    fetch_outbox_activity_ref(
                        client,
                        output,
                        user_resource_path_base,
                        item,
                    ).await?;
                }

                for item in &collection_page.ordered_collection_items.ordered_items {
                    fetch_outbox_activity_ref(
                        client,
                        output,
                        user_resource_path_base,
                        item,
                    ).await?;
                }

                match &collection_page.collection_page_items.next {
                    None => {
                        return Ok(FetchNextCollectionPageResult {
                            next_page_opt: None,
                            fetched_pages_count,
                        });
                    }
                    Some(next_collection_page_ref) => {
                        collection_page_ref = next_collection_page_ref;
                        fetched_pages_count += 1;
                    }
                }
            }
        }
    }
}

async fn fetch_outbox_activity_ref(
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    activity_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match activity_ref {
        ap_model::ObjectOrLink::Link(activity_ref) => {
            let uri = activity_ref.href.to_string();
            let activity = activitypub::fetch_object(client, uri).await?;
            fetch_outbox_activity(
                client,
                output,
                user_resource_path_base,
                &activity,
            ).await
        }
        ap_model::ObjectOrLink::Object(activity) => {
            fetch_outbox_activity(
                client,
                output,
                user_resource_path_base,
                activity,
            ).await
        }
    }
}

async fn fetch_outbox_activity(
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    activity: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    for object_ref in &activity.activity_items.object {
        fetch_outbox_object_ref(
            client,
            output,
            user_resource_path_base,
            object_ref,
        ).await?;
    }
    Ok(())
}

async fn fetch_outbox_object_ref(
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    object_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match object_ref {
        ap_model::ObjectOrLink::Link(object_ref) => {
            let uri = object_ref.href.to_string();
            let object = activitypub::fetch_object(client, uri).await?;
            fetch_outbox_object(
                client,
                output,
                user_resource_path_base,
                &object,
            ).await
        }
        ap_model::ObjectOrLink::Object(object) => {
            fetch_outbox_object(
                client,
                output,
                user_resource_path_base,
                object,
            ).await
        }
    }
}

async fn fetch_outbox_object(
    client: &reqwest::Client,
    output: &Output,
    user_resource_path_base: &str,
    object: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    let id_re = Regex::new(r".*/(?<id>\d+)$").unwrap();
    let Some(caps) = (match &object.id {
        None => return Err(format!("Object ID should be available.").into()),
        Some(x) => id_re.captures(x),
    }) else {
        return Err(format!("The format of object ID is not supported.").into())
    };
    let id = &caps["id"];

    todo!();

    Ok(())
}
