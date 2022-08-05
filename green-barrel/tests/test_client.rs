// Testing of Client
// *************************************************************************************************
// cargo test test_client -- --nocapture
#[test]
fn test_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017/")?;
    assert!(!client.list_database_names(None, None)?.is_empty());
    Ok(())
}
