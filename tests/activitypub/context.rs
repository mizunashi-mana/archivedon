use archivedon::activitypub::context::Context;

#[test]
fn deserialize() {
    let serialized_data = r#"[
        "https://www.w3.org/ns/activitystreams"
    ]"#;

    let data: Context = serde_json::from_str(serialized_data).unwrap();

    assert_eq!(
        data,
        Context::Mix(vec![Context::from("https://www.w3.org/ns/activitystreams"),])
    );
}
