use std::{collections::HashMap, str::FromStr};

use archivedon::activitypub::json::ModelConv;
use archivedon::activitypub::model as ap_model;
use chrono::DateTime;
use serde_json::Value;

#[test]
fn deserialize_context() {
    let serialized_data = r#"[
        "https://www.w3.org/ns/activitystreams"
    ]"#;

    let data: ap_model::Context =
        ModelConv::to_model(serde_json::from_str(serialized_data).unwrap()).unwrap();

    assert_eq!(
        data,
        ap_model::Context::Mix(vec![ap_model::Context::from(
            "https://www.w3.org/ns/activitystreams"
        ),])
    );
}

#[test]
fn serialize_object() {
    let data = ap_model::Object {
        schema_context: Some(ap_model::Context::object_default()),
        id: Some("https://example.com/users/sample".to_string()),
        typ: vec!["Person".to_string()],
        object_items: ap_model::ObjectItems {
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
            url: Some(ap_model::Link::from("https://example.com/@sample")),
            content: vec![],
            content_map: HashMap::new(),
            name: vec!["Name".to_string()],
            name_map: HashMap::new(),
            duration: None,
            media_type: vec![],
            end_time: None,
            published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
            summary: vec!["Summary".to_string()],
            summary_map: HashMap::new(),
            updated: None,
            describes: None,
        },
        actor_items: Some(ap_model::ActorItems {
            inbox: "https://example.com/users/sample/inbox".to_string(),
            outbox: "https://example.com/users/sample/outbox".to_string(),
            following: "https://example.com/users/sample/following".to_string(),
            followers: "https://example.com/users/sample/followers".to_string(),
            preferred_username: Some("sample".to_string()),
            endpoints: HashMap::from([(
                "sharedInbox".to_string(),
                "https://example.com/inbox".to_string(),
            )]),
        }),
        activity_items: ap_model::ActivityItems::empty(),
        collection_items: ap_model::CollectionItems::empty(),
        ordered_collection_items: ap_model::OrderedCollectionItems::empty(),
        collection_page_items: ap_model::CollectionPageItems::empty(),
        ordered_collection_page_items: ap_model::OrderedCollectionPageItems::empty(),
        relationship_items: ap_model::RelationshipItems::empty(),
        tombstone_items: ap_model::TombstoneItems::empty(),
        question_items: ap_model::QuestionItems::empty(),
        place_items: ap_model::PlaceItems::empty(),
        activity_streams_ext_items: ap_model::ActivityStreamExtItems {
            manually_approves_followers: Some(false),
            also_known_as: vec!["https://example.com/users/sample_alias".to_string()],
            moved_to: Some("https://example.com/users/sample_alias".to_string()),
        },
        mastodon_ext_items: ap_model::MastodonExtItems {
            featured: Some("https://example.com/users/sample/collections/featured".to_string()),
            featured_tags: Some("https://example.com/users/sample/collections/tags".to_string()),
            discoverable: Some(true),
            suspended: Some(true),
            devices: Some("https://example.com/users/sample/collections/devices".to_string()),
        },
        security_items: ap_model::SecurityItems {
            public_key: Some(ap_model::Key {
                id: "https://example.com/users/sample#main-key".to_string(),
                owner: "https://example.com/users/sample".to_string(),
                public_key_pem: Some("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n".to_string()),
            }),
        },
        property_items: ap_model::PropertyItems {
            value: None,
        },
    };
    let serialized_data = serde_json::to_value(&data.from_model().unwrap()).unwrap();
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
          },
          {
            "schema": "http://schema.org#",
            "PropertyValue": "schema:PropertyValue",
            "value": "schema:value"
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
        "alsoKnownAs": "https://example.com/users/sample_alias",
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
        "endpoints": {
            "sharedInbox": "https://example.com/inbox"
        }
    }"#;
    let expected_data: Value = serde_json::from_str(expected_data).unwrap();

    assert_eq!(serialized_data, expected_data,);
}

#[test]
fn deserialize_object() {
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
          },
          {
            "schema": "http://schema.org#",
            "PropertyValue": "schema:PropertyValue",
            "value": "schema:value"
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
    let data: ap_model::Object =
        ModelConv::to_model(serde_json::from_str(serialized_data).unwrap()).unwrap();

    assert_eq!(
        data,
        ap_model::Object {
            schema_context: Some(ap_model::Context::Mix(vec![
                ap_model::Context::from("https://www.w3.org/ns/activitystreams"),
                ap_model::Context::from("https://w3id.org/security/v1"),
                ap_model::Context::from([
                    ("manuallyApprovesFollowers", ap_model::Iri::from("as:manuallyApprovesFollowers")),
                    ("alsoKnownAs", ap_model::Iri::TypeCoercion { id: "as:alsoKnownAs".to_string(), typ: Some("@id".to_string()) }),
                    ("movedTo", ap_model::Iri::TypeCoercion { id: "as:movedTo".to_string(), typ: Some("@id".to_string()) })
                ]),
                ap_model::Context::from([
                    ("featuredTags", ap_model::Iri::TypeCoercion { id: "toot:featuredTags".to_string(), typ: Some("@id".to_string()) }),
                    ("devices", ap_model::Iri::TypeCoercion { id: "toot:devices".to_string(), typ: Some("@id".to_string()) }),
                    ("featured", ap_model::Iri::TypeCoercion { id: "toot:featured".to_string(), typ: Some("@id".to_string()) }),
                    ("toot", ap_model::Iri::from("http://joinmastodon.org/ns#")),
                    ("discoverable", ap_model::Iri::from("toot:discoverable")),
                    ("suspended", ap_model::Iri::from("toot:suspended")),
                ]),
                ap_model::Context::from([
                    ("schema", ap_model::Iri::from("http://schema.org#")),
                    ("value", ap_model::Iri::from("schema:value")),
                    ("PropertyValue", ap_model::Iri::from("schema:PropertyValue")),
                ]),
            ])),
            id: Some("https://example.com/users/sample".to_string()),
            typ: vec!["Person".to_string()],
            object_items: ap_model::ObjectItems {
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
                url: Some(ap_model::Link::from("https://example.com/@sample")),
                content: vec![],
                content_map: HashMap::new(),
                name: vec!["Name".to_string()],
                name_map: HashMap::new(),
                duration: None,
                media_type: vec![],
                end_time: None,
                published: Some(DateTime::from_str("2023-04-15T11:22:33Z").unwrap()),
                summary: vec!["Summary".to_string()],
                summary_map: HashMap::new(),
                updated: None,
                describes: None,
            },
            actor_items: Some(ap_model::ActorItems {
                inbox: "https://example.com/users/sample/inbox".to_string(),
                outbox: "https://example.com/users/sample/outbox".to_string(),
                following: "https://example.com/users/sample/following".to_string(),
                followers: "https://example.com/users/sample/followers".to_string(),
                preferred_username: Some("sample".to_string()),
                endpoints: HashMap::from([
                    ("sharedInbox".to_string(), "https://example.com/inbox".to_string()),
                ]),
            }),
            activity_items: ap_model::ActivityItems::empty(),
            collection_items: ap_model::CollectionItems::empty(),
            ordered_collection_items: ap_model::OrderedCollectionItems::empty(),
            collection_page_items: ap_model::CollectionPageItems::empty(),
            ordered_collection_page_items: ap_model::OrderedCollectionPageItems::empty(),
            relationship_items: ap_model::RelationshipItems::empty(),
            tombstone_items: ap_model::TombstoneItems::empty(),
            question_items: ap_model::QuestionItems::empty(),
            place_items: ap_model::PlaceItems::empty(),
            activity_streams_ext_items: ap_model::ActivityStreamExtItems {
                manually_approves_followers: Some(false),
                also_known_as: vec!["https://example.com/users/sample_alias".to_string()],
                moved_to: Some("https://example.com/users/sample_alias".to_string()),
            },
            mastodon_ext_items: ap_model::MastodonExtItems {
                featured: Some("https://example.com/users/sample/collections/featured".to_string()),
                featured_tags: Some("https://example.com/users/sample/collections/tags".to_string()),
                discoverable: Some(true),
                suspended: Some(true),
                devices: Some("https://example.com/users/sample/collections/devices".to_string()),
            },
            security_items: ap_model::SecurityItems {
                public_key: Some(ap_model::Key {
                    id: "https://example.com/users/sample#main-key".to_string(),
                    owner: "https://example.com/users/sample".to_string(),
                    public_key_pem: Some("-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxcce3F6A9ZFVG/q7t/4V\nkCQ0fs7RlLhgynbH/0BBqEq+PUOj77d42bw2LEv/gBE9bHeqyXlPDuZ6qFtzR6Ux\n6z7jjvz7zR0C0XkfmGXiWhMZXt/jHKqiIjVipo82ysI6blsA6F/y7m5ASniPSITk\nvs82dodLA21h3XccFJldtELdPPX3KDeCHN0hvlXHj7R0Z4kNPNleg9xppQ3Ry8es\nOZtJcUHeWRbeVabIVhY7Y75pcdsfIQc3rcXtLkS5iU6bAVAl1riCjWS2XXQDufdG\nrBEiBSFn+sf6ulRy+bzYHgCW1pNr8L7HqWkMcwGxKWyfZ9dhi8fIqYlli8Y1EaHo\ngwIDAQAB\n-----END PUBLIC KEY-----\n".to_string()),
                }),
            },
            property_items: ap_model::PropertyItems {
                value: None,
            },
        },
    );
}

#[test]
fn deserialize_collection() {
    let serialized_data = r#"{
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": "https://example.com/users/sample/outbox",
        "type": "OrderedCollection",
        "totalItems": 0,
        "items": []
    }"#;

    let data: ap_model::Object =
        ModelConv::to_model(serde_json::from_str(serialized_data).unwrap()).unwrap();

    assert_eq!(
        data,
        ap_model::Object::new_collection(
            Some("https://example.com/users/sample/outbox".to_string()),
            vec!["OrderedCollection".to_string()],
            Some(0),
            None,
            None,
            None,
            vec![],
            vec![],
        ),
    );
}
