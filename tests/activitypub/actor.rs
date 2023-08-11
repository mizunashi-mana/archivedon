use std::{collections::HashMap, str::FromStr};

use archivedon::activitypub::actor::{Actor, default_context};
use chrono::DateTime;
use serde_json::Value;

#[test]
fn serialize() {
    let data = Actor {
        schema_context: default_context(),
        id: String::from("https://example.com/users/sample"),
        typ: String::from("Person"),
        name: Some(String::from("Name")),
        summary: Some(String::from("Summary")),
        url: Some(String::from("https://example.com/@sample")),
        inbox: String::from("https://example.com/users/sample/inbox"),
        outbox: String::from("https://example.com/users/sample/outbox"),
        following: String::from("https://example.com/users/sample/following"),
        followers: String::from("https://example.com/users/sample/followers"),
        manually_approves_followers: Some(false),
        also_known_as: Some(vec![String::from("https://example.com/users/sample_alias"),]),
        moved_to: Some(String::from("https://example.com/users/sample_alias")),
        featured: Some(String::from("https://example.com/users/sample/collections/featured")),
        featured_tags: Some(String::from("https://example.com/users/sample/collections/tags")),
        discoverable: Some(true),
        suspended: Some(true),
        devices: Some(String::from("https://example.com/users/sample/collections/devices")),
        preferred_username: Some(String::from("sample")),
        endpoints: Some(HashMap::from([
            (String::from("sharedInbox"), String::from("https://example.com/inbox")),
        ])),
        published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
    };
    let serialized_data = serde_json::to_value(&data).unwrap();
    let expected_data = r#"{
        "@context": [
          "https://www.w3.org/ns/activitystreams",
          "https://w3id.org/security/v1",
          {
            "movedTo": {
              "@id": "as:movedTo",
              "@type": "@id"
            },
            "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
            "alsoKnownAs": {
              "@id": "as:alsoKnownAs",
              "@type": "@id"
            }
          },
          {
            "toot": "http://joinmastodon.org/ns#",
            "featured": {
              "@id": "toot:featured",
              "@type": "@id"
            },
            "discoverable": "toot:discoverable",
            "featuredTags": {
              "@id": "toot:featuredTags",
              "@type": "@id"
            },
            "suspended": "toot:suspended",
            "devices": {
                "@id": "toot:devices",
                "@type": "@id"
            }
          }
        ],
        "id": "https://example.com/users/sample",
        "type": "Person",
        "name": "Name",
        "summary": "Summary",
        "url": "https://example.com/@sample",
        "inbox": "https://example.com/users/sample/inbox",
        "outbox": "https://example.com/users/sample/outbox",
        "following": "https://example.com/users/sample/following",
        "followers": "https://example.com/users/sample/followers",
        "manuallyApprovesFollowers": false,
        "alsoKnownAs": [
          "https://example.com/users/sample_alias"
        ],
        "movedTo": "https://example.com/users/sample_alias",
        "featured": "https://example.com/users/sample/collections/featured",
        "featuredTags": "https://example.com/users/sample/collections/tags",
        "discoverable": true,
        "suspended": true,
        "devices": "https://example.com/users/sample/collections/devices",
        "preferredUsername": "sample",
        "endpoints": {
            "sharedInbox": "https://example.com/inbox"
        },
        "published": "2023-04-15T11:22:33Z"
    }"#;
    let expected_data: Value = serde_json::from_str(expected_data).unwrap();

    assert_eq!(
        serialized_data,
        expected_data,
    );
}

#[test]
fn deserialize() {
    let serialized_data = r#"{
        "@context": [
          "https://www.w3.org/ns/activitystreams",
          "https://w3id.org/security/v1",
          {
            "movedTo": {
              "@id": "as:movedTo",
              "@type": "@id"
            },
            "manuallyApprovesFollowers": "as:manuallyApprovesFollowers",
            "alsoKnownAs": {
              "@id": "as:alsoKnownAs",
              "@type": "@id"
            }
          },
          {
            "toot": "http://joinmastodon.org/ns#",
            "featured": {
              "@id": "toot:featured",
              "@type": "@id"
            },
            "discoverable": "toot:discoverable",
            "featuredTags": {
              "@id": "toot:featuredTags",
              "@type": "@id"
            },
            "suspended": "toot:suspended",
            "devices": {
                "@id": "toot:devices",
                "@type": "@id"
            }
          }
        ],
        "id": "https://example.com/users/sample",
        "type": "Person",
        "name": "Name",
        "summary": "Summary",
        "url": "https://example.com/@sample",
        "inbox": "https://example.com/users/sample/inbox",
        "outbox": "https://example.com/users/sample/outbox",
        "following": "https://example.com/users/sample/following",
        "followers": "https://example.com/users/sample/followers",
        "manuallyApprovesFollowers": false,
        "alsoKnownAs": [
          "https://example.com/users/sample_alias"
        ],
        "movedTo": "https://example.com/users/sample_alias",
        "featured": "https://example.com/users/sample/collections/featured",
        "featuredTags": "https://example.com/users/sample/collections/tags",
        "discoverable": true,
        "suspended": true,
        "devices": "https://example.com/users/sample/collections/devices",
        "preferredUsername": "sample",
        "endpoints": {
            "sharedInbox": "https://example.com/inbox"
        },
        "published": "2023-04-15T11:22:33Z"
    }"#;
    let data: Actor = serde_json::from_str(serialized_data).unwrap();

    assert_eq!(
        data,
        Actor {
            schema_context: default_context(),
            id: String::from("https://example.com/users/sample"),
            typ: String::from("Person"),
            name: Some(String::from("Name")),
            summary: Some(String::from("Summary")),
            url: Some(String::from("https://example.com/@sample")),
            inbox: String::from("https://example.com/users/sample/inbox"),
            outbox: String::from("https://example.com/users/sample/outbox"),
            following: String::from("https://example.com/users/sample/following"),
            followers: String::from("https://example.com/users/sample/followers"),
            manually_approves_followers: Some(false),
            also_known_as: Some(vec![String::from("https://example.com/users/sample_alias"),]),
            moved_to: Some(String::from("https://example.com/users/sample_alias")),
            featured: Some(String::from("https://example.com/users/sample/collections/featured")),
            featured_tags: Some(String::from("https://example.com/users/sample/collections/tags")),
            discoverable: Some(true),
            suspended: Some(true),
            devices: Some(String::from("https://example.com/users/sample/collections/devices")),
            preferred_username: Some(String::from("sample")),
            endpoints: Some(HashMap::from([
                (String::from("sharedInbox"), String::from("https://example.com/inbox")),
            ])),
            published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
        }
    );
}
