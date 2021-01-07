//! # Caching.
//! Caching information about Models for speed up work.
//!
//! Trait:
//! `Caching` - Methods caching information about Models for speed up work.
//! Methods:
//! `form_wig` - Get an widgets map for page template.
//! `form_json` - Get Form attributes in Json format for page templates.
//! `form_html` - Get Html Form of Model for page templates.
//!

use crate::{
    forms::Widget,
    models::{Meta, ToModel},
    store::{FormCache, DB_MAP_CLIENT_NAMES, FORM_CACHE},
};

// Caching information about Models for speed up work.
// *************************************************************************************************
pub trait CachingModel: ToModel {
    // Get an widgets map for page template.
    // ---------------------------------------------------------------------------------------------
    fn form_wig() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        Ok(form_cache.unwrap().map_widgets.clone())
    }

    // Get Form attributes in Json format for page templates.
    // ---------------------------------------------------------------------------------------------
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there.
        if form_cache.attrs_json.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let json_line = serde_json::to_string(&map_widgets)?;
            let mut form_cache: FormCache = form_cache.clone();
            // Update data.
            form_cache.attrs_json = json_line.clone();
            // Save data to cache.
            form_store.insert(key, form_cache.clone());
            // Return result.
            return Ok(json_line);
        }
        Ok(form_cache.attrs_json.clone())
    }

    // Get Html Form of Model for page templates.
    // ---------------------------------------------------------------------------------------------
    fn form_html() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there.
        if form_cache.controls_html.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let controls: String = Self::to_html(&form_cache.meta.fields_name, map_widgets);
            let mut form_cache: FormCache = form_cache.clone();
            form_cache.controls_html = controls.clone();
            form_store.insert(key, form_cache.clone());
            return Ok(controls);
        }
        Ok(form_cache.controls_html.clone())
    }

    // Get cached Model data.
    // ---------------------------------------------------------------------------------------------
    fn get_cache_data_for_query(
    ) -> Result<(FormCache, mongodb::sync::Client), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Access to the cached model data.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Accessing cached MongoDB clients.
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock()?;
        // Add the `FormCache` structure to the cache for the current model if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Get model metadata from cache.
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client for current model.
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        // Return result
        Ok((form_cache.clone(), client_cache.clone()))
    }

    // Accepts json-line to update data, for dynamic widgets.
    // Hint: Used in conjunction with the admin panel.
    // Example (json_line): {"field_name":[["value","Title"]]}
    // or
    // r#"{
    //    "field_name":[["value","Title"]],
    //    "field_name_2":[["value","Title 2"]],
    //    "field_name_3":[["value","Title 3"]],
    // }"#
    // ---------------------------------------------------------------------------------------------
    fn db_update_dyn_widgets(json_line: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        let mango_tech_keyword = format!("mango_tech__{}", meta.keyword.clone());
        let mango_tech_db = client_cache.database(&mango_tech_keyword);
        let collection = mango_tech_db.collection("dynamic_widgets");
        let query = mongodb::bson::doc! {
            "database": meta.database_name.clone(),
            "collection": meta.collection_name.clone()
        };
        let json_data: serde_json::Value = serde_json::from_str(json_line)?;
        let bson_data = serde_json::from_value::<mongodb::bson::Bson>(json_data)?;
        let new_doc = mongodb::bson::doc! {
            "fields": bson_data
        };
        let update: mongodb::bson::document::Document = mongodb::bson::doc! {
            "$set": mongodb::bson::Bson::Document(new_doc),
        };
        collection.update_one(query, update, None)?;

        Ok(())
    }
}
