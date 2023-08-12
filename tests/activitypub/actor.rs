use std::{collections::HashMap, str::FromStr};

use archivedon::activitypub::actor::{default_context, Actor, Key};
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
        also_known_as: Some(vec![String::from("https://example.com/users/sample_alias")]),
        moved_to: Some(String::from("https://example.com/users/sample_alias")),
        featured: Some(String::from(
            "https://example.com/users/sample/collections/featured",
        )),
        featured_tags: Some(String::from(
            "https://example.com/users/sample/collections/tags",
        )),
        discoverable: Some(true),
        suspended: Some(true),
        devices: Some(String::from(
            "https://example.com/users/sample/collections/devices",
        )),
        preferred_username: Some(String::from("sample")),
        endpoints: Some(HashMap::from([(
            String::from("sharedInbox"),
            String::from("https://example.com/inbox"),
        )])),
        published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
        attachment: Some(vec![]),
        tag: Some(vec![]),
        icon: None,
        image: None,
        public_key: Some(Key {
            id: String::from("https://example.com/users/sample#main-key"),
            owner: String::from("https://example.com/users/sample"),
            public_key_pem: Some(String::from("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n")),
        }),
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
        "published": "2023-04-15T11:22:33Z",
        "publicKey": {
            "id": "https://example.com/users/sample#main-key",
            "owner": "https://example.com/users/sample",
            "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n"
        },
        "tag": [],
        "attachment": [],
        "endpoints": {
            "sharedInbox": "https://example.com/inbox"
        }
    }"#;
    let expected_data: Value = serde_json::from_str(expected_data).unwrap();

    assert_eq!(serialized_data, expected_data,);
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
        "published": "2023-04-15T11:22:33Z",
        "publicKey": {
            "id": "https://example.com/users/sample#main-key",
            "owner": "https://example.com/users/sample",
            "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n"
        },
        "tag": [],
        "attachment": [],
        "endpoints": {
            "sharedInbox": "https://example.com/inbox"
        }
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
            featured: Some(String::from(
                "https://example.com/users/sample/collections/featured"
            )),
            featured_tags: Some(String::from(
                "https://example.com/users/sample/collections/tags"
            )),
            discoverable: Some(true),
            suspended: Some(true),
            devices: Some(String::from(
                "https://example.com/users/sample/collections/devices"
            )),
            preferred_username: Some(String::from("sample")),
            endpoints: Some(HashMap::from([(
                String::from("sharedInbox"),
                String::from("https://example.com/inbox")
            ),])),
            published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
            attachment: Some(vec![]),
            tag: Some(vec![]),
            icon: None,
            image: None,
            public_key: Some(Key {
                id: String::from("https://example.com/users/sample#main-key"),
                owner: String::from("https://example.com/users/sample"),
                public_key_pem: Some(String::from("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n")),
            }),
        }
    );
}
