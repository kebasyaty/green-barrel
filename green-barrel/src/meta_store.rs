//! Global store of metadata for models.

use async_lock::RwLock;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

use crate::models::helpers::Meta;

// Store
lazy_static! {
    // Storage of metadata
    pub static ref META_STORE: RwLock<HashMap<String, Meta>> = {
        RwLock::new(HashMap::new())
    };

    // Storage of  regular expressions
    pub static ref REGEX_IS_COLOR_CODE: Regex = RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$").case_insensitive(true).build().unwrap();
    pub static ref REGEX_IS_PASSWORD: Regex = Regex::new(r"^[a-zA-Z0-9@#$%^&+=*!~)(]{8,256}$").unwrap();
    pub static ref REGEX_TOKEN_DATED_PATH: Regex = Regex::new(r"(?:(?:/|\\)\d{4}\-\d{2}\-\d{2}\-utc(?:/|\\))").unwrap();
}
