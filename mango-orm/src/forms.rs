//! # Forms
//!
//! Abstract Form methods for interacting with the database.

use crate::widgets::Widget;
use std::collections::HashMap;

// FORMS ===========================================================================================
/// Abstract Form
pub trait Form {
    fn form(&self) -> HashMap<&'static str, Widget>;
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
