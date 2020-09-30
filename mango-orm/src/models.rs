//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use crate::widgets::Transport;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Cache {
    pub form_map_attrs: HashMap<String, Transport>,
    pub form_json_attrs: String,
    pub form_html: String,
}

lazy_static! {
    static ref CACHE: Mutex<HashMap<&'static str, HashMap<&'static str, HashMap<&'static str, Cache>>>> = {
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
