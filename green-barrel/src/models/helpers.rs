//! Collection of auxiliary Structures, Enumerations.

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
    // <field_name, default_value>
    pub default_value_map: std::collections::HashMap<String, serde_json::Value>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
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
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FileData {
    pub path: String,
    pub url: String,
    pub name: String,
    pub size: u32, // bytes
    pub is_delete: bool,
}

impl Default for FileData {
    fn default() -> Self {
        Self {
            path: String::new(),
            url: String::new(),
            name: String::new(),
            size: 0,
            is_delete: false,
        }
    }
}

/// Helper structures for inputImage fields type.
// -------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ImageData {
    pub path: String, // max size == original
    pub path_xs: String,
    pub path_sm: String,
    pub path_md: String,
    pub path_lg: String,
    pub url: String, // max size == original
    pub url_xs: String,
    pub url_sm: String,
    pub url_md: String,
    pub url_lg: String,
    pub name: String,
    pub size: u32,   // bytes
    pub width: u32,  // pixels
    pub height: u32, // pixels
    pub is_delete: bool,
}

impl Default for ImageData {
    fn default() -> Self {
        Self {
            path: String::new(),
            path_xs: String::new(),
            path_sm: String::new(),
            path_md: String::new(),
            path_lg: String::new(),
            url: String::new(),
            url_xs: String::new(),
            url_sm: String::new(),
            url_md: String::new(),
            url_lg: String::new(),
            name: String::new(),
            size: 0,
            width: 0,
            height: 0,
            is_delete: false,
        }
    }
}

/// To optimize the update_dyn_wig method.
// -------------------------------------------------------------------------------------------------
pub enum ControlArr<'a> {
    Text(Vec<&'a str>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F64(Vec<f64>),
}
impl<'a> ControlArr<'a> {
    pub fn control_arr_str(&self) -> &Vec<&'a str> {
        match self {
            Self::Text(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_i32(&self) -> &Vec<i32> {
        match self {
            Self::I32(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_i64(&self) -> &Vec<i64> {
        match self {
            Self::I64(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_f64(&self) -> &Vec<f64> {
        match self {
            Self::F64(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
}
