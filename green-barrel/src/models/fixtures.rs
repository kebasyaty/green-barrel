//! To populate the database with pre-created data.

use async_trait::async_trait;
use mongodb::Client;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::{error::Error, fs, io::ErrorKind};

use crate::{
    meta_store::META_STORE,
    models::{
        caching::Caching,
        db_query_api::{commons::QCommons, paladins::QPaladins},
    },
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
///     let fixture_name = "cities";
///     ModelName::run_fixture(&client, fixture_name).await?;
///     Ok(())
/// }
/// ```
///
#[async_trait(?Send)]
pub trait Fixtures: Caching + QPaladins + QCommons {
    async fn run_fixture(client: &Client, fixture_name: &str) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // If the collection is not empty, exit the method
        if Self::estimated_document_count(client, None).await? > 0 {
            return Ok(());
        }
        let (model_name, model_json, field_type_map) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.model_name.clone(),
                    meta.model_json.clone(),
                    meta.field_type_map.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `run_fixture()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get data from fixture file
        let json_val = {
            // Create path
            let fixture_path = format!("./fixtures/{fixture_name}.json");
            // Get json-line
            let json_str = fs::read_to_string(fixture_path.clone()).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    Err(format!(
                        "Model: `{model_name}` > Method: \
                        `run_fixture()` => File is missing - {fixture_path}"
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
                let output_data = instance.save(client, None, None).await?;
                if !output_data.is_valid() {
                    Err(format!(
                        "Model: `{model_name}` > Method: `run_fixture()` => {0}",
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
