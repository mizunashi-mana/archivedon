use archivedon::activitypub::typ::OrderedCollection;

#[test]
fn deserialize_ordered_collection() {
    let serialized_data = r#""OrderedCollection""#;
    let data: OrderedCollection = serde_json::from_str(serialized_data).unwrap();

    assert_eq!(data, OrderedCollection,);
}
