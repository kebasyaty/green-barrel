//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

use crate::widgets::{FieldType, Transport, Widget};
use async_trait::async_trait;
use mongodb::Client;
use std::collections::HashMap;

// MODELS ==========================================================================================
/// Metadata
pub struct Meta {
    pub database: &'static str,
    pub collection: &'static str,
}
/// Abstract Model ---------------------------------------------------------------------------------
#[async_trait]
pub trait Model {
    // Metadata (database name, collection name, etc)
    fn meta() -> Meta;
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
    // Checking Models and creating migrations to the Database.
    async fn migrat(_client: Client) {
        let _meta: Meta = Self::meta();
        let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
        let struct_name: &str = Self::struct_name();
        // Checking Widgets
        for (_field, widget) in attrs {
            match widget.field_type {
                FieldType::InputCheckBox => {
                    if widget.relation_model != String::new() {
                        panic!(
                            "{} FieldType `InputCheckBox` -> relation_model = blank string",
                            struct_name
                        )
                    }
                }
                _ => panic!("{} - Non-existent field type.", struct_name),
            }
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
