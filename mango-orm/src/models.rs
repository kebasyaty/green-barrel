//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use crate::widgets::Transport;
use async_mutex::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

// FOR MODELS
// #################################################################################################
/// Global storage
/// ************************************************************************************************
/// For caching Form (map, json) attributes and Html
#[derive(Default, Clone, Debug)]
pub struct FormCache {
    pub form_map_attrs: HashMap<String, Transport>,
    pub form_json_attrs: String,
    pub form_html: String,
    pub map_widget_type: HashMap<String, &'static str>,
}

// Store
lazy_static! {
    // FORM_CACHE - For caching Form (map, json) attributes and Html
    pub static ref FORM_CACHE: Mutex<HashMap<&'static str, FormCache>> = {
        let mut _map = HashMap::new();
        Mutex::new(_map)
    };
}

/// Metadata
/// ************************************************************************************************
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
