//! Global Store.

use async_lock::Mutex;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

use crate::models::helpers::Meta;

lazy_static! {
    // Metadata caching for Models.
    pub static ref META_STORE: Mutex<HashMap<String, Meta>> = {
        Mutex::new(HashMap::new())
    };
    // Regular expression caching.
    pub static ref VALIDATE_COLOR_CODE: Regex = RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$").case_insensitive(true).build().unwrap();
}
