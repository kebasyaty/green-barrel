//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).
//! `ToModel` - Model options and widget map for Form.

use crate::{
    forms::{html_controls::HtmlControls, Widget},
    models::{
        password::Password,
        validation::{AdditionalValidation, Validation},
    },
    store::{FormCache, DB_MAP_CLIENT_NAMES, FORM_CACHE},
};

pub mod caching;
pub mod db_query_api;
pub mod output_data;
pub mod password;
pub mod validation;

// MODEL
// #################################################################################################
// Metadata
// ( Model parameters )
// *************************************************************************************************
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Meta {
    pub model_name: String,
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
    pub map_field_type: std::collections::HashMap<String, String>,
    pub map_widget_type: std::collections::HashMap<String, String>,
    // <field_name, (widget_type, value)>
    pub map_default_values: std::collections::HashMap<String, (String, String)>,
    pub map_related_models:
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
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
            map_field_type: std::collections::HashMap::new(),
            map_widget_type: std::collections::HashMap::new(),
            map_default_values: std::collections::HashMap::new(),
            map_related_models: std::collections::HashMap::new(),
            // List of field names that will not be saved to the database
            ignore_fields: Vec::new(),
        }
    }
}

// Model options and widget map for Form
// *************************************************************************************************
pub trait ToModel: HtmlControls + AdditionalValidation + Validation + Password {
    // // Get a key to access Model data in the cache
    // ( key = collection name, alternatively, not to call `meta()` )
    // ---------------------------------------------------------------------------------------------
    fn key_store() -> Result<String, Box<dyn std::error::Error>>;

    // Get metadata of Model
    // ---------------------------------------------------------------------------------------------
    fn meta() -> Result<Meta, Box<dyn std::error::Error>>;

    // Get map of widgets for model fields
    // <field name, Widget>
    // ---------------------------------------------------------------------------------------------
    fn widgets() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>>;

    // Getter and Setter for field `hash`
    // ---------------------------------------------------------------------------------------------
    fn get_hash(&self) -> Option<String>;
    fn set_hash(&mut self, value: String);

    // Serialize an instance of the Model to a hash-line
    // ---------------------------------------------------------------------------------------------
    fn self_to_json(&self) -> Result<serde_json::value::Value, Box<dyn std::error::Error>>;

    // Convert hash-line to MongoDB ID
    // ---------------------------------------------------------------------------------------------
    fn hash_to_id(hash: &str) -> Result<mongodb::bson::oid::ObjectId, Box<dyn std::error::Error>> {
        Ok(mongodb::bson::oid::ObjectId::with_string(hash)?)
    }

    // Convert MongoDB ID to hash-line
    // ---------------------------------------------------------------------------------------------
    fn id_to_hash(id: mongodb::bson::oid::ObjectId) -> String {
        id.to_hex()
    }

    // Database Query API
    // *********************************************************************************************
    // Get cached Model data
    // (Database Query API)
    // ---------------------------------------------------------------------------------------------
    fn get_cache_data_for_query(
    ) -> Result<(FormCache, mongodb::sync::Client), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache
        let key: String = Self::key_store()?;
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add the `FormCache` structure to the cache for the current model if it is not there
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Get model metadata from cache
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client for current model
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock()?;
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        Ok((form_cache.clone(), client_cache.clone()))
    }
}
