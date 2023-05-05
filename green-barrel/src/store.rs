//! Global Store.

use async_lock::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::models::helpers::Meta;

lazy_static! {
    // Metadata caching for Models.
    pub static ref METADATA: Mutex<HashMap<String, Meta>> = {
        Mutex::new(HashMap::new())
    };

}
