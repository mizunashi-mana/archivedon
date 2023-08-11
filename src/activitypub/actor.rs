use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::activitypub::context::{Context, Iri};

use super::context::TypeCoercion;

pub fn default_context() -> Context {
    Context::Mix(vec![
        Context::from("https://www.w3.org/ns/activitystreams"),
        Context::from("https://w3id.org/security/v1"),
        Context::from([
            (
                "alsoKnownAs",
                Iri::from(TypeCoercion {
                    id: String::from("as:alsoKnownAs"),
                    typ: Some(String::from("@id")),
                }),
            ),
            (
                "manuallyApprovesFollowers",
                Iri::from("as:manuallyApprovesFollowers"),
            ),
            (
                "movedTo",
                Iri::from(TypeCoercion {
                    id: String::from("as:movedTo"),
                    typ: Some(String::from("@id")),
                }),
            ),
        ]),
        Context::from([
            ("toot", Iri::from("http://joinmastodon.org/ns#")),
            ("deviceId", Iri::from("toot:deviceId")),
            (
                "devices",
                Iri::from(TypeCoercion {
                    id: String::from("toot:devices"),
                    typ: Some(String::from("@id")),
                }),
            ),
            ("discoverable", Iri::from("toot:discoverable")),
            (
                "featured",
                Iri::from(TypeCoercion {
                    id: String::from("toot:featured"),
                    typ: Some(String::from("@id")),
                }),
            ),
            (
                "featuredTags",
                Iri::from(TypeCoercion {
                    id: String::from("toot:featuredTags"),
                    typ: Some(String::from("@id")),
                }),
            ),
            ("suspended", Iri::from("toot:suspended")),
        ]),
    ])
}

/**
 * Schema:
 * - https://www.w3.org/ns/activitystreams
 * - https://docs.joinmastodon.org/spec/activitypub/#contexts
 * - https://docs.joinmastodon.org/spec/activitypub/#as
 * - https://docs.joinmastodon.org/spec/activitypub/#toot
 * - https://w3id.org/security/v1
 *
 * Reference:
 * - https://www.w3.org/wiki/Activity_Streams_extensions
 */
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Actor {
    #[serde(rename = "@context")]
    pub schema_context: Context,

    // https://www.w3.org/ns/activitystreams#Object
    pub id: String,
    #[serde(rename = "type")]
    pub typ: String,
    pub name: String,
    pub summary: String,
    pub url: String,

    // https://www.w3.org/ns/activitystreams#Actor
    pub inbox: String,
    pub outbox: String,
    pub following: String,
    pub followers: String,
    #[serde(rename = "preferredUsername")]
    pub preferred_username: Option<String>,
    pub endpoints: Option<HashMap<String, String>>,

    // Extensions of https://www.w3.org/ns/activitystreams#
    #[serde(rename = "manuallyApprovesFollowers")]
    pub manually_approves_followers: Option<bool>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: Option<Vec<String>>,
    #[serde(rename = "movedTo")]
    pub moved_to: Option<String>,

    // http://joinmastodon.org/ns#featured
    pub featured: Option<String>,

    // http://joinmastodon.org/ns#featuredTags
    #[serde(rename = "featuredTags")]
    pub featured_tags: Option<String>,

    // http://joinmastodon.org/ns#discoverable
    pub discoverable: Option<bool>,

    // http://joinmastodon.org/ns#suspended
    pub suspended: Option<bool>,

    // http://joinmastodon.org/ns#devices
    pub devices: Option<String>,

    // http://joinmastodon.org/ns#deviceId
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,

    // https://w3id.org/security/v1
}
