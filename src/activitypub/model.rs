use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::Value;

/**
 * Schema: https://www.w3.org/TR/json-ld/#the-context
 */
#[derive(PartialEq, Debug)]
pub enum Context {
    Single(Iri),
    Mix(Vec<Context>),
    TermDefs(HashMap<String, Iri>),
}

impl Context {
    pub fn pure_ap() -> Context {
        Context::from("https://www.w3.org/ns/activitystreams")
    }

    pub fn object_default() -> Context {
        Context::Mix(vec![
            Context::from("https://www.w3.org/ns/activitystreams"),
            Context::from("https://w3id.org/security/v1"),
            Context::from([
                (
                    "alsoKnownAs",
                    Iri::TypeCoercion {
                        id: "as:alsoKnownAs".to_string(),
                        typ: Some("@id".to_string()),
                    },
                ),
                (
                    "manuallyApprovesFollowers",
                    Iri::from("as:manuallyApprovesFollowers"),
                ),
                (
                    "movedTo",
                    Iri::TypeCoercion {
                        id: "as:movedTo".to_string(),
                        typ: Some("@id".to_string()),
                    },
                ),
            ]),
            Context::from([
                ("toot", Iri::from("http://joinmastodon.org/ns#")),
                (
                    "devices",
                    Iri::TypeCoercion {
                        id: "toot:devices".to_string(),
                        typ: Some("@id".to_string()),
                    },
                ),
                ("discoverable", Iri::from("toot:discoverable")),
                (
                    "featured",
                    Iri::TypeCoercion {
                        id: "toot:featured".to_string(),
                        typ: Some("@id".to_string()),
                    },
                ),
                (
                    "featuredTags",
                    Iri::TypeCoercion {
                        id: "toot:featuredTags".to_string(),
                        typ: Some("@id".to_string()),
                    },
                ),
                ("suspended", Iri::from("toot:suspended")),
            ]),
        ])
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Iri {
    Direct(String),
    TypeCoercion { id: String, typ: Option<String> },
}

/**
 * Reference: https://www.w3.org/ns/activitystreams
 */
#[derive(PartialEq, Debug)]
pub struct Object {
    pub schema_context: Option<Context>,
    pub id: Option<String>,
    pub typ: Vec<String>,

    pub object_items: ObjectItems,
    pub actor_items: Option<ActorItems>,
    pub activity_items: ActivityItems,
    pub collection_items: CollectionItems,
    pub ordered_collection_items: OrderedCollectionItems,
    pub collection_page_items: CollectionPageItems,
    pub ordered_collection_page_items: OrderedCollectionPageItems,
    pub relationship_items: RelationshipItems,
    pub tombstone_items: TombstoneItems,
    pub question_items: QuestionItems,
    pub place_items: PlaceItems,
    pub activity_streams_ext_items: ActivityStreamExtItems,
    pub mastodon_ext_items: MastodonExtItems,
    pub security_items: SecurityItems,
}

impl Object {
    pub fn new_collection(
        id: Option<String>,
        typ: Vec<String>,
        total_items: Option<usize>,
        current: Option<Box<ObjectOrLink>>,
        first: Option<Box<ObjectOrLink>>,
        last: Option<Box<ObjectOrLink>>,
        items: Vec<ObjectOrLink>,
        ordered_items: Vec<ObjectOrLink>,
    ) -> Self {
        Self {
            schema_context: Some(Context::pure_ap()),
            id,
            typ,
            object_items: ObjectItems::empty(),
            actor_items: None,
            activity_items: ActivityItems::empty(),
            collection_items: CollectionItems {
                total_items,
                current,
                first,
                last,
                items,
            },
            ordered_collection_items: OrderedCollectionItems { ordered_items },
            collection_page_items: CollectionPageItems::empty(),
            ordered_collection_page_items: OrderedCollectionPageItems::empty(),
            relationship_items: RelationshipItems::empty(),
            tombstone_items: TombstoneItems::empty(),
            question_items: QuestionItems::empty(),
            place_items: PlaceItems::empty(),
            activity_streams_ext_items: ActivityStreamExtItems::empty(),
            mastodon_ext_items: MastodonExtItems::empty(),
            security_items: SecurityItems::empty(),
        }
    }
}

/**
 * Reference: https://www.w3.org/TR/activitystreams-vocabulary/#dfn-link
 */
#[derive(PartialEq, Debug)]
pub struct Link {
    pub schema_context: Option<Context>,
    pub id: Option<String>,
    pub typ: Vec<String>,

    pub href: String,
    pub height: Option<usize>,
    pub hreflang: Option<String>,
    pub media_type: Vec<String>,
    pub rel: Vec<String>,
    pub width: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub enum ObjectOrLink {
    Link(Link),
    Object(Object),
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Object
 */
#[derive(PartialEq, Debug)]
pub struct ObjectItems {
    pub attachment: Vec<ObjectOrLink>,
    pub attributed_to: Vec<ObjectOrLink>,
    pub audience: Vec<ObjectOrLink>,
    pub bcc: Vec<ObjectOrLink>,
    pub bto: Vec<ObjectOrLink>,
    pub cc: Vec<ObjectOrLink>,
    pub context: Vec<ObjectOrLink>,
    pub generator: Vec<ObjectOrLink>,
    // Range: Image | Link
    pub icon: Vec<ObjectOrLink>,
    // Range: Image | Link
    pub image: Vec<ObjectOrLink>,
    pub in_reply_to: Vec<ObjectOrLink>,
    pub location: Vec<ObjectOrLink>,
    pub preview: Vec<ObjectOrLink>,
    // Range: Collection
    pub replies: Option<Box<Object>>,
    pub tag: Vec<ObjectOrLink>,
    pub to: Vec<ObjectOrLink>,
    pub url: Option<Link>,
    pub content: Vec<String>,
    pub content_map: HashMap<String, String>,
    pub name: Vec<String>,
    pub name_map: HashMap<String, String>,
    // TODO: more strict
    pub duration: Option<String>,
    pub media_type: Vec<String>,
    pub end_time: Option<DateTime<Utc>>,
    pub published: Option<DateTime<Utc>>,
    pub summary: Vec<String>,
    pub summary_map: HashMap<String, String>,
    pub updated: Option<DateTime<Utc>>,
    pub describes: Option<Box<Object>>,
}

impl ObjectItems {
    pub fn empty() -> Self {
        Self {
            attachment: vec![],
            attributed_to: vec![],
            audience: vec![],
            bcc: vec![],
            bto: vec![],
            cc: vec![],
            context: vec![],
            generator: vec![],
            icon: vec![],
            image: vec![],
            in_reply_to: vec![],
            location: vec![],
            preview: vec![],
            replies: None,
            tag: vec![],
            to: vec![],
            url: None,
            content: vec![],
            content_map: HashMap::new(),
            name: vec![],
            name_map: HashMap::new(),
            duration: None,
            media_type: vec![],
            end_time: None,
            published: None,
            summary: vec![],
            summary_map: HashMap::new(),
            updated: None,
            describes: None,
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Actor
 */
#[derive(PartialEq, Debug)]
pub struct ActorItems {
    pub inbox: String,
    pub outbox: String,
    pub following: String,
    pub followers: String,
    pub preferred_username: Option<String>,
    pub endpoints: HashMap<String, String>,
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Activity
 */
#[derive(PartialEq, Debug)]
pub struct ActivityItems {
    pub actor: Vec<ObjectOrLink>,
    pub instrument: Vec<ObjectOrLink>,
    pub origin: Vec<ObjectOrLink>,
    pub object: Vec<ObjectOrLink>,
    pub result: Vec<ObjectOrLink>,
    pub target: Vec<ObjectOrLink>,
}

impl ActivityItems {
    pub fn empty() -> Self {
        Self {
            actor: vec![],
            instrument: vec![],
            origin: vec![],
            object: vec![],
            result: vec![],
            target: vec![],
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Collection
 */
#[derive(PartialEq, Debug)]
pub struct CollectionItems {
    pub total_items: Option<usize>,
    // Range: CollectionPage | Link
    pub current: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    pub first: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    pub last: Option<Box<ObjectOrLink>>,
    pub items: Vec<ObjectOrLink>,
}

impl CollectionItems {
    pub fn empty() -> Self {
        Self {
            total_items: None,
            current: None,
            first: None,
            last: None,
            items: vec![],
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#OrderedCollection
 */
#[derive(PartialEq, Debug)]
pub struct OrderedCollectionItems {
    pub ordered_items: Vec<ObjectOrLink>,
}

impl OrderedCollectionItems {
    pub fn empty() -> Self {
        Self {
            ordered_items: vec![],
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#CollectionPage
 */
#[derive(PartialEq, Debug)]
pub struct CollectionPageItems {
    pub next: Option<Box<ObjectOrLink>>,
    pub prev: Option<Box<ObjectOrLink>>,
    // Range: Link | Collection
    pub part_of: Option<Box<ObjectOrLink>>,
}

impl CollectionPageItems {
    pub fn empty() -> Self {
        Self {
            next: None,
            prev: None,
            part_of: None,
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#OrderedCollectionPage
 */
#[derive(PartialEq, Debug)]
pub struct OrderedCollectionPageItems {
    pub start_index: Option<usize>,
}

impl OrderedCollectionPageItems {
    pub fn empty() -> Self {
        Self { start_index: None }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Relationship
 */
#[derive(PartialEq, Debug)]
pub struct RelationshipItems {
    pub subject: Option<Box<ObjectOrLink>>,
    pub relationship: Vec<Object>,
}

impl RelationshipItems {
    pub fn empty() -> Self {
        Self {
            subject: None,
            relationship: vec![],
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Tombstone
 */
#[derive(PartialEq, Debug)]
pub struct TombstoneItems {
    pub former_type: Vec<Object>,
    pub deleted: Option<DateTime<Utc>>,
}

impl TombstoneItems {
    pub fn empty() -> Self {
        Self {
            former_type: vec![],
            deleted: None,
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Question
 */
#[derive(PartialEq, Debug)]
pub struct QuestionItems {
    pub one_of: Vec<ObjectOrLink>,
    pub any_of: Vec<ObjectOrLink>,
    // TODO: more strict
    pub closed: Option<Value>,
}

impl QuestionItems {
    pub fn empty() -> Self {
        Self {
            one_of: vec![],
            any_of: vec![],
            closed: None,
        }
    }
}

/**
 * Reference: https://www.w3.org/ns/activitystreams#Place
 */
#[derive(PartialEq, Debug)]
pub struct PlaceItems {
    pub accuracy: Option<f64>,
    pub altitude: Option<f64>,
    pub latitute: Option<f64>,
    pub longitute: Option<f64>,
    pub radius: Option<f64>,
    pub units: Option<String>,
}

impl PlaceItems {
    pub fn empty() -> Self {
        Self {
            accuracy: None,
            altitude: None,
            latitute: None,
            longitute: None,
            radius: None,
            units: None,
        }
    }
}

/**
 * Reference: https://docs.joinmastodon.org/spec/activitypub/#as
 */
#[derive(PartialEq, Eq, Debug)]
pub struct ActivityStreamExtItems {
    pub manually_approves_followers: Option<bool>,
    pub also_known_as: Vec<String>,
    pub moved_to: Option<String>,
}

impl ActivityStreamExtItems {
    pub fn empty() -> Self {
        Self {
            manually_approves_followers: None,
            also_known_as: vec![],
            moved_to: None,
        }
    }
}

/**
 * Reference: https://docs.joinmastodon.org/spec/activitypub/#toot
 */
#[derive(PartialEq, Eq, Debug)]
pub struct MastodonExtItems {
    // http://joinmastodon.org/ns#featured
    pub featured: Option<String>,

    // http://joinmastodon.org/ns#featuredTags
    pub featured_tags: Option<String>,

    // http://joinmastodon.org/ns#discoverable
    pub discoverable: Option<bool>,

    // http://joinmastodon.org/ns#suspended
    pub suspended: Option<bool>,

    // http://joinmastodon.org/ns#devices
    pub devices: Option<String>,
}

impl MastodonExtItems {
    pub fn empty() -> Self {
        Self {
            featured: None,
            featured_tags: None,
            discoverable: None,
            suspended: None,
            devices: None,
        }
    }
}

/**
 * Reference: https://w3id.org/security/v1
 */
#[derive(PartialEq, Debug)]
pub struct SecurityItems {
    pub public_key: Option<Key>,
}

impl SecurityItems {
    pub fn empty() -> Self {
        Self { public_key: None }
    }
}

/**
 * Reference: https://w3c.github.io/vc-data-integrity/vocab/security/vocabulary.html#Key
 */
#[derive(PartialEq, Eq, Debug)]
pub struct Key {
    pub id: String,
    pub owner: String,
    pub public_key_pem: Option<String>,
}

impl From<&str> for Context {
    fn from(value: &str) -> Self {
        Self::Single(Iri::from(value))
    }
}

impl<const N: usize> From<[(&str, Iri); N]> for Context {
    fn from(value: [(&str, Iri); N]) -> Self {
        let defs = HashMap::from_iter(
            value
                .into_iter()
                .map(|entry| (entry.0.to_string(), entry.1)),
        );
        Self::TermDefs(defs)
    }
}

impl From<&str> for Iri {
    fn from(value: &str) -> Self {
        Self::Direct(value.to_string())
    }
}

impl From<&str> for Link {
    fn from(value: &str) -> Self {
        Link::from(value.to_string())
    }
}

impl From<String> for Link {
    fn from(value: String) -> Self {
        Self {
            href: value,
            schema_context: None,
            id: None,
            typ: vec![],
            height: None,
            hreflang: None,
            media_type: vec![],
            rel: vec![],
            width: None,
        }
    }
}
