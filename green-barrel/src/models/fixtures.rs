//! To populate the database with pre-created data.

use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::{error::Error, fs, io::ErrorKind};

use crate::models::{caching::Caching, Meta};

/// To populate the database with pre-created data.
/// Create a fixtures folder at the root of the project.
/// Method parameters:
/// `fixture_name` - Name of the fixture file in the ./fixtures directory, no extension (.json).
/// `unique_field`- The name of any unique field in the Model.
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
///     ModelName::run_fixture("cities", "city_name");
///     Ok(())
/// }
/// ```
///
pub trait Fixtures: Caching {
    fn run_fixture(fixture_name: &str, _unique_field: &str) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, _) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        let fields_name = &meta.fields_name;
        // Get fixtures list
        let json_val = {
            // Create path
            let fixture_path = format!("./fixtures/{fixture_name}.json");
            // Get json-line
            let json_str = fs::read_to_string(fixture_path.clone()).unwrap_or_else(|error| {
                if error.kind() == ErrorKind::NotFound {
                    Err(format!(
                        "Model: `{} > Method: \
                    run_fixture()` => File is missing - {fixture_path}",
                        meta.model_name
                    ))
                    .unwrap()
                } else {
                    Err(format!(
                        "Model: `{} > Method: \
                    run_fixture()` => Problem opening the file: {:?}",
                        meta.model_name, error
                    ))
                    .unwrap()
                }
            });
            serde_json::from_str::<Value>(json_str.as_str())?
        };
        //
        if let Some(fixtures_vec) = json_val.as_array() {
            for fixture in fixtures_vec {
                let mut model_json = model_cache.model_json.clone();
                for field_name in fields_name {
                    if let Some(field) = model_json.get_mut(field_name) {
                        *field.get_mut("value").unwrap() = fixture.get(field_name).unwrap().clone();
                    }
                }
                let instance = serde_json::from_value::<Self>(model_json)?;
            }
        } else {
            Err(format!(
                "Model: `{} > Method: \
                    run_fixture()` => Fixture does not contain an array of objects.",
                meta.model_name
            ))?
        }

        Ok(())
    }
}
