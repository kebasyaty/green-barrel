//! # Models
//!
//! `Meta` - Metadata of models.
//! `Model` - Defining common behavior of models.
//! `run_monitor` - Creating and updating a database for monitoring the state of models.

use crate::widgets::{Transport, Widget};
use async_trait::async_trait;
use mongodb::Client;
use std::collections::HashMap;

// MODELS ==========================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}
/// Model ------------------------------------------------------------------------------------------
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
/// For Migration ----------------------------------------------------------------------------------
/// Creating and refreshin a database for monitoring the state of models
pub struct Monitor<'a> {
    pub password: &'static str,
    pub _client: &'a Client,
}
impl<'a> Monitor<'a> {
    // Refresh models state
    pub async fn refresh(&self) {
        println!("{}", self.password);
    }
    // Reorganize database state
    pub async fn run(&self) {
        println!("{}", self.password);
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
