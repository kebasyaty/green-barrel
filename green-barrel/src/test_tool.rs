//! Auxiliary tools for testing models.

use mongodb::Client;
use std::error::Error;

use crate::store::METADATA;

/// Remove test databases
/// Hint: See the tests in the `test-drive` section for an example.
pub async fn del_test_db(
    client: &Client,
    app_name: &str,
    unique_app_key: &str,
    model_key_list: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    // Get metadata store
    let metadata = { METADATA.lock().await.clone() };
    // Name of the technical database for testing
    let db_green_tech = format!("green_tech__{app_name}__{unique_app_key}");
    // Removing databases
    for model_key in model_key_list.iter() {
        // Get metadata of Model.
        let meta = if let Some(meta) = metadata.get(model_key) {
            meta
        } else {
            Err(format!(
                "Model key: `{model_key}` ; Method: `json()` => \
                Failed to get data from cache.",
            ))?
        };
        let database_names: Vec<String> = client.list_database_names(None, None).await?;
        if database_names.contains(&db_green_tech) {
            client.database(db_green_tech.as_str()).drop(None).await?;
        }
        if database_names.contains(&meta.database_name) {
            client
                .database(meta.database_name.as_str())
                .drop(None)
                .await?;
        }
    }
    //
    Ok(())
}
