//! Global store of settings and metadata for models.

use lazy_static::lazy_static;
use mongodb::sync::Client;
use regex::{Regex, RegexBuilder};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::helpers::Meta;

// GLOBAL STORAGE
// #################################################################################################
/// Structure for caching map of fields type, json and html, for mango models.
#[derive(Default, Clone, Debug)]
pub struct ModelCache {
    pub meta: Meta,
    pub model_json: Value,
}

// Store
lazy_static! {
    // Storage of settings for mango models
    // ---------------------------------------------------------------------------------------------
    pub static ref MODEL_STORE: RwLock<HashMap<String, ModelCache>> = {
        RwLock::new(HashMap::new())
    };
    // Caching clients MongoDB
    // ---------------------------------------------------------------------------------------------
    pub static ref MONGODB_CLIENT_STORE: RwLock<HashMap<String, Client>> = {
        RwLock::new(HashMap::new())
    };
    // Regular expressions
    // ---------------------------------------------------------------------------------------------
    pub static ref REGEX_IS_COLOR_CODE: Regex = RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$").case_insensitive(true).build().unwrap();
    pub static ref REGEX_IS_PASSWORD: Regex = Regex::new(r"^[a-zA-Z0-9@#$%^&+=*!~)(]{8,256}$").unwrap();
    pub static ref REGEX_TOKEN_DATED_PATH: Regex = Regex::new(r"(?:(?:/|\\)\d{4}\-\d{2}\-\d{2}\-utc(?:/|\\))").unwrap();
}
