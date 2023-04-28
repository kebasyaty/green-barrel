//! Global store of metadata for models.

use async_lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::models::helpers::Meta;

// Store
lazy_static! {
    // Metadata for Models
    pub static ref META_STORE: Mutex<HashMap<String, Meta>> = {
        Mutex::new(HashMap::new())
    };
}
