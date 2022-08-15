//! Structures

use serde::{Deserialize, Serialize};

/// Metadata ( model parameters )
// -------------------------------------------------------------------------------------------------
#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub model_name: String,
    pub project_name: String,
    pub unique_project_key: String,
    pub service_name: String,
    pub database_name: String,
    pub db_client_name: String,
    pub db_query_docs_limit: u32,
    pub collection_name: String,
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_docs: bool,
    pub is_up_docs: bool,
    pub is_del_docs: bool,
    // <field_name, field_value_type>
    pub field_value_type_map: std::collections::HashMap<String, String>,
    // <field_name, field_type>
    pub field_type_map: std::collections::HashMap<String, String>,
    // <field_name, (_type, value)>
    pub default_value_map: std::collections::HashMap<String, (String, String)>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
            project_name: String::new(),
            unique_project_key: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_client_name: String::new(),
            db_query_docs_limit: 0_u32,
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_docs: true,
            is_up_docs: true,
            is_del_docs: true,
            field_value_type_map: std::collections::HashMap::new(),
            field_type_map: std::collections::HashMap::new(),
            default_value_map: std::collections::HashMap::new(),
            ignore_fields: Vec::new(),
        }
    }
}

/// Helper structures for inputFile fields type.
// -------------------------------------------------------------------------------------------------
#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FileData {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // bytes
}

/// Helper structures for inputImage fields type.
// -------------------------------------------------------------------------------------------------
#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ImageData {
    #[serde(default)]
    pub path: String, // max size == original
    #[serde(default)]
    pub path_xs: String,
    #[serde(default)]
    pub path_sm: String,
    #[serde(default)]
    pub path_md: String,
    #[serde(default)]
    pub path_lg: String,
    #[serde(default)]
    pub url: String, // max size == original
    #[serde(default)]
    pub url_xs: String,
    #[serde(default)]
    pub url_sm: String,
    #[serde(default)]
    pub url_md: String,
    #[serde(default)]
    pub url_lg: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // bytes
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub width: u32, // pixels
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub height: u32, // pixels
}
