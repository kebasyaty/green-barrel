//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

use crate::widgets::{Transport, Widget};
use async_trait::async_trait;
use std::collections::HashMap;

// MODELS ==========================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}
/// Abstract Model ---------------------------------------------------------------------------------
#[async_trait]
pub trait Model {
    // Get raw attributes for further processing
    fn raw_attrs() -> HashMap<&'static str, Widget>;
    // Get pure attributes for a page templating engine
    fn form_attrs() -> HashMap<String, Transport> {
        let raw_attrs = Self::raw_attrs();
        let mut clean_attrs = HashMap::new();
        for (field, widget) in &raw_attrs {
            clean_attrs.insert(field.to_string(), widget.get_clean_attrs(field));
        }
        clean_attrs
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
