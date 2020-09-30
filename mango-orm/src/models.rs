//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref CACHE_MAP: Mutex<HashMap<&'static str, HashMap<&'static str, &'static str>>> = {
        let mut _cache = HashMap::new();
        Mutex::new(_cache)
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
