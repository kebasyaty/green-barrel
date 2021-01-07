//! # Caching.
//! Caching information about Forms for speed up work.
//!
//! Trait:
//! `Caching` - Methods caching information about Forms for speed up work.
//! Methods:
//! `form_wig` - Get an widgets map for page template.
//! `form_json` - Get Form attributes in Json format for page templates.
//! `form_html` - Get Html Form of Model for page templates.
//!

use crate::{
    forms::{html_controls::HtmlControls, ToForm, Widget},
    store::{FormCache, FORM_CACHE},
};

// Caching information about Forms for speed up work.
// *************************************************************************************************
pub trait CachingForm: ToForm + HtmlControls {
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
            // Create `FormCache` default and add map of widgets.
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
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
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets.
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
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
            // Save data to cache
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
            // Create `FormCache` default and add map of widgets.
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
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
            let controls: String = Self::to_html(&Self::fields_name()?, map_widgets);
            let mut form_cache: FormCache = form_cache.clone();
            form_cache.controls_html = controls.clone();
            form_store.insert(key, form_cache.clone());
            return Ok(controls);
        }
        Ok(form_cache.controls_html.clone())
    }

    // Get cached Form data.
    // ---------------------------------------------------------------------------------------------
    fn get_cache_data() -> Result<FormCache, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add the `FormCache` structure to the cache for the current Form if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets.
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        Ok(form_cache.clone())
    }
}
