//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use crate::widgets::Transport;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Store {
    pub widgets: HashMap<&'static str, Transport>,
}

lazy_static! {
    static ref CACHE: Mutex<HashMap<&'static str, HashMap<&'static str, HashMap<&'static str, Store>>>> = {
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
