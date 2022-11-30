//! Auxiliary tools for testing models.

use mongodb::sync::Client;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, error::Error};

use crate::models::helpers::Meta;

/// Remove test databases
/// Hint: See the tests in the `test-drive` section for an example.
pub fn del_test_db(
    project_name: &str,
    unique_project_key: &str,
    model_key_list: &Vec<String>,
    meta_store: &Arc<Mutex<HashMap<String, Meta>>>,
    client: &Client,
) -> Result<(), Box<dyn Error>> {
    // Get metadata store
    let store = meta_store.lock().unwrap();
    // Name of the technical database for testing
    let db_green_tech = format!("green_tech__{}__{}", project_name, unique_project_key);
    // Removing databases
    for model_key in model_key_list {
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(model_key) {
            meta
        } else {
            Err(format!(
                "Model key: `{model_key}` ; Method: `json()` => \
                Failed to get data from cache.",
            ))?
        };
        let database_names: Vec<String> = client.list_database_names(None, None)?;
        if database_names.contains(&db_green_tech) {
            client.database(db_green_tech.as_str()).drop(None)?;
        }
        if database_names.contains(&meta.database_name) {
            client.database(meta.database_name.as_str()).drop(None)?;
        }
    }
    //
    Ok(())
}
