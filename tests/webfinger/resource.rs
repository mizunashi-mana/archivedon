use archivedon::webfinger::resource::Resource;
use serde_json::Value;

#[test]
fn serialize() {
    let data = Resource {
        subject: String::from("acct:sample@example.com"),
        aliases: None,
        properties: None,
        links: None,
    };
    let serialized_data = serde_json::to_value(data).unwrap();

    let expected_data = r#"{
        "subject": "acct:sample@example.com"
    }"#;
    let expected_data: Value = serde_json::from_str(expected_data).unwrap();

    assert_eq!(serialized_data, expected_data);
}
