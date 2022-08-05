//! Auxiliary tools for testing models.

use mongodb::sync::Client;
use std::error::Error;

use crate::{models::Meta, store::MONGODB_CLIENT_STORE};

/// Remove test databases
/// Hint: See the tests in the `test-drive` section for an example.
pub fn del_test_db(
    project_name: &str,
    unique_project_key: &str,
    models: &Vec<Meta>,
) -> Result<(), Box<dyn Error>> {
    // Name of the technical database for testing
    let db_green_tech: String = format!("green_tech__{}__{}", project_name, unique_project_key);
    //
    let client_store = MONGODB_CLIENT_STORE.read()?;
    // Removing databases
    for meta in models {
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let database_names: Vec<String> = client.list_database_names(None, None)?;
        //
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
