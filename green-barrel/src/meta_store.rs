//! Global store of metadata for models.

use async_lock::RwLock;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::models::helpers::Meta;

// Store
lazy_static! {
    // Storage of metadata
    pub static ref META_STORE: RwLock<HashMap<String, Meta>> = {
        RwLock::new(HashMap::new())
    };
}
