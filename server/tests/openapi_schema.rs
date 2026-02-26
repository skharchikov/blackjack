use server::routes::openapi;

#[test]
fn openapi_schema_is_up_to_date() {
    let schema =
        serde_json::to_string_pretty(&openapi()).expect("Failed to serialize OpenAPI schema");
    let golden_path = format!("{}/openapi.json", env!("CARGO_MANIFEST_DIR"));

    if std::env::var("BLESS").is_ok() {
        std::fs::write(&golden_path, &schema).expect("Failed to write golden file");
        return;
    }

    let expected = std::fs::read_to_string(&golden_path).unwrap_or_else(|_| {
        panic!("Golden file not found at {golden_path}. Run: BLESS=1 cargo test -p server")
    });

    assert_eq!(
        schema, expected,
        "OpenAPI schema is out of date. Run: BLESS=1 cargo test -p server"
    );
}
