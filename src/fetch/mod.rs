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
use self::templates::{ProfileHtmlParams, Templates, ObjectHtmlParams};

pub async fn run(input_path: &str, output_path: &str, default_max_pages: usize) -> Result<(), Box<dyn Error>> {
    let input = input::load(input_path).await?;
    let output_path_buf = tokio::fs::canonicalize(output_path).await?;

    let env = Env {
        client: reqwest::Client::new(),
        output: Output::create(output_path_buf).await?,
        templates: Templates::create()?,
        default_max_pages,
        static_base_url: Url::parse(&input.static_base_url)?,
    };

    let predef_urls = save_predefs(&env).await?;

    for account in input.accounts {
        fetch_account(
            &env,
            &predef_urls,
            &account,
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
    domain: String,
    ident: String,

    actor_path: String,
    profile_path: String,
    base_path: String,

    actor_url: Url,
    profile_url: Url,
}

impl Account {
    fn new(username: &str, domain: &str, static_base_url: &Url) -> Result<Account, Box<dyn Error>> {
        let actor_path = format!("users/{}/{}.json", domain, username);
        let profile_path = format!("users/{}/{}.html", domain, username);
        let base_path = format!("users/{}/{}/", domain, username);

        Ok(Account {
            domain: domain.to_string(),
            ident: format!("{}@{}", username, domain),
            actor_path: actor_path.to_string(),
            profile_path: profile_path.to_string(),
            base_path,
            actor_url: static_base_url.join(&actor_path)?,
            profile_url: static_base_url.join(&profile_path)?,
        })
    }
}

async fn fetch_account<'a>(
    env: &Env<'a>,
    predef_urls: &PredefUrls,
    account: &str,
) -> Result<(), Box<dyn Error>> {
    let account_stripped = account.strip_prefix("@").unwrap_or(&account);
    let account = match account_stripped.split_once("@") {
        None => return Err(format!("Illegal account: {account}").into()),
        Some((username, domain)) => Account::new(
            username,
            domain,
            &env.static_base_url,
        )?,
    };

    let subject = format!("acct:{}", account.ident);

    let account_actor_url = webfinger::fetch_ap_account_actor_url(
        &env.client,
        &account.domain,
        &subject,
    ).await?;
    let account_actor = activitypub::fetch_actor(&env.client, account_actor_url).await?;

    if !account_actor
        .mastodon_ext_items
        .suspended
        .is_some_and(|x| x)
    {
        println!("Warning: account={} is not suspended.", account.ident);
    }

    save_webfinger_resource(
        &env.output,
        subject,
        &account.actor_url,
        &account.profile_url,
    ).await?;

    for actor_items in &account_actor.actor_items {
        fetch_outbox_collection_ref(
            env,
            &account,
            &ap_model::ObjectOrLink::Link(ap_model::Link::from(actor_items.outbox.as_str())),
        ).await?;
    }

    save_profile_resource(
        &env.output,
        &account,
        &account_actor,
        &None,
        &env.templates,
    )
    .await?;

    save_actor_resource(
        &env.output,
        &account,
        account_actor,
        predef_urls,
    )
    .await?;

    Ok(())
}

async fn save_webfinger_resource(
    output: &Output,
    subject: String,
    ap_resource_url: &Url,
    profile_url: &Url,
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

async fn save_predefs<'a>(
    env: &Env<'a>,
) -> Result<PredefUrls, Box<dyn Error>> {
    let inbox_path = "predef/inbox.json";
    let empty_collection_path = "predef/empty-collection.json";
    let empty_ordered_collection_path = "predef/empty-ordered-collection.json";
    let predef_urls = PredefUrls {
        inbox_url: env.static_base_url.join(inbox_path)?,
        empty_collection_url: env.static_base_url.join(empty_collection_path)?,
        empty_ordered_collection_url: env.static_base_url.join(empty_ordered_collection_path)?,
    };

    env.output
        .save_static_json_resource(inbox_path, &json!({"error": "Not Found"}))
        .await?;

    env.output
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

    env.output
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
    account: &Account,
    original_actor: &ap_model::Object,
    moved_profile_url: &Option<String>,
    templates: &Templates<'a>,
) -> Result<(), Box<dyn Error>> {
    output
        .save_static_text_resource(
            &account.profile_path,
            &templates.render_profile_html(&ProfileHtmlParams {
                typ: original_actor.typ.first().cloned(),
                account: account.ident.to_string(),
                actor_url: account.actor_url.to_string(),
                name: original_actor.object_items.name.first().cloned(),
                name_map: original_actor.object_items.name_map.clone(),
                summary: original_actor.object_items.summary.first().cloned(),
                summary_map: original_actor.object_items.summary_map.clone(),
                url: match &original_actor.object_items.url {
                    None => None,
                    Some(item) => Some(item.href.to_string())
                },
                moved_to: original_actor
                    .activity_streams_ext_items
                    .moved_to
                    .to_owned(),
                moved_profile_url: moved_profile_url.to_owned(),
                published: match &original_actor.object_items.published {
                    None => None,
                    Some(item) => Some(item.to_rfc3339()),
                }
            })?,
        )
        .await
}

async fn save_actor_resource(
    output: &Output,
    account: &Account,
    original_actor: ap_model::Object,
    predef_urls: &PredefUrls,
) -> Result<(), Box<dyn Error>> {
    let original_actor_items = match original_actor.actor_items {
        None => panic!("unreachable: actor_items should be available."),
        Some(x) => x,
    };

    let actor = ap_model::Object {
        schema_context: Some(ap_model::Context::object_default()),
        id: Some(account.actor_url.to_string()),
        typ: original_actor.typ,
        object_items: ap_model::ObjectItems {
            url: original_actor.object_items.url,
            updated: Some(Utc::now()),
            attachment: original_actor.object_items.attachment,
            attributed_to: original_actor.object_items.attributed_to,
            audience: original_actor.object_items.audience,
            bcc: original_actor.object_items.bcc,
            bto: original_actor.object_items.bto,
            cc: original_actor.object_items.cc,
            context: original_actor.object_items.context,
            generator: original_actor.object_items.generator,
            icon: original_actor.object_items.icon,
            image: original_actor.object_items.image,
            in_reply_to: original_actor.object_items.in_reply_to,
            location: original_actor.object_items.location,
            preview: original_actor.object_items.preview,
            replies: original_actor.object_items.replies,
            tag: original_actor.object_items.tag,
            to: original_actor.object_items.to,
            content: original_actor.object_items.content,
            content_map: original_actor.object_items.content_map,
            name: original_actor.object_items.name,
            name_map: original_actor.object_items.name_map,
            duration: original_actor.object_items.duration,
            media_type: original_actor.object_items.media_type,
            end_time: original_actor.object_items.end_time,
            published: original_actor.object_items.published,
            summary: original_actor.object_items.summary,
            summary_map: original_actor.object_items.summary_map,
            describes: original_actor.object_items.describes,
        },
        actor_items: Some(ap_model::ActorItems {
            inbox: predef_urls.inbox_url.to_string(),
            outbox: predef_urls.empty_ordered_collection_url.to_string(),
            following: predef_urls.empty_ordered_collection_url.to_string(),
            followers: predef_urls.empty_ordered_collection_url.to_string(),
            preferred_username: original_actor_items.preferred_username,
            endpoints: HashMap::new(),
        }),
        activity_items: original_actor.activity_items,
        collection_items: original_actor.collection_items,
        ordered_collection_items: original_actor.ordered_collection_items,
        collection_page_items: original_actor.collection_page_items,
        ordered_collection_page_items: original_actor.ordered_collection_page_items,
        relationship_items: original_actor.relationship_items,
        tombstone_items: original_actor.tombstone_items,
        question_items: original_actor.question_items,
        place_items: original_actor.place_items,
        activity_streams_ext_items: original_actor.activity_streams_ext_items,
        mastodon_ext_items: ap_model::MastodonExtItems {
            featured: Some(predef_urls.empty_ordered_collection_url.to_string()),
            featured_tags: Some(predef_urls.empty_collection_url.to_string()),
            discoverable: original_actor.mastodon_ext_items.discoverable,
            suspended: Some(true),
            devices: Some(predef_urls.empty_collection_url.to_string()),
        },
        security_items: original_actor.security_items,
    };

    output
        .save_static_json_resource(&account.actor_path, &actor.from_model()?)
        .await
}

async fn fetch_outbox_collection_ref<'a>(
    env: &Env<'a>,
    account: &Account,
    collection_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match collection_ref {
        ap_model::ObjectOrLink::Link(collection_ref) => {
            let collection = activitypub::fetch_object(&env.client, collection_ref.href.to_string()).await?;
            fetch_outbox_collection(
                env,
                account,
                &collection,
            ).await
        }
        ap_model::ObjectOrLink::Object(collection) => {
            fetch_outbox_collection(
                env,
                account,
                collection,
            ).await
        }
    }
}

async fn fetch_outbox_collection<'a>(
    env: &Env<'a>,
    account: &Account,
    collection: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    for item in &collection.collection_items.items {
        fetch_outbox_activity_ref(
            env,
            account,
            item,
        ).await?;
    }

    for item in &collection.ordered_collection_items.ordered_items {
        fetch_outbox_activity_ref(
            env,
            account,
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
            env,
            collection.collection_items.total_items.unwrap_or(env.default_max_pages),
            account,
            init_collection_page_ref,
        ).await
    }
}

async fn fetch_outbox_collection_pages<'a>(
    env: &Env<'a>,
    max_pages_count: usize,
    account: &Account,
    init_collection_page_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
        env,
        max_pages_count,
        account,
        init_collection_page_ref,
    ).await?;

    let mut next_page_opt = result.next_page_opt;
    let mut max_pages_count = max_pages_count - result.fetched_pages_count;
    while let Some(next_page) = next_page_opt {
        let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
            env,
            max_pages_count,
            account,
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
async fn fetch_outbox_collection_pages_in_object_and_fetch_next<'a>(
    env: &Env<'a>,
    max_pages_count: usize,
    account: &Account,
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
                let next_page = activitypub::fetch_object(&env.client, next_page_uri).await?;
                return Ok(FetchNextCollectionPageResult {
                    next_page_opt: Some(next_page),
                    fetched_pages_count: fetched_pages_count + 1,
                })
            }
            ap_model::ObjectOrLink::Object(collection_page) => {
                for item in &collection_page.collection_items.items {
                    fetch_outbox_activity_ref(
                        env,
                        account,
                        item,
                    ).await?;
                }

                for item in &collection_page.ordered_collection_items.ordered_items {
                    fetch_outbox_activity_ref(
                        env,
                        account,
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

async fn fetch_outbox_activity_ref<'a>(
    env: &Env<'a>,
    account: &Account,
    activity_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match activity_ref {
        ap_model::ObjectOrLink::Link(activity_ref) => {
            let uri = activity_ref.href.to_string();
            let activity = activitypub::fetch_object(&env.client, uri).await?;
            fetch_outbox_activity(
                env,
                account,
                &activity,
            ).await
        }
        ap_model::ObjectOrLink::Object(activity) => {
            fetch_outbox_activity(
                env,
                account,
                activity,
            ).await
        }
    }
}

async fn fetch_outbox_activity<'a>(
    env: &Env<'a>,
    account: &Account,
    activity: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    {
        let mut accepted_type_count = 0;
        for typ in &activity.typ {
            match typ.as_str() {
                "Announce" => {
                    // do nothing
                }
                _ => {
                    accepted_type_count += 1;
                }
            }
        }
        if accepted_type_count == 0 {
            return Ok(());
        }
    }

    for object_ref in &activity.activity_items.object {
        fetch_outbox_object_ref(
            env,
            account,
            object_ref,
        ).await?;
    }
    Ok(())
}

async fn fetch_outbox_object_ref<'a>(
    env: &Env<'a>,
    account: &Account,
    object_ref: &ap_model::ObjectOrLink,
) -> Result<(), Box<dyn Error>> {
    match object_ref {
        ap_model::ObjectOrLink::Link(object_ref) => {
            let uri = object_ref.href.to_string();
            let object = activitypub::fetch_object(&env.client, uri).await?;
            save_outbox_object(
                env,
                account,
                &object,
            ).await
        }
        ap_model::ObjectOrLink::Object(object) => {
            save_outbox_object(
                env,
                account,
                object,
            ).await
        }
    }
}

async fn save_outbox_object<'a>(
    env: &Env<'a>,
    account: &Account,
    object: &ap_model::Object,
) -> Result<(), Box<dyn Error>> {
    let id_re = Regex::new(r".*/(?<id>\d+)$").unwrap();
    let Some(caps) = (match &object.id {
        None => return Err(format!("Object ID should be available.").into()),
        Some(x) => id_re.captures(x),
    }) else {
        return Err(format!("The format of object ID is not supported: id={:?}", &object.id).into())
    };
    let id = &caps["id"];

    let save_json_path = format!("{}entities/{id}.json", account.base_path);
    let new_object_url = env.static_base_url.join(&save_json_path)?;
    env.output.save_static_json_resource(
        &save_json_path,
        &ap_model::Object {
            schema_context: Some(ap_model::Context::object_default()),
            id: Some(new_object_url.to_string()),
            typ: object.typ.clone(),
            object_items: object.object_items.clone(),
            actor_items: object.actor_items.clone(),
            activity_items: object.activity_items.clone(),
            collection_items: object.collection_items.clone(),
            ordered_collection_items: object.ordered_collection_items.clone(),
            collection_page_items: object.collection_page_items.clone(),
            ordered_collection_page_items: object.ordered_collection_page_items.clone(),
            relationship_items: object.relationship_items.clone(),
            tombstone_items: object.tombstone_items.clone(),
            question_items: object.question_items.clone(),
            place_items: object.place_items.clone(),
            activity_streams_ext_items: object.activity_streams_ext_items.clone(),
            mastodon_ext_items: object.mastodon_ext_items.clone(),
            security_items: object.security_items.clone(),
        }.from_model()?,
    ).await?;

    let save_html_path = format!("{}entities/{id}.html", account.base_path);
    env.output.save_static_text_resource(
        &save_html_path,
        &env.templates.render_object_html(&ObjectHtmlParams {
            typ: object.typ.first().cloned(),
            account: account.ident.to_string(),
            account_url: account.profile_url.to_string(),
            object_url: new_object_url.to_string(),
            to: match object.object_items.to.first() {
                None => None,
                Some(item) => match item {
                    ap_model::ObjectOrLink::Link(item) => Some(item.href.to_string()),
                    ap_model::ObjectOrLink::Object(item) => item.id.clone(),
                }
            },
            content: object.object_items.content.first().cloned(),
            content_map: object.object_items.content_map.clone(),
            url: match &object.object_items.url {
                None => None,
                Some(item) => Some(item.href.to_string()),
            },
            published: match object.object_items.published {
                None => None,
                Some(item) => Some(item.to_rfc3339()),
            },
        })?,
    ).await?;

    Ok(())
}
