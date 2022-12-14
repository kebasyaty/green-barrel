// Testing of Client
// *************************************************************************************************
// cargo test test_client -- --nocapture
#[tokio::test]
async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = mongodb::Client::with_uri_str(uri).await?;
    assert!(!client.list_database_names(None, None).await?.is_empty());
    Ok(())
}
