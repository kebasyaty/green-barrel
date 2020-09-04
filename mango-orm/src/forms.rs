//! # Forms
//!
//! Abstract Form methods for interacting with the database.

use crate::widgets::{Transport, Widget};
use std::collections::HashMap;

// FORMS ===========================================================================================
/// Abstract Form
pub trait Form {
    // Get raw attributes for further processing
    fn raw_attrs(&self) -> HashMap<&'static str, Widget>;
    // Get pure attributes for a page templating engine
    fn form_attrs(&self) -> HashMap<String, Transport> {
        let raw_attrs = self.raw_attrs();
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
