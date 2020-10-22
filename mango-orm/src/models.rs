//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

use std::collections::HashMap;
use std::error::Error;

// MODEL
// #################################################################################################
// Model settings
// *************************************************************************************************
// Metadata
#[derive(Default, Debug)]
pub struct Meta<'a> {
    pub service: String,
    pub database: String,
    pub collection: String,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<&'a str>,
}

// Model settings
// *************************************************************************************************
pub trait Model {
    // Metadata (database name, collection name, etc)
    // *********************************************************************************************
    fn meta<'a>() -> Result<Meta<'a>, Box<dyn Error>> {
        Ok(Meta {
            ..Default::default()
        })
    }

    // Custom validation of model fields
    // (Don't forget to check for ignored fields -> `ignore_fields()`)
    // *********************************************************************************************
    fn custom_check<'a>(&self) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
        // .insert("field_name", "Error message")
        let error_map = HashMap::new();
        Ok(error_map)
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
