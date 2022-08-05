//! Adapts the Structure for database queries using a programmatic or web interface.

pub mod administrator;
pub mod caching;
pub mod converters;
pub mod db_query_api;
pub mod hooks;
pub mod output_data;
pub mod validation;

use crate::widgets::Widget;
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Client,
};
use serde::Deserialize;
use serde_json::value::Value;
use std::{collections::HashMap, error::Error};

// MODEL
// #################################################################################################
/// Metadata
/// ( model parameters )
// *************************************************************************************************
#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub model_name: String,
    pub project_name: String,
    pub unique_project_key: String,
    pub service_name: String,
    pub database_name: String,
    pub db_client_name: String,
    pub db_query_docs_limit: u32,
    pub collection_name: String,
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_docs: bool,
    pub is_up_docs: bool,
    pub is_del_docs: bool,
    // <field_name, field_type>.
    pub field_type_map: std::collections::HashMap<String, String>,
    // <field_name, widget_type>.
    pub widget_type_map: std::collections::HashMap<String, String>,
    // <field_name, (widget_type, value)>.
    pub default_value_map: std::collections::HashMap<String, (String, String)>,
    // List of field names that will not be saved to the database.
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
            project_name: String::new(),
            unique_project_key: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_client_name: String::new(),
            db_query_docs_limit: 0_u32,
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_docs: true,
            is_up_docs: true,
            is_del_docs: true,
            field_type_map: std::collections::HashMap::new(),
            widget_type_map: std::collections::HashMap::new(),
            default_value_map: std::collections::HashMap::new(),
            // List of field names that will not be saved to the database.
            ignore_fields: Vec::new(),
        }
    }
}

/// Model options and widget map for Form.
// *************************************************************************************************
pub trait Main {
    /// Get model key
    /// ( to access data in the cache )
    // ---------------------------------------------------------------------------------------------
    fn key() -> Result<String, Box<dyn Error>>;

    /// Get metadata of Model.
    // ---------------------------------------------------------------------------------------------
    fn meta() -> Result<Meta, Box<dyn Error>>;

    /// Get map of widgets for model fields.
    /// Hint: <field name, Widget>
    // ---------------------------------------------------------------------------------------------
    fn widgets() -> Result<HashMap<String, Widget>, Box<dyn Error>>;

    /// Getter and Setter for field `hash`.
    // ---------------------------------------------------------------------------------------------
    fn get_hash(&self) -> String;
    fn set_hash(&mut self, value: String);

    /// Getter and Setter for field `created_at`.
    // ---------------------------------------------------------------------------------------------
    fn get_created_at(&self) -> String;
    fn set_created_at(&mut self, value: String);

    /// Getter and Setter for field `updated_at`.
    // ---------------------------------------------------------------------------------------------
    fn get_updated_at(&self) -> String;
    fn set_updated_at(&mut self, value: String);

    /// Serialize an instance of the Model to a hash-line.
    // ---------------------------------------------------------------------------------------------
    fn self_to_json(&self) -> Result<Value, Box<dyn Error>>;

    /// Convert hash-line to MongoDB ID.
    // ---------------------------------------------------------------------------------------------
    fn hash_to_id(hash: &str) -> Result<ObjectId, Box<dyn Error>> {
        Ok(ObjectId::with_string(hash)?)
    }

    /// Convert MongoDB ID to hash-line.
    // ---------------------------------------------------------------------------------------------
    fn id_to_hash(object_id: ObjectId) -> String {
        object_id.to_hex()
    }

    /// Enrich the widget map with values for dynamic widgets.
    // ---------------------------------------------------------------------------------------------
    fn vitaminize(
        project_name: &str,
        unique_project_key: &str,
        collection_name: &str,
        client: &Client,
        widget_map: &mut HashMap<String, Widget>,
    ) -> Result<(), Box<dyn Error>> {
        // Init the name of the project's technical database.
        let db_mango_tech: String = format!("mango_tech__{}__{}", project_name, unique_project_key);
        // Access to the collection with values for dynamic widgets.
        let collection = client
            .database(&db_mango_tech)
            .collection("dynamic_widgets");
        // Filter for searching a document.
        let filter = doc! {
            "collection": collection_name
        };
        // Get a document with values for dynamic widgets.
        if let Some(doc) = collection.find_one(filter, None)? {
            let dyn_values_doc = doc.get_document("fields")?;
            // Updating the `options` parameter for fields with a dynamic widget.
            for (field_name, widget) in widget_map {
                let widget_type = widget.widget.clone();
                if widget_type.contains("Dyn") {
                    let arr = dyn_values_doc.get_array(field_name)?;
                    let options = if widget_type.contains("Text") {
                        arr.iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_str().unwrap().to_string(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(String, String)>>()
                    } else if widget_type.contains("I32") {
                        arr.iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_i32().unwrap().to_string(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(String, String)>>()
                    } else if widget_type.contains("U32") || widget_type.contains("I64") {
                        arr.iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_i64().unwrap().to_string(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(String, String)>>()
                    } else if widget_type.contains("F64") {
                        arr.iter()
                            .map(|item| {
                                let arr = item.as_array().unwrap();
                                (
                                    arr[0].as_f64().unwrap().to_string(),
                                    arr[1].as_str().unwrap().to_string(),
                                )
                            })
                            .collect::<Vec<(String, String)>>()
                    } else {
                        Err(format!(
                            "Model: {} > Method: `vitaminize` => \
                            Invalid data type.",
                            Self::meta()?.model_name,
                        ))?
                    };
                    //
                    widget.options = options;
                }
            }
        } else {
            Err(format!(
                "Model: {} ; Method: `vitaminize()` => \
                Document with values for dynamic widgets not found.",
                Self::meta()?.model_name
            ))?
        }
        //
        Ok(())
    }
}
