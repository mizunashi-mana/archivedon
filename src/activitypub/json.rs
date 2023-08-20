use std::{collections::HashMap, error::Error};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::model;

pub trait ModelConv
where
    Self: Sized,
{
    type JsonSerdeValue: Serialize + DeserializeOwned;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>>;
    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>>;
}

impl ModelConv for model::Object {
    type JsonSerdeValue = Object;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        let (inbox, outbox, followers, following, preferred_username, endpoints) =
            match self.actor_items {
                Some(item) => (
                    Some(item.inbox),
                    Some(item.outbox),
                    Some(item.followers),
                    Some(item.following),
                    item.preferred_username,
                    if item.endpoints.is_empty() {
                        None
                    } else {
                        Some(item.endpoints)
                    },
                ),
                None => (None, None, None, None, None, None),
            };

        Ok(Object {
            schema_context: match self.schema_context {
                None => None,
                Some(origin) => Some(origin.from_model()?),
            },
            id: self.id,
            typ: to_lax_array(self.typ)?,
            attachment: to_lax_array(self.object_items.attachment)?,
            attributed_to: to_lax_array(self.object_items.attributed_to)?,
            audience: to_lax_array(self.object_items.audience)?,
            bcc: to_lax_array(self.object_items.bcc)?,
            bto: to_lax_array(self.object_items.bto)?,
            cc: to_lax_array(self.object_items.cc)?,
            context: to_lax_array(self.object_items.context)?,
            generator: to_lax_array(self.object_items.generator)?,
            icon: to_lax_array(self.object_items.icon)?,
            image: to_lax_array(self.object_items.image)?,
            in_reply_to: to_lax_array(self.object_items.in_reply_to)?,
            location: to_lax_array(self.object_items.location)?,
            preview: to_lax_array(self.object_items.preview)?,
            replies: match self.object_items.replies {
                None => None,
                Some(origin) => Some(Box::new(origin.from_model()?)),
            },
            tag: to_lax_array(self.object_items.tag)?,
            to: to_lax_array(self.object_items.to)?,
            url: match self.object_items.url {
                None => None,
                Some(item) => {
                    if {
                        item.height.is_none()
                            && item.hreflang.is_none()
                            && item.id.is_none()
                            && item.media_type.is_empty()
                            && item.rel.is_empty()
                            && item.typ.is_empty()
                            && item.width.is_none()
                    } {
                        Some(Value::String(item.href))
                    } else {
                        Some(serde_json::to_value(item.from_model()?)?)
                    }
                }
            },
            content: to_lax_array(self.object_items.content)?,
            content_map: if self.object_items.content_map.is_empty() {
                None
            } else {
                Some(self.object_items.content_map)
            },
            name: to_lax_array(self.object_items.name)?,
            name_map: if self.object_items.name_map.is_empty() {
                None
            } else {
                Some(self.object_items.name_map)
            },
            duration: self.object_items.duration,
            media_type: to_lax_array(self.object_items.media_type)?,
            end_time: self.object_items.end_time,
            published: self.object_items.published,
            summary: to_lax_array(self.object_items.summary)?,
            summary_map: if self.object_items.summary_map.is_empty() {
                None
            } else {
                Some(self.object_items.summary_map)
            },
            updated: self.object_items.updated,
            describes: match self.object_items.describes {
                None => None,
                Some(origin) => Some(Box::new(origin.from_model()?)),
            },
            inbox,
            outbox,
            followers,
            following,
            preferred_username,
            endpoints,
            actor: to_lax_array(self.activity_items.actor)?,
            instrument: to_lax_array(self.activity_items.instrument)?,
            origin: to_lax_array(self.activity_items.origin)?,
            object: to_lax_array(self.activity_items.object)?,
            result: to_lax_array(self.activity_items.result)?,
            target: to_lax_array(self.activity_items.target)?,
            total_items: self.collection_items.total_items,
            current: match self.collection_items.current {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            first: match self.collection_items.first {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            last: match self.collection_items.last {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            items: to_lax_array(self.collection_items.items)?,
            ordered_items: to_lax_array(self.ordered_collection_items.ordered_items)?,
            next: match self.collection_page_items.next {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            prev: match self.collection_page_items.prev {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            part_of: match self.collection_page_items.part_of {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            start_index: self.ordered_collection_page_items.start_index,
            subject: match self.relationship_items.subject {
                None => None,
                Some(item) => Some(Box::new(item.from_model()?)),
            },
            relationship: to_lax_array(self.relationship_items.relationship)?,
            former_type: to_lax_array(self.tombstone_items.former_type)?,
            deleted: self.tombstone_items.deleted,
            one_of: to_lax_array(self.question_items.one_of)?,
            any_of: to_lax_array(self.question_items.any_of)?,
            closed: self.question_items.closed,
            accuracy: self.place_items.accuracy,
            altitude: self.place_items.altitude,
            latitute: self.place_items.latitute,
            longitute: self.place_items.longitute,
            radius: self.place_items.radius,
            units: self.place_items.units,
            manually_approves_followers: self
                .activity_streams_ext_items
                .manually_approves_followers,
            also_known_as: to_lax_array(self.activity_streams_ext_items.also_known_as)?,
            moved_to: self.activity_streams_ext_items.moved_to,
            featured: self.mastodon_ext_items.featured,
            featured_tags: self.mastodon_ext_items.featured_tags,
            discoverable: self.mastodon_ext_items.discoverable,
            suspended: self.mastodon_ext_items.suspended,
            devices: self.mastodon_ext_items.devices,
            public_key: match self.security_items.public_key {
                None => None,
                Some(item) => Some(item.from_model()?),
            },
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            schema_context: match origin.schema_context {
                None => None,
                Some(item) => Some(model::Context::to_model(item)?),
            },
            id: origin.id,
            typ: from_lax_array(origin.typ)?,
            object_items: model::ObjectItems {
                attachment: from_lax_array(origin.attachment)?,
                attributed_to: from_lax_array(origin.attributed_to)?,
                audience: from_lax_array(origin.audience)?,
                bcc: from_lax_array(origin.bcc)?,
                bto: from_lax_array(origin.bto)?,
                cc: from_lax_array(origin.cc)?,
                context: from_lax_array(origin.context)?,
                generator: from_lax_array(origin.generator)?,
                icon: from_lax_array(origin.icon)?,
                image: from_lax_array(origin.image)?,
                in_reply_to: from_lax_array(origin.in_reply_to)?,
                location: from_lax_array(origin.location)?,
                preview: from_lax_array(origin.preview)?,
                replies: match origin.replies {
                    None => None,
                    Some(item) => Some(Box::new(model::Object::to_model(*item)?)),
                },
                tag: from_lax_array(origin.tag)?,
                to: from_lax_array(origin.to)?,
                url: match origin.url {
                    None => None,
                    Some(Value::String(item)) => Some(model::Link {
                        href: item,
                        schema_context: None,
                        id: None,
                        typ: vec![],
                        height: None,
                        hreflang: None,
                        media_type: vec![],
                        rel: vec![],
                        width: None,
                    }),
                    Some(item) => {
                        let item: Link = serde_json::from_value(item)?;
                        Some(model::Link::to_model(item)?)
                    }
                },
                content: from_lax_array(origin.content)?,
                content_map: match origin.content_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                name: from_lax_array(origin.name)?,
                name_map: match origin.name_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                duration: origin.duration,
                media_type: from_lax_array(origin.media_type)?,
                end_time: origin.end_time,
                published: origin.published,
                summary: from_lax_array(origin.summary)?,
                summary_map: match origin.summary_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                updated: origin.updated,
                describes: match origin.describes {
                    None => None,
                    Some(item) => Some(Box::new(model::Object::to_model(*item)?)),
                },
            },
            actor_items: match (
                origin.inbox,
                origin.outbox,
                origin.followers,
                origin.following,
            ) {
                (Some(inbox), Some(outbox), Some(followers), Some(following)) => {
                    Some(model::ActorItems {
                        inbox,
                        outbox,
                        following,
                        followers,
                        preferred_username: origin.preferred_username,
                        endpoints: match origin.endpoints {
                            None => HashMap::new(),
                            Some(item) => item,
                        },
                    })
                }
                _ => None,
            },
            activity_items: model::ActivityItems {
                actor: from_lax_array(origin.actor)?,
                instrument: from_lax_array(origin.instrument)?,
                origin: from_lax_array(origin.origin)?,
                object: from_lax_array(origin.object)?,
                result: from_lax_array(origin.result)?,
                target: from_lax_array(origin.target)?,
            },
            collection_items: model::CollectionItems {
                total_items: origin.total_items,
                current: match origin.current {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                first: match origin.first {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                last: match origin.last {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                items: from_lax_array(origin.items)?,
            },
            ordered_collection_items: model::OrderedCollectionItems {
                ordered_items: from_lax_array(origin.ordered_items)?,
            },
            collection_page_items: model::CollectionPageItems {
                next: match origin.next {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                prev: match origin.prev {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                part_of: match origin.part_of {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
            },
            ordered_collection_page_items: model::OrderedCollectionPageItems {
                start_index: origin.start_index,
            },
            relationship_items: model::RelationshipItems {
                subject: match origin.subject {
                    None => None,
                    Some(item) => Some(model::ObjectOrLink::to_model(*item)?),
                },
                relationship: from_lax_array(origin.relationship)?,
            },
            tombstone_items: model::TombstoneItems {
                former_type: from_lax_array(origin.former_type)?,
                deleted: origin.deleted,
            },
            question_items: model::QuestionItems {
                one_of: from_lax_array(origin.one_of)?,
                any_of: from_lax_array(origin.any_of)?,
                closed: origin.closed,
            },
            place_items: model::PlaceItems {
                accuracy: origin.accuracy,
                altitude: origin.altitude,
                latitute: origin.latitute,
                longitute: origin.longitute,
                radius: origin.radius,
                units: origin.units,
            },
            activity_streams_ext_items: model::ActivityStreamExtItems {
                manually_approves_followers: origin.manually_approves_followers,
                also_known_as: from_lax_array(origin.also_known_as)?,
                moved_to: origin.moved_to,
            },
            mastodon_ext_items: model::MastodonExtItems {
                featured: origin.featured,
                featured_tags: origin.featured_tags,
                discoverable: origin.discoverable,
                suspended: origin.suspended,
                devices: origin.devices,
            },
            security_items: model::SecurityItems {
                public_key: match origin.public_key {
                    None => None,
                    Some(item) => Some(model::Key::to_model(item)?),
                },
            },
        })
    }
}

impl ModelConv for model::Link {
    type JsonSerdeValue = Link;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(Link {
            schema_context: match self.schema_context {
                None => None,
                Some(item) => Some(item.from_model()?),
            },
            id: self.id,
            typ: to_lax_array(self.typ)?,
            href: self.href,
            height: self.height,
            hreflang: self.hreflang,
            media_type: to_lax_array(self.media_type)?,
            rel: to_lax_array(self.rel)?,
            width: self.width,
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(model::Link {
            schema_context: match origin.schema_context {
                None => None,
                Some(item) => Some(model::Context::to_model(item)?),
            },
            id: origin.id,
            typ: from_lax_array(origin.typ)?,
            href: origin.href,
            height: origin.height,
            hreflang: origin.hreflang,
            media_type: from_lax_array(origin.media_type)?,
            rel: from_lax_array(origin.rel)?,
            width: origin.width,
        })
    }
}

impl ModelConv for model::ObjectOrLink {
    type JsonSerdeValue = ObjectOrLink;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Object(origin) => Ok(ObjectOrLink::Object(origin.from_model()?)),
            Self::Link(origin) => {
                if {
                    origin.height.is_none()
                        && origin.hreflang.is_none()
                        && origin.id.is_none()
                        && origin.media_type.is_empty()
                        && origin.rel.is_empty()
                        && origin.typ.is_empty()
                        && origin.width.is_none()
                } {
                    Ok(ObjectOrLink::Uri(origin.href))
                } else {
                    Ok(ObjectOrLink::Link(origin.from_model()?))
                }
            }
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            ObjectOrLink::Link(origin) => Ok(Self::Link(model::Link::to_model(origin)?)),
            ObjectOrLink::Uri(origin) => Ok(Self::Link(model::Link {
                schema_context: None,
                id: None,
                typ: vec![],
                href: origin,
                height: None,
                hreflang: None,
                media_type: vec![],
                rel: vec![],
                width: None,
            })),
            ObjectOrLink::Object(origin) => {
                Ok(Self::Object(Box::new(model::Object::to_model(origin)?)))
            }
        }
    }
}

impl ModelConv for model::Context {
    type JsonSerdeValue = Context;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Single(origin) => Ok(Context::Single(origin.from_model()?)),
            Self::Mix(origin) => {
                let mut dest = Vec::with_capacity(origin.len());
                for item in origin {
                    dest.push(item.from_model()?);
                }
                Ok(Context::Mix(dest))
            }
            Self::TermDefs(origin) => {
                let mut dest = HashMap::with_capacity(origin.len());
                for (key, item) in origin {
                    dest.insert(key, item.from_model()?);
                }
                Ok(Context::TermDefs(dest))
            }
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            Context::Single(origin) => Ok(model::Context::Single(ModelConv::to_model(origin)?)),
            Context::Mix(origin) => {
                let mut dest = Vec::with_capacity(origin.len());
                for item in origin {
                    dest.push(ModelConv::to_model(item)?)
                }
                Ok(model::Context::Mix(dest))
            }
            Context::TermDefs(origin) => {
                let mut dest = HashMap::with_capacity(origin.len());
                for (key, item) in origin {
                    dest.insert(key, ModelConv::to_model(item)?);
                }
                Ok(model::Context::TermDefs(dest))
            }
        }
    }
}

impl ModelConv for model::Iri {
    type JsonSerdeValue = Iri;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Direct(origin) => Ok(Iri::Direct(origin)),
            Self::TypeCoercion { id, typ } => Ok(Iri::TypeCoercion(TypeCoercion { id, typ })),
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            Iri::Direct(origin) => Ok(model::Iri::Direct(origin)),
            Iri::TypeCoercion(origin) => Ok(model::Iri::TypeCoercion {
                id: origin.id,
                typ: origin.typ,
            }),
        }
    }
}

impl ModelConv for model::Key {
    type JsonSerdeValue = Key;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(Key {
            id: self.id,
            owner: self.owner,
            public_key_pem: self.public_key_pem,
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: origin.id,
            owner: origin.owner,
            public_key_pem: origin.public_key_pem,
        })
    }
}

impl ModelConv for String {
    type JsonSerdeValue = String;

    fn from_model(self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(self)
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(origin)
    }
}

pub fn to_lax_array<T: ModelConv>(origin: Vec<T>) -> Result<Option<Value>, Box<dyn Error>> {
    match origin.len() {
        0 | 1 => {
            for item in origin {
                return Ok(Some(serde_json::to_value(item.from_model()?)?));
            }
            Ok(None)
        }
        _ => {
            let mut dest = Vec::with_capacity(origin.len());
            for item in origin {
                dest.push(serde_json::to_value(item.from_model()?)?)
            }
            Ok(Some(Value::Array(dest)))
        }
    }
}

pub fn from_lax_array<T: ModelConv>(origin: Option<Value>) -> Result<Vec<T>, Box<dyn Error>> {
    match origin {
        None => Ok(vec![]),
        Some(origin) => {
            if origin.is_array() {
                let inter: Vec<T::JsonSerdeValue> = serde_json::from_value(origin)?;
                let mut dest = Vec::with_capacity(inter.len());
                for item in inter {
                    dest.push(T::to_model(item)?);
                }
                Ok(dest)
            } else {
                Ok(vec![T::to_model(serde_json::from_value(origin)?)?])
            }
        }
    }
}

/**
 * Schema: https://www.w3.org/TR/json-ld/#the-context
 */
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Context {
    Single(Iri),
    Mix(Vec<Context>),
    TermDefs(HashMap<String, Iri>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Iri {
    Direct(String),
    TypeCoercion(TypeCoercion),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TypeCoercion {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    typ: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Object {
    #[serde(rename = "@context")]
    schema_context: Option<Context>,
    id: Option<String>,
    #[serde(rename = "type")]
    typ: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Object
    attachment: Option<Value>,
    #[serde(rename = "attributeTo")]
    attributed_to: Option<Value>,
    audience: Option<Value>,
    bcc: Option<Value>,
    bto: Option<Value>,
    cc: Option<Value>,
    context: Option<Value>,
    generator: Option<Value>,
    // Range: Image | Link
    icon: Option<Value>,
    // Range: Image | Link
    image: Option<Value>,
    #[serde(rename = "inReplyTo")]
    in_reply_to: Option<Value>,
    location: Option<Value>,
    preview: Option<Value>,
    // Range: Collection
    replies: Option<Box<Object>>,
    tag: Option<Value>,
    to: Option<Value>,
    url: Option<Value>,
    content: Option<Value>,
    #[serde(rename = "contentMap")]
    content_map: Option<HashMap<String, String>>,
    name: Option<Value>,
    #[serde(rename = "nameMap")]
    name_map: Option<HashMap<String, String>>,
    duration: Option<String>,
    #[serde(rename = "mediaType")]
    media_type: Option<Value>,
    #[serde(rename = "endTime")]
    end_time: Option<DateTime<Utc>>,
    published: Option<DateTime<Utc>>,
    summary: Option<Value>,
    #[serde(rename = "summaryMap")]
    summary_map: Option<HashMap<String, String>>,
    updated: Option<DateTime<Utc>>,
    describes: Option<Box<Object>>,

    // https://www.w3.org/ns/activitystreams#Actor
    inbox: Option<String>,
    outbox: Option<String>,
    following: Option<String>,
    followers: Option<String>,
    #[serde(rename = "preferredUsername")]
    preferred_username: Option<String>,
    endpoints: Option<HashMap<String, String>>,

    // https://www.w3.org/ns/activitystreams#Activity
    actor: Option<Value>,
    instrument: Option<Value>,
    origin: Option<Value>,
    object: Option<Value>,
    result: Option<Value>,
    target: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Collection
    #[serde(rename = "totalItems")]
    total_items: Option<usize>,
    // Range: CollectionPage | Link
    current: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    first: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    last: Option<Box<ObjectOrLink>>,
    items: Option<Value>,

    // https://www.w3.org/ns/activitystreams#OrderedCollection
    #[serde(rename = "orderedItems")]
    ordered_items: Option<Value>,

    // https://www.w3.org/ns/activitystreams#CollectionPage
    next: Option<Box<ObjectOrLink>>,
    prev: Option<Box<ObjectOrLink>>,
    // Range: Link | Collection
    #[serde(rename = "partOf")]
    part_of: Option<Box<ObjectOrLink>>,

    // https://www.w3.org/ns/activitystreams#OrderedCollectionPage
    #[serde(rename = "startIndex")]
    start_index: Option<usize>,

    // https://www.w3.org/ns/activitystreams#Relationship
    subject: Option<Box<ObjectOrLink>>,
    relationship: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Tombstone
    former_type: Option<Value>,
    deleted: Option<DateTime<Utc>>,

    // https://www.w3.org/ns/activitystreams#Question
    #[serde(rename = "oneOf")]
    one_of: Option<Value>,
    #[serde(rename = "anyOf")]
    any_of: Option<Value>,
    closed: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Place
    accuracy: Option<f64>,
    altitude: Option<f64>,
    latitute: Option<f64>,
    longitute: Option<f64>,
    radius: Option<f64>,
    units: Option<String>,

    // https://docs.joinmastodon.org/spec/activitypub/#as
    #[serde(rename = "manuallyApprovesFollowers")]
    manually_approves_followers: Option<bool>,
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Option<Value>,
    #[serde(rename = "movedTo")]
    moved_to: Option<String>,

    // http://joinmastodon.org/ns#featured
    featured: Option<String>,

    // http://joinmastodon.org/ns#featuredTags
    #[serde(rename = "featuredTags")]
    featured_tags: Option<String>,

    // http://joinmastodon.org/ns#discoverable
    discoverable: Option<bool>,

    // http://joinmastodon.org/ns#suspended
    suspended: Option<bool>,

    // http://joinmastodon.org/ns#devices
    devices: Option<String>,

    // https://w3id.org/security/v1
    #[serde(rename = "publicKey")]
    public_key: Option<Key>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Link {
    schema_context: Option<Context>,
    id: Option<String>,
    typ: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Link
    href: String,
    height: Option<usize>,
    hreflang: Option<String>,
    media_type: Option<Value>,
    rel: Option<Value>,
    width: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum ObjectOrLink {
    Uri(String),
    Link(Link),
    Object(Object),
}

/**
 * Reference: https://w3c.github.io/vc-data-integrity/vocab/security/vocabulary.html#Key
 */
#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Key {
    id: String,
    owner: String,
    #[serde(rename = "publicKeyPem")]
    public_key_pem: Option<String>,
}
