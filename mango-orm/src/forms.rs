//! # Forms
//!
//! Abstract Form methods for interacting with the database.

use crate::widgets::{Transport, Widget};
use std::collections::HashMap;

// FORMS ===========================================================================================
/// Abstract Form
pub trait Form {
    fn raw_attrs(&self) -> HashMap<&'static str, Widget>;
    fn form_attrs(&self) -> HashMap<String, Transport>;
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
