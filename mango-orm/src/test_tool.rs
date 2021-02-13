//! # Testing tools
//!.
//! `del_test_base` - Remove test databases.

use crate::store::DB_MAP_CLIENT_NAMES;

// Remove test databases
pub fn del_test_base(
    project_name: &str,
    unique_project_key: &str,
    models: &Vec<crate::models::Meta>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Name of the technical database for testing
    let mango_tech_keyword: String =
        format!("mango_tech__{}__{}", project_name, unique_project_key);
    let client_store = DB_MAP_CLIENT_NAMES.lock()?;
    // Removing databases
    for meta in models {
        let client: &mongodb::sync::Client =
            client_store.get(meta.db_client_name.as_str()).unwrap();
        let database_names: Vec<String> = client.list_database_names(None, None)?;
        //
        if database_names.contains(&mango_tech_keyword) {
            client.database(mango_tech_keyword.as_str()).drop(None)?;
        }
        if database_names.contains(&meta.database_name) {
            client.database(meta.database_name.as_str()).drop(None)?;
        }
    }
    //
    Ok(())
}
