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
        let key: String = Self::model_key();
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
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        Ok(form_cache.unwrap().map_widgets.clone())
    }

    // Get Form attributes in Json format for page templates.
    // ---------------------------------------------------------------------------------------------
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::model_key();
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
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
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
        let key: String = Self::model_key();
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
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
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
        let key: String = Self::model_key();
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add the `FormCache` structure to the cache for the current model if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Get model metadata from cache.
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client for current model.
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock()?;
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        Ok((form_cache.clone(), client_cache.clone()))
    }
}
