use archivedon::activitypub::{
    outbox::{default_context, Outbox},
    typ::OrderedCollection,
};

#[test]
fn deserialize() {
    let serialized_data = r#"{
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": "https://example.com/users/sample/outbox",
        "type": "OrderedCollection",
        "totalItems": 0,
        "items": []
    }"#;

    let data: Outbox = serde_json::from_str(serialized_data).unwrap();

    assert_eq!(
        data,
        Outbox {
            schema_context: default_context(),
            id: String::from("https://example.com/users/sample/outbox"),
            typ: OrderedCollection,
            total_items: 0,
            items: Some(vec![]),
            first: None,
            last: None,
        },
    );
}
