//! To populate the database with pre-created data.

use mongodb::sync::Client;
use regex::Regex;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, error::Error, fs, io::ErrorKind};

use crate::models::{
    caching::Caching,
    db_query_api::{commons::QCommons, paladins::QPaladins},
    Meta,
};

/// To populate the database with pre-created data.
///
/// 1.Create a fixtures folder at the root of the project.
/// 2.Save data files for Models in it.
///
/// # Example:
///
/// ```
/// // Initial data for Model.
/// // ./fixtures/cities.json
/// [
///   {
///     "city_name": "London",
///     "description": "London is the capital city of England and the United Kingdom.",
///  },
///  {
///     "city_name": "Dresden",
///     "description": "Dresden is the capital of the East German state of Saxony.",
///  }
/// ]
///
/// // Run fixtures
/// fn run_migration() -> Result<(), Box<dyn Error>> {
///     ...
///     // fixture_name - Name of the fixture file in the ./fixtures directory, no extension (.json).
///     ModelName::run_fixture("cities", &meta_store, &client, &validators, &media_dir)?;
///     Ok(())
/// }
/// ```
///
pub trait Fixtures: Caching + QPaladins + QCommons {
    fn run_fixture(
        fixture_name: &str,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        validators: &HashMap<String, Regex>,
        media_dir: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // If the collection is not empty, exit the method
        if Self::estimated_document_count(meta_store, client, None)? > 0 {
            return Ok(());
        }
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().unwrap();
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `run_fixture()` => \
                Failed to get data from cache.",
            ))?
        };
        let model_name = meta.model_name.clone();
        let model_json = meta.model_json.clone();
        let field_type_map = meta.field_type_map.clone();
        // Get data from fixture file
        let json_val = {
            // Create path
            let fixture_path = format!("./fixtures/{fixture_name}.json");
            // Get json-line
            let json_str = fs::read_to_string(fixture_path.clone()).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    Err(format!(
                        "Model: `{model_name}` > Method: \
                            `run_fixture()` => File is missing - {fixture_path}",
                    ))
                    .unwrap()
                } else {
                    Err(format!(
                        "Model: `{model_name}` > Method: \
                            `run_fixture()` => Problem opening the file: {0:?}",
                        error
                    ))
                    .unwrap()
                }
            });
            serde_json::from_str::<Value>(json_str.as_str())?
        };
        // Unlock
        drop(store);
        // Get an array of fixtures
        if let Some(fixtures_vec) = json_val.as_array() {
            for fixture in fixtures_vec {
                let mut model_json = model_json.clone();
                for (field_name, field_type) in field_type_map.iter() {
                    if let Some(data) = fixture.get(field_name) {
                        let value_key = if field_type == "CheckBox" {
                            "checked"
                        } else {
                            "value"
                        };
                        *model_json
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut(value_key)
                            .unwrap() = data.clone();
                    }
                }
                // Get an instance of the model and save the data to the database
                let mut instance = serde_json::from_value::<Self>(model_json)?;
                let output_data =
                    instance.save(meta_store, client, validators, media_dir, None, None)?;
                if !output_data.is_valid() {
                    Err(format!(
                        "Model: `{model_name}` > Method: `run_fixture()` => {}",
                        output_data.err_msg()
                    ))?
                }
            }
        } else {
            Err(format!(
                "Model: `{model_name}` > Method: \
                `run_fixture()` => Fixture does not contain an array of objects."
            ))?
        }

        Ok(())
    }
}
