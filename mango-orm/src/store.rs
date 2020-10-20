//! # Global storage
//!
//! `FormCache` - Structure for caching From (map, json) attributes and Html.

use crate::widgets::Transport;
use async_mutex::Mutex;
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;

// Global storage
// #################################################################################################
/// Structure for caching From (map, json) attributes and Html
#[derive(Default, Clone, Debug)]
pub struct FormCache {
    pub attrs_map: HashMap<String, Transport>,
    pub attrs_json: String,
    pub form_html: String,
    pub widget_map: HashMap<String, String>,
}

// Store
lazy_static! {
    // FORM_CACHE - For caching Form (map, json) attributes and Html
    // ---------------------------------------------------------------------------------------------
    pub static ref FORM_CACHE: Mutex<HashMap<String, FormCache>> = {
        let mut _map = HashMap::new();
        Mutex::new(_map)
    };
    // Regular expressions
    // ---------------------------------------------------------------------------------------------
    static ref REGEX_IS_PASSWORD: Regex = RegexBuilder::new(r"^[a-z0-9@#$%^&+=*!~)(]{8,}$").case_insensitive(true).build().unwrap();
    static ref REGEX_IS_COLOR_CODE: Regex = RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6})\b|(?:rgb|hsl)a?\([^\)]*\)$").case_insensitive(true).build().unwrap();
    static ref REGEX_IS_DATE: Regex = Regex::new(r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)$").unwrap();
    static ref REGEX_IS_DATETIME: Regex = Regex::new(r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d$").unwrap();
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
