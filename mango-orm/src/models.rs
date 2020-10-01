//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use crate::widgets::Transport;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// For caching Form (map, json) attributes and Html
pub struct FormCache {
    pub form_map_attrs: HashMap<String, Transport>,
    pub form_json_attrs: String,
    pub form_html: String,
}

// Global storage
lazy_static! {
    static ref FORM_CACHE: Mutex<HashMap<&'static str, FormCache>> = {
        let mut _map = HashMap::new();
        Mutex::new(_map)
    };
}

// MODELS
// =================================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}

// TESTS
// =================================================================================================
#[cfg(test)]
mod tests {
    //
}
