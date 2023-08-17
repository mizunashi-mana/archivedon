use archivedon::activitypub::collection::{default_context, Collection};

#[test]
fn deserialize() {
    let serialized_data = r#"{
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": "https://example.com/users/sample/outbox",
        "type": "OrderedCollection",
        "totalItems": 0,
        "items": []
    }"#;

    let data: Collection = serde_json::from_str(serialized_data).unwrap();

    assert_eq!(
        data,
        Collection {
            schema_context: default_context(),
            id: "https://example.com/users/sample/outbox".to_string(),
            typ: "OrderedCollection".to_string(),
            total_items: 0,
            items: Some(vec![]),
            ordered_items: None,
            first: None,
            last: None,
        },
    );
}
