use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

mod activitypub;
mod env;
mod input;
mod output;
mod templates;
mod webfinger;

use archivedon::activitypub::json::ModelConv;
use archivedon::activitypub::model as ap_model;
use archivedon::redirect_map::RedirectMap;
use archivedon::webfinger::resource::{Link as WebfingerLink, Resource as WebfingerResource};
use chrono::Utc;
use once_cell::sync::Lazy;
use output::Output;
use regex::Regex;
use serde_json::json;
use url::Url;

use self::env::Env;
use self::templates::{ObjectHtmlParams, ProfileHtmlParams, Templates, TopHtmlParams};

pub async fn run(
    input_path: &str,
    output_path: &str,
    fetch_outbox: bool,
    default_max_pages: usize,
    page_items_count: usize,
) -> Result<(), Box<dyn Error>> {
    let input = input::load(input_path).await?;

    let env = Env {
        client: reqwest::Client::new(),
        output: Output::load(Path::new(output_path)).await?,
        templates: Templates::create()?,
        default_max_pages,
        static_base_url: Url::parse(&input.static_base_url)?,
        fetch_outbox,
        page_items_count,
    };

    let predef_urls = save_predefs(&env).await?;

    env.output
        .save_top_page(&env.templates.render_top_html(&TopHtmlParams {
            title: match input.title {
                Some(title) => title,
                None => "Archived ActivityPub Server".to_string(),
            },
            description: match input.description {
                Some(description) => description,
                None => "A hub of archived ActivityPub servers.".to_string(),
            },
        })?)
        .await?;

    for account in input.accounts {
        fetch_account(&env, &predef_urls, &account).await?;
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
        Some((username, domain)) => Account::new(username, domain, &env.static_base_url)?,
    };

    let subject = format!("acct:{}", account.ident);

    let account_actor_url =
        webfinger::fetch_ap_account_actor_url(&env.client, &account.domain, &subject).await?;
    let account_actor = activitypub::fetch_actor(&env.client, account_actor_url).await?;

    if !account_actor
        .mastodon_ext_items
        .suspended
        .is_some_and(|x| x)
        && account_actor.activity_streams_ext_items.moved_to.is_none()
    {
        println!(
            "Warning: account={} is not suspended or moved.",
            account.ident
        );
    }

    save_webfinger_resource(
        &env.output,
        subject,
        &account.actor_url,
        &account.profile_url,
    )
    .await?;

    let outbox_url_opt = if env.fetch_outbox {
        match &account_actor.actor_items {
            None => None,
            Some(actor_items) => Some(
                fetch_outbox_collection_ref(
                    env,
                    &account,
                    &ap_model::ObjectOrLink::Link(ap_model::Link::from(
                        actor_items.outbox.as_str(),
                    )),
                )
                .await?,
            ),
        }
    } else {
        None
    };

    save_profile_resource(&env.output, &account, &account_actor, &env.templates).await?;

    let original_account_link_opt = account_actor.object_items.url.clone();

    save_actor_resource(
        &env.output,
        &account,
        account_actor,
        predef_urls,
        outbox_url_opt,
    )
    .await?;

    if let Some(link) = original_account_link_opt {
        if let Some(old_url) = link.as_full_url() {
            let domain = match old_url.domain() {
                None => panic!("unreachable: The domain of full URL should be available."),
                Some(x) => x,
            };
            save_redirect_map(
                env,
                domain,
                old_url.path(),
                &["application/activity+json".to_string()],
                &account.actor_url,
            )
            .await?;

            let mut rest_media_type = vec!["*/*".to_string()];
            for typ in link.media_type {
                match typ.as_str() {
                    "application/activity+json" => {
                        // do nothing
                    }
                    _ => rest_media_type.push(typ),
                }
            }
            save_redirect_map(
                env,
                domain,
                old_url.path(),
                &rest_media_type,
                &account.profile_url,
            )
            .await?;
        }
    }

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

async fn save_predefs<'a>(env: &Env<'a>) -> Result<PredefUrls, Box<dyn Error>> {
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
                    Some(item) => Some(item.href.to_string()),
                },
                moved_to: original_actor
                    .activity_streams_ext_items
                    .moved_to
                    .to_owned(),
                published: match &original_actor.object_items.published {
                    None => None,
                    Some(item) => Some(item.to_rfc3339()),
                },
            })?,
        )
        .await
}

async fn save_actor_resource(
    output: &Output,
    account: &Account,
    original_actor: ap_model::Object,
    predef_urls: &PredefUrls,
    outbox_url_opt: Option<Url>,
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
            outbox: match outbox_url_opt {
                None => predef_urls.empty_ordered_collection_url.to_string(),
                Some(outbox_url) => outbox_url.to_string(),
            },
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
        security_items: ap_model::SecurityItems {
            public_key: match original_actor.security_items.public_key {
                None => None,
                Some(original_key) => {
                    let mut key_id = account.actor_url.clone();
                    key_id.set_fragment(Some("main-key"));
                    Some(ap_model::Key {
                        // Misskey check the host of the id of the public key is same one of the actor.
                        id: key_id.to_string(),
                        owner: original_key.owner,
                        public_key_pem: original_key.public_key_pem,
                    })
                }
            },
        },
    };

    output
        .save_static_json_resource(&account.actor_path, &actor.from_model()?)
        .await
}

struct NewOutboxCollectionManager {
    // immutable
    page_items_count: usize,

    // mutable
    first_page_url_opt: Option<Url>,
    prev_page_url_opt: Option<Url>,
    head_object_base_path_opt: Option<String>,
    total_items_count: usize,
    items: Vec<ap_model::ObjectOrLink>,
}

struct NewOutboxCollection {
    total_items_count: usize,
    first_page_url_opt: Option<Url>,
    last_page_url_opt: Option<Url>,
}

impl NewOutboxCollectionManager {
    fn new(page_items_count: usize) -> Self {
        assert!(page_items_count > 0);

        Self {
            page_items_count,
            first_page_url_opt: None,
            prev_page_url_opt: None,
            head_object_base_path_opt: None,
            total_items_count: 0,
            items: vec![],
        }
    }

    async fn add_activity_and_save_if_needed<'a>(
        &mut self,
        env: &Env<'a>,
        object_base_path: String,
        item: ap_model::Object,
    ) -> Result<(), Box<dyn Error>> {
        self.items.push(ap_model::ObjectOrLink::Object(item));
        self.total_items_count += 1;
        if self.head_object_base_path_opt.is_none() {
            self.head_object_base_path_opt = Some(object_base_path);
        }

        if self.items.len() < self.page_items_count {
            return Ok(());
        }

        let save_page_path = match &self.head_object_base_path_opt {
            Some(object_base_path) => format!("{}page.json", object_base_path),
            None => panic!(
                "unreachable: head object base path should be available if items are not empty."
            ),
        };
        let page_url = env.static_base_url.join(&save_page_path)?;

        let mut items = vec![];
        items.append(&mut self.items);

        save_outbox_collection_page(
            env,
            &save_page_path,
            &self.first_page_url_opt,
            &self.prev_page_url_opt,
            items,
        )
        .await?;

        self.head_object_base_path_opt = None;
        if self.first_page_url_opt.is_none() {
            self.first_page_url_opt = Some(page_url.clone());
        }
        self.prev_page_url_opt = Some(page_url);

        Ok(())
    }

    async fn save_rest_items<'a>(
        self,
        env: &Env<'a>,
    ) -> Result<NewOutboxCollection, Box<dyn Error>> {
        let save_page_path = match self.head_object_base_path_opt {
            None => {
                return Ok(NewOutboxCollection {
                    total_items_count: self.total_items_count,
                    first_page_url_opt: self.first_page_url_opt,
                    last_page_url_opt: None,
                })
            }
            Some(head_object_base_path) => format!("{}page.json", head_object_base_path),
        };
        let last_page_url = env.static_base_url.join(&save_page_path)?;

        save_outbox_collection_page(
            env,
            &save_page_path,
            &self.first_page_url_opt,
            &self.prev_page_url_opt,
            self.items,
        )
        .await?;

        Ok(NewOutboxCollection {
            total_items_count: self.total_items_count,
            first_page_url_opt: self.first_page_url_opt,
            last_page_url_opt: Some(last_page_url),
        })
    }
}

async fn fetch_outbox_collection_ref<'a>(
    env: &Env<'a>,
    account: &Account,
    collection_ref: &ap_model::ObjectOrLink,
) -> Result<Url, Box<dyn Error>> {
    match collection_ref {
        ap_model::ObjectOrLink::Link(collection_ref) => {
            let collection =
                activitypub::fetch_object(&env.client, collection_ref.href.to_string()).await?;
            fetch_outbox_collection(env, account, &collection).await
        }
        ap_model::ObjectOrLink::Object(collection) => {
            fetch_outbox_collection(env, account, collection).await
        }
    }
}

async fn fetch_outbox_collection<'a>(
    env: &Env<'a>,
    account: &Account,
    collection: &ap_model::Object,
) -> Result<Url, Box<dyn Error>> {
    let mut new_outbox_collection_manager = NewOutboxCollectionManager::new(env.page_items_count);

    for item in &collection.collection_items.items {
        fetch_outbox_activity_ref(env, account, item, &mut new_outbox_collection_manager).await?;
    }

    for item in &collection.ordered_collection_items.ordered_items {
        fetch_outbox_activity_ref(env, account, item, &mut new_outbox_collection_manager).await?;
    }

    if {
        collection.collection_items.total_items != Some(0)
            && collection.collection_items.items.is_empty()
            && collection.ordered_collection_items.ordered_items.is_empty()
    } {
        match &collection.collection_items.first {
            None => {
                // do nothing
            }
            Some(init_collection_page_ref) => {
                fetch_outbox_collection_pages(
                    env,
                    collection
                        .collection_items
                        .total_items
                        .unwrap_or(env.default_max_pages),
                    account,
                    init_collection_page_ref,
                    &mut new_outbox_collection_manager,
                )
                .await?;
            }
        }
    }

    let new_outbox_collection = new_outbox_collection_manager.save_rest_items(env).await?;
    let new_outbox_url = save_outbox_collection(
        env,
        &format!("{}outbox.json", account.base_path),
        new_outbox_collection.total_items_count,
        new_outbox_collection.first_page_url_opt,
        new_outbox_collection.last_page_url_opt,
    )
    .await?;

    Ok(new_outbox_url)
}

async fn fetch_outbox_collection_pages<'a>(
    env: &Env<'a>,
    max_pages_count: usize,
    account: &Account,
    init_collection_page_ref: &ap_model::ObjectOrLink,
    new_outbox_collection_manager: &mut NewOutboxCollectionManager,
) -> Result<(), Box<dyn Error>> {
    let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
        env,
        max_pages_count,
        account,
        init_collection_page_ref,
        new_outbox_collection_manager,
    )
    .await?;

    let mut next_page_opt = result.next_page_opt;
    let mut max_pages_count = max_pages_count - result.fetched_pages_count;
    while let Some(next_page) = next_page_opt {
        let result = fetch_outbox_collection_pages_in_object_and_fetch_next(
            env,
            max_pages_count,
            account,
            &ap_model::ObjectOrLink::Object(next_page),
            new_outbox_collection_manager,
        )
        .await?;

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
    new_outbox_collection_manager: &mut NewOutboxCollectionManager,
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
                });
            }
            ap_model::ObjectOrLink::Object(collection_page) => {
                for item in &collection_page.collection_items.items {
                    fetch_outbox_activity_ref(env, account, item, new_outbox_collection_manager)
                        .await?;
                }

                for item in &collection_page.ordered_collection_items.ordered_items {
                    fetch_outbox_activity_ref(env, account, item, new_outbox_collection_manager)
                        .await?;
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
    new_outbox_collection_manager: &mut NewOutboxCollectionManager,
) -> Result<(), Box<dyn Error>> {
    match activity_ref {
        ap_model::ObjectOrLink::Link(activity_ref) => {
            let uri = activity_ref.href.to_string();
            let activity = activitypub::fetch_object(&env.client, uri).await?;
            fetch_outbox_activity(env, account, &activity, new_outbox_collection_manager).await
        }
        ap_model::ObjectOrLink::Object(activity) => {
            fetch_outbox_activity(env, account, activity, new_outbox_collection_manager).await
        }
    }
}

async fn fetch_outbox_activity<'a>(
    env: &Env<'a>,
    account: &Account,
    activity: &ap_model::Object,
    new_outbox_collection_manager: &mut NewOutboxCollectionManager,
) -> Result<(), Box<dyn Error>> {
    {
        let mut accepted_type = false;
        for typ in &activity.typ {
            match typ.as_str() {
                "Announce" => {
                    // do nothing
                }
                _ => {
                    accepted_type = true;
                    break;
                }
            }
        }
        if !accepted_type {
            return Ok(());
        }
    }

    for object_ref in &activity.activity_items.object {
        let new_object = fetch_outbox_object_ref(env, account, object_ref).await?;

        let save_activity_path = format!("{}activity.json", &new_object.base_path);
        let new_activity = save_outbox_activity(
            env,
            account,
            &save_activity_path,
            activity,
            &new_object.object,
        )
        .await?;

        new_outbox_collection_manager
            .add_activity_and_save_if_needed(env, new_object.base_path, new_activity)
            .await?;
    }

    Ok(())
}

async fn fetch_outbox_object_ref<'a>(
    env: &Env<'a>,
    account: &Account,
    object_ref: &ap_model::ObjectOrLink,
) -> Result<NewObject, Box<dyn Error>> {
    match object_ref {
        ap_model::ObjectOrLink::Link(object_ref) => {
            let uri = object_ref.href.to_string();
            let object = activitypub::fetch_object(&env.client, uri).await?;
            save_outbox_object(env, account, &object).await
        }
        ap_model::ObjectOrLink::Object(object) => save_outbox_object(env, account, object).await,
    }
}

struct NewObject {
    base_path: String,
    object: ap_model::Object,
}

static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r".*/(?<id>\d+)$").unwrap());

async fn save_outbox_object<'a>(
    env: &Env<'a>,
    account: &Account,
    object: &ap_model::Object,
) -> Result<NewObject, Box<dyn Error>> {
    let Some(caps) = (match &object.id {
        None => return Err(format!("Object ID should be available.").into()),
        Some(x) => RE_ID.captures(x),
    }) else {
        return Err(format!(
            "The format of object ID is not supported: id={:?}",
            &object.id
        )
        .into());
    };
    let id = &caps["id"];

    let save_json_path = format!("{}entities/{id}.json", account.base_path);
    let new_object_url = env.static_base_url.join(&save_json_path)?;
    let new_object = ap_model::Object {
        schema_context: Some(ap_model::Context::object_default()),
        id: Some(new_object_url.to_string()),
        typ: object.typ.clone(),
        object_items: ap_model::ObjectItems {
            updated: Some(Utc::now()),
            attachment: object.object_items.attachment.clone(),
            attributed_to: object.object_items.attributed_to.clone(),
            audience: object.object_items.audience.clone(),
            bcc: object.object_items.bcc.clone(),
            bto: object.object_items.bto.clone(),
            cc: object.object_items.cc.clone(),
            context: object.object_items.context.clone(),
            generator: object.object_items.generator.clone(),
            icon: object.object_items.icon.clone(),
            image: object.object_items.image.clone(),
            in_reply_to: object.object_items.in_reply_to.clone(),
            location: object.object_items.location.clone(),
            preview: object.object_items.preview.clone(),
            replies: object.object_items.replies.clone(),
            tag: object.object_items.tag.clone(),
            to: object.object_items.to.clone(),
            url: object.object_items.url.clone(),
            content: object.object_items.content.clone(),
            content_map: object.object_items.content_map.clone(),
            name: object.object_items.name.clone(),
            name_map: object.object_items.name_map.clone(),
            duration: object.object_items.duration.clone(),
            media_type: object.object_items.media_type.clone(),
            end_time: object.object_items.end_time.clone(),
            published: object.object_items.published.clone(),
            summary: object.object_items.summary.clone(),
            summary_map: object.object_items.summary_map.clone(),
            describes: object.object_items.describes.clone(),
        },
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
    };
    env.output
        .save_static_json_resource(&save_json_path, &new_object.clone().from_model()?)
        .await?;

    let save_html_path = format!("{}entities/{id}.html", account.base_path);
    env.output
        .save_static_text_resource(
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
                    },
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
        )
        .await?;

    if let Some(link) = &object.object_items.url {
        if let Some(old_url) = &link.as_full_url() {
            let domain = match old_url.domain() {
                None => panic!("unreachable: The domain of full URL should be available."),
                Some(x) => x,
            };
            save_redirect_map(
                env,
                domain,
                old_url.path(),
                &link.media_type,
                &new_object_url,
            )
            .await?;
        }
    }

    Ok(NewObject {
        base_path: format!("{}entities/{id}/", account.base_path),
        object: new_object,
    })
}

async fn save_outbox_activity<'a>(
    env: &Env<'a>,
    account: &Account,
    new_activity_path: &str,
    original_activity: &ap_model::Object,
    new_object: &ap_model::Object,
) -> Result<ap_model::Object, Box<dyn Error>> {
    let new_activity_url = env.static_base_url.join(new_activity_path)?;
    let new_activity = ap_model::Object {
        schema_context: Some(ap_model::Context::object_default()),
        id: Some(new_activity_url.to_string()),
        typ: original_activity.typ.clone(),
        object_items: ap_model::ObjectItems {
            updated: Some(Utc::now()),
            attachment: original_activity.object_items.attachment.clone(),
            attributed_to: original_activity.object_items.attributed_to.clone(),
            audience: original_activity.object_items.audience.clone(),
            bcc: original_activity.object_items.bcc.clone(),
            bto: original_activity.object_items.bto.clone(),
            cc: original_activity.object_items.cc.clone(),
            context: original_activity.object_items.context.clone(),
            generator: original_activity.object_items.generator.clone(),
            icon: original_activity.object_items.icon.clone(),
            image: original_activity.object_items.image.clone(),
            in_reply_to: original_activity.object_items.in_reply_to.clone(),
            location: original_activity.object_items.location.clone(),
            preview: original_activity.object_items.preview.clone(),
            replies: original_activity.object_items.replies.clone(),
            tag: original_activity.object_items.tag.clone(),
            to: original_activity.object_items.to.clone(),
            url: original_activity.object_items.url.clone(),
            content: original_activity.object_items.content.clone(),
            content_map: original_activity.object_items.content_map.clone(),
            name: original_activity.object_items.name.clone(),
            name_map: original_activity.object_items.name_map.clone(),
            duration: original_activity.object_items.duration.clone(),
            media_type: original_activity.object_items.media_type.clone(),
            end_time: original_activity.object_items.end_time.clone(),
            published: original_activity.object_items.published.clone(),
            summary: original_activity.object_items.summary.clone(),
            summary_map: original_activity.object_items.summary_map.clone(),
            describes: original_activity.object_items.describes.clone(),
        },
        actor_items: original_activity.actor_items.clone(),
        activity_items: ap_model::ActivityItems {
            actor: vec![ap_model::ObjectOrLink::Link(ap_model::Link::from(
                account.actor_url.to_string(),
            ))],
            instrument: vec![],
            origin: match &original_activity.id {
                None => vec![],
                Some(item) => vec![ap_model::ObjectOrLink::Link(ap_model::Link::from(
                    item.to_string(),
                ))],
            },
            object: vec![ap_model::ObjectOrLink::Object(
                new_object.clone_without_schema_context(),
            )],
            result: vec![],
            target: vec![],
        },
        collection_items: original_activity.collection_items.clone(),
        ordered_collection_items: original_activity.ordered_collection_items.clone(),
        collection_page_items: original_activity.collection_page_items.clone(),
        ordered_collection_page_items: original_activity.ordered_collection_page_items.clone(),
        relationship_items: original_activity.relationship_items.clone(),
        tombstone_items: original_activity.tombstone_items.clone(),
        question_items: original_activity.question_items.clone(),
        place_items: original_activity.place_items.clone(),
        activity_streams_ext_items: original_activity.activity_streams_ext_items.clone(),
        mastodon_ext_items: original_activity.mastodon_ext_items.clone(),
        security_items: original_activity.security_items.clone(),
    };

    env.output
        .save_static_json_resource(new_activity_path, &new_activity.clone().from_model()?)
        .await?;

    Ok(new_activity)
}

async fn save_outbox_collection_page<'a>(
    env: &Env<'a>,
    save_page_path: &str,
    first_page_url_opt: &Option<Url>,
    prev_page_url_opt: &Option<Url>,
    items: Vec<ap_model::ObjectOrLink>,
) -> Result<(), Box<dyn Error>> {
    let page_url = env.static_base_url.join(&save_page_path)?;

    env.output
        .save_static_json_resource(
            &save_page_path,
            &ap_model::Object {
                schema_context: Some(ap_model::Context::object_default()),
                id: Some(page_url.to_string()),
                typ: vec!["OrderedCollectionPage".to_string()],
                object_items: ap_model::ObjectItems::empty(),
                actor_items: None,
                activity_items: ap_model::ActivityItems::empty(),
                collection_items: ap_model::CollectionItems {
                    total_items: None,
                    current: None,
                    first: match &first_page_url_opt {
                        None => None,
                        Some(first_page_url) => Some(Box::new(ap_model::ObjectOrLink::Link(
                            ap_model::Link::from(first_page_url.as_str()),
                        ))),
                    },
                    last: None,
                    items: vec![],
                },
                ordered_collection_items: ap_model::OrderedCollectionItems {
                    ordered_items: items,
                },
                collection_page_items: ap_model::CollectionPageItems {
                    next: None,
                    prev: match &prev_page_url_opt {
                        None => None,
                        Some(prev_page_url) => Some(Box::new(ap_model::ObjectOrLink::Link(
                            ap_model::Link::from(prev_page_url.as_str()),
                        ))),
                    },
                    part_of: None,
                },
                ordered_collection_page_items: ap_model::OrderedCollectionPageItems {
                    start_index: None,
                },
                relationship_items: ap_model::RelationshipItems::empty(),
                tombstone_items: ap_model::TombstoneItems::empty(),
                question_items: ap_model::QuestionItems::empty(),
                place_items: ap_model::PlaceItems::empty(),
                activity_streams_ext_items: ap_model::ActivityStreamExtItems::empty(),
                mastodon_ext_items: ap_model::MastodonExtItems::empty(),
                security_items: ap_model::SecurityItems::empty(),
            }
            .from_model()?,
        )
        .await?;

    Ok(())
}

async fn save_outbox_collection<'a>(
    env: &Env<'a>,
    save_path: &str,
    total_items_count: usize,
    first_page_url_opt: Option<Url>,
    last_page_url_opt: Option<Url>,
) -> Result<Url, Box<dyn Error>> {
    let url = env.static_base_url.join(save_path)?;

    env.output
        .save_static_json_resource(
            save_path,
            &ap_model::Object::new_collection(
                Some(url.to_string()),
                vec!["OrderedCollection".to_string()],
                Some(total_items_count),
                None,
                match first_page_url_opt {
                    None => None,
                    Some(first_page_url) => Some(Box::new(ap_model::ObjectOrLink::Link(
                        ap_model::Link::from(first_page_url.to_string()),
                    ))),
                },
                match last_page_url_opt {
                    None => None,
                    Some(last_page_url) => Some(Box::new(ap_model::ObjectOrLink::Link(
                        ap_model::Link::from(last_page_url.to_string()),
                    ))),
                },
                vec![],
                vec![],
            )
            .from_model()?,
        )
        .await?;

    Ok(url)
}

async fn save_redirect_map<'a>(
    env: &Env<'a>,
    domain: &str,
    url_path: &str,
    media_type: &[String],
    new_url: &Url,
) -> Result<(), Box<dyn Error>> {
    match env
        .output
        .get_redirect_map_resource(domain, url_path)
        .await?
    {
        None => {
            let redirect_map = RedirectMap::new();
            save_redirect_map_with_old_resource(
                env,
                domain,
                url_path,
                redirect_map,
                media_type,
                new_url,
            )
            .await
        }
        Some(redirect_map) => {
            save_redirect_map_with_old_resource(
                env,
                domain,
                url_path,
                redirect_map,
                media_type,
                new_url,
            )
            .await
        }
    }
}

async fn save_redirect_map_with_old_resource<'a>(
    env: &Env<'a>,
    domain: &str,
    url_path: &str,
    mut redirect_map: RedirectMap,
    media_type: &[String],
    new_url: &Url,
) -> Result<(), Box<dyn Error>> {
    if media_type.is_empty() {
        redirect_map.insert_entry("*/*".to_string(), new_url);
    }

    for typ in media_type {
        redirect_map.insert_entry(typ.to_string(), new_url);
    }

    env.output
        .save_redirect_map_resource(domain, url_path, &redirect_map)
        .await?;

    Ok(())
}
