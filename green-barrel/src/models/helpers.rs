//! Collection of auxiliary Structures, Enumerations.

use async_lock::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Metadata ( model parameters )
// -------------------------------------------------------------------------------------------------
#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub model_name: String,
    pub project_name: String,
    pub unique_project_key: String,
    pub service_name: String,
    pub database_name: String,
    pub db_query_docs_limit: u32,
    pub collection_name: String, // Field type map
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_docs: bool,
    pub is_up_docs: bool,
    pub is_del_docs: bool,
    pub is_use_add_valid: bool,
    pub is_use_hooks: bool,
    pub is_use_hash_slug: bool,
    // <field_name, field_value_type>
    pub field_value_type_map: HashMap<String, String>,
    // <field_name, fields_type>
    pub field_type_map: HashMap<String, String>,
    // <field_name, default_value>
    pub default_value_map: HashMap<String, Value>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
    // Option maps for fields type `select` - <field_name, options>
    pub option_str_map: HashMap<String, Vec<String>>,
    pub option_i32_map: HashMap<String, Vec<i32>>,
    pub option_i64_map: HashMap<String, Vec<i64>>,
    pub option_f64_map: HashMap<String, Vec<f64>>,
    pub model_json: Value,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            model_name: String::new(),
            project_name: String::new(),
            unique_project_key: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_query_docs_limit: 0_u32,
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_docs: true,
            is_up_docs: true,
            is_del_docs: true,
            is_use_add_valid: false,
            is_use_hooks: false,
            is_use_hash_slug: false,
            field_value_type_map: HashMap::new(),
            field_type_map: HashMap::new(),
            default_value_map: HashMap::new(),
            ignore_fields: Vec::new(),
            option_str_map: HashMap::new(),
            option_i32_map: HashMap::new(),
            option_i64_map: HashMap::new(),
            option_f64_map: HashMap::new(),
            model_json: json!(null),
        }
    }
}

/// Helper structures for inputFile fields type.
// -------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct FileData {
    pub path: String,
    pub url: String,
    pub name: String,
    pub size: f64, // bytes
    pub is_delete: bool,
}

/// Helper structures for inputImage fields type.
// -------------------------------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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
    pub size: f64,   // bytes
    pub width: f64,  // pixels
    pub height: f64, // pixels
    pub is_delete: bool,
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

// Metadata storage for Models.
// -------------------------------------------------------------------------------------------------
pub fn get_meta_store() -> RwLock<HashMap<String, Meta>> {
    RwLock::new(HashMap::<String, Meta>::new())
}
