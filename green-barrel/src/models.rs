//! Adapts the Structure for database queries using a programmatic or web interface.

pub mod caching;
pub mod control;
pub mod converters;
pub mod db_query_api;
pub mod fixtures;
pub mod helpers;
pub mod hooks;
pub mod output_data;
pub mod validation;
use async_trait::async_trait;

use mongodb::{
    bson::{doc, document::Document, oid::ObjectId},
    Client,
};
use serde_json::{json, value::Value};
use std::error::Error;

use crate::models::helpers::Meta;

/// Model options and field type map for Form.
// *************************************************************************************************
#[async_trait(?Send)]
pub trait Main {
    /// Get model key
    /// ( to access model metadata in cache ).
    // ---------------------------------------------------------------------------------------------
    fn key() -> Result<String, Box<dyn Error>>;

    /// Model instance from `create` method, convert to intermediate state `serde_json::value::Value`,
    /// with the addition of Html-ID and data validation.
    // ---------------------------------------------------------------------------------------------
    fn custom_default_to_json_val() -> Result<Value, Box<dyn Error>>
    where
        Self: serde::de::DeserializeOwned + Sized;

    /// Generate metadata of Model.
    // ---------------------------------------------------------------------------------------------
    fn generate_metadata() -> Result<Meta, Box<dyn Error>>
    where
        Self: serde::de::DeserializeOwned + Sized;

    /// Getter and Setter for field `hash`.
    // ---------------------------------------------------------------------------------------------
    fn hash(&self) -> String;
    fn set_hash(&mut self, value: String);

    /// ObjectId from hash field.
    // ---------------------------------------------------------------------------------------------
    fn obj_id(&self) -> Result<Option<ObjectId>, Box<dyn Error>>;

    /// ObjectId to hash field.
    // ---------------------------------------------------------------------------------------------
    fn set_obj_id(&mut self, object_id: ObjectId);

    /// Getter and Setter for field `created_at`.
    // ---------------------------------------------------------------------------------------------
    fn created_at(&self) -> String;
    fn set_created_at(&mut self, value: String);

    /// Getter and Setter for field `updated_at`.
    // ---------------------------------------------------------------------------------------------
    fn updated_at(&self) -> String;
    fn set_updated_at(&mut self, value: String);

    /// Serializing the model instance to serde_json::Value format.
    // ---------------------------------------------------------------------------------------------
    fn self_to_json_val(&self) -> Result<Value, Box<dyn Error>>;

    /// Enrich field type map with values for dynamic fields type.
    // ---------------------------------------------------------------------------------------------
    async fn injection(
        client: &Client,
        app_name: &str,
        unique_app_key: &str,
        collection_name: &str,
        model_json: &mut Value,
        fields_name: &Vec<String>,
    ) -> Result<(), Box<dyn Error>>
    where
        Self: serde::de::DeserializeOwned + Sized,
    {
        // Init the name of the project's technical database.
        let db_green_tech: String = format!("green_tech__{app_name}__{unique_app_key}");
        // Access to the collection with values for dynamic fields type.
        let collection = client
            .database(&db_green_tech)
            .collection::<Document>("dynamic_fields");
        // Filter for searching a document.
        let filter = doc! {
            "collection": collection_name
        };
        // Get a document with values for dynamic fields type.
        if let Some(doc) = collection.find_one(filter, None).await? {
            let dyn_values_doc = doc.get_document("fields")?;
            // Updating the `options` parameter for fields with a dynamic field type.
            for field_name in fields_name {
                let field_type = model_json
                    .get(field_name)
                    .unwrap()
                    .get("field_type")
                    .unwrap()
                    .as_str()
                    .unwrap();
                //
                if field_type.contains("Dyn") {
                    let arr = dyn_values_doc.get_array(field_name)?;
                    if field_type.contains("Text") {
                        let options = arr
                            .iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_str().unwrap().to_string(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(String, String)>>();
                        *model_json
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut("options")
                            .unwrap() = json!(options);
                    } else if field_type.contains("I32") {
                        let options = arr
                            .iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_i32().unwrap(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(i32, String)>>();
                        *model_json
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut("options")
                            .unwrap() = json!(options);
                    } else if field_type.contains("U32") || field_type.contains("I64") {
                        let options = arr
                            .iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_i64().unwrap(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(i64, String)>>();
                        *model_json
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut("options")
                            .unwrap() = json!(options);
                    } else if field_type.contains("F64") {
                        let options = arr
                            .iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_f64().unwrap(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(f64, String)>>();
                        *model_json
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut("options")
                            .unwrap() = json!(options);
                    } else {
                        Err(format!(
                            "Model: {} > Method: `injection()` => \
                                Invalid data type.",
                            Self::generate_metadata()?.model_name,
                        ))?
                    }
                }
            }
        }

        Ok(())
    }
}
