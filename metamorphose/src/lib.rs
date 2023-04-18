//! # Macro
//! **For converting structure to green-barrel model.**
//!
//! ## Usage:
//!
//! [Basic Example](https://github.com/kebasyaty/green-barrel/tree/master/examples/basic "Basic Example")
//!
//! ## Model parameters
//!
//! **_( all parameters are optional )_**
//!
//! | Parameter:          | Default:     | Description:                                                                                         |
//! | :------------------ | :----------- | :--------------------------------------------------------------------------------------------------- |
//! | db_query_docs_limit | 1000         | limiting query results.                                                                              |
//! | is_add_doc          | true         | Create documents in the database. **false** - Alternatively, use it to validate data from web forms. |
//! | is_up_doc           | true         | Update documents in the database.                                                                    |
//! | is_del_doc          | true         | Delete documents from the database.                                                                  |
//! | ignore_fields       | empty string | Fields that are not included in the database (separated by commas).                                  |
//! | is_use_add_valid    | false        | Allows additional validation - **impl AdditionalValidation for ModelName**.                          |
//! | is_use_hooks        | false        | Allows hooks methods - **impl Hooks for ModelName**.                                                 |
//!

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::{
    parse2, parse_macro_input, AttributeArgs, Data::Struct, DeriveInput, Fields::Named, NestedMeta,
    Type::Path,
};

// MODEL - MACRO FOR CONVERTING STRUCTURE TO GREEN-BARREL MODEL
// #################################################################################################
/// Macro for converting Structure to green-barrel Model.
/// The model can access the database.
/// The model can create, update, and delete documents in collections.

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Model(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut ast = parse_macro_input!(input as DeriveInput);
    impl_create_model(&args, &mut ast)
}

// Parsing fields and attributes of a structure, creating implementation of methods.
// *************************************************************************************************
fn impl_create_model(args: &Vec<NestedMeta>, ast: &mut DeriveInput) -> TokenStream {
    let model_name_ident = &ast.ident;
    let model_name_str = model_name_ident.to_string();
    //
    if model_name_str.len() > 30 {
        panic!("Model: `{model_name_str}` => Model name: Max size 30 characters.")
    }
    //
    let mut trans_meta = Meta {
        model_name: model_name_str.clone(),
        ..Default::default()
    };
    //
    let mut html_id_map = std::collections::HashMap::<String, String>::new();
    //
    let mut add_trait_custom_valid = quote! {impl AdditionalValidation for #model_name_ident {}};
    let mut add_trait_hooks = quote! {impl Hooks for #model_name_ident {}};
    let add_trait_fixtures = quote! {impl Fixtures for #model_name_ident {}};

    // Get Model attributes.
    // *********************************************************************************************
    for nested_meta in args {
        if let NestedMeta::Meta(meta) = nested_meta {
            if let syn::Meta::NameValue(mnv) = meta {
                if mnv.path.is_ident("database") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        trans_meta.database_name = lit_str.value().trim().to_string();
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `database`. Use the `&str` type."
                        )
                    }
                } else if mnv.path.is_ident("db_query_docs_limit") {
                    if let syn::Lit::Int(lit_int) = &mnv.lit {
                        trans_meta.db_query_docs_limit = lit_int.base10_parse::<u32>().unwrap();
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `db_query_docs_limit`. Use the `&str` type."
                        )
                    }
                } else if mnv.path.is_ident("is_add_doc") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_add_doc = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `is_add_docs`. Use the `bool` type."
                        )
                    }
                } else if mnv.path.is_ident("is_up_doc") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_up_doc = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `is_up_docs`. Use the `bool` type."
                        )
                    }
                } else if mnv.path.is_ident("is_del_doc") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_del_doc = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `is_del_docs`. Use the `bool` type."
                        )
                    }
                } else if mnv.path.is_ident("ignore_fields") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        let mut value = lit_str.value();
                        value.retain(|chr| !chr.is_whitespace());
                        trans_meta.ignore_fields = value
                            .to_lowercase()
                            .split(',')
                            .map(|item| item.to_string())
                            .collect();
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `ignore_fields`. Use the type `&str` in \
                            the format - <field_name, field_name>."
                        )
                    }
                } else if mnv.path.is_ident("is_use_add_valid") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_use_add_valid = lit_bool.value;
                        if lit_bool.value {
                            add_trait_custom_valid = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `is_use_add_valid`. Use the `bool` type."
                        )
                    }
                } else if mnv.path.is_ident("is_use_hooks") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_use_hooks = lit_bool.value;
                        if lit_bool.value {
                            add_trait_hooks = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{model_name_str}` => Could not determine value for \
                            parameter `is_use_hooks`. Use the `bool` type."
                        )
                    }
                } else {
                    panic!(
                        "Model: `{model_name_str}` => Invalid parameter! => \
                        Valid Parameters: database | db_client_name | db_query_docs_limit | \
                        is_add_doc | is_up_doc | is_del_doc | ignore_fields | \
                        is_use_add_valid | is_use_hooks"
                    )
                }
            } else {
                panic!("Model: `{model_name_str}` => syn::Meta::NameValue is missing.")
            }
        }
    }

    // Get fields of Model.
    // *********************************************************************************************
    if let Struct(ref mut data) = &mut ast.data {
        if let Named(ref mut fields) = &mut data.fields {
            let fields = &mut fields.named;

            //
            for field in fields.clone() {
                // Get field name.
                if let Some(ident) = &field.ident {
                    let field_name = ident.to_string();

                    // Check for fields with reserved names - 'hash', `created_at`, `updated_at`.
                    if field_name == "hash" {
                        panic!("Model: `{model_name_str}` => The field named `hash` is reserved.")
                    } else if field_name == "created_at" {
                        panic!(
                            "Model: `{model_name_str}` => The field named `created_at` is reserved."
                        )
                    } else if field_name == "updated_at" {
                        panic!(
                            "Model: `{model_name_str}` => The field named `updated_at` is reserved."
                        )
                    }
                }
            }

            // Add new field `hash`.
            let new_hash_field: syn::FieldsNamed = parse2(quote! {
                {pub hash: HiddenHash}
            })
            .unwrap_or_else(|err| panic!("{}", err.to_string()));
            let new_hash_field = new_hash_field.named.first().unwrap().to_owned();
            fields.push(new_hash_field);
            // Add new field `created_at`.
            let new_created_at_field: syn::FieldsNamed = parse2(quote! {
                {pub created_at: HiddenDateTime}
            })
            .unwrap_or_else(|err| panic!("{}", err.to_string()));
            let new_created_at_field = new_created_at_field.named.first().unwrap().to_owned();
            fields.push(new_created_at_field);
            // Add new field `updated_at`.
            let new_updated_at_field: syn::FieldsNamed = parse2(quote! {
                {pub updated_at: HiddenDateTime}
            })
            .unwrap_or_else(|err| panic!("{}", err.to_string()));
            let new_updated_at_field = new_updated_at_field.named.first().unwrap().to_owned();
            fields.push(new_updated_at_field);

            // Get the number of fields.
            trans_meta.fields_count = fields.len();

            // Loop over fields.
            // -------------------------------------------------------------------------------------
            for field in fields {
                let mut field_name = String::new();
                let mut field_type = String::new();
                //
                // Get field name.
                if let Some(ident) = &field.ident {
                    field_name = ident.to_string();
                    trans_meta.fields_name.push(field_name.clone());
                }
                // Add field name and field value type to map.
                if let Path(ty) = &field.ty {
                    field_type = {
                        let tmp_str = quote! {#ty}.to_string();
                        let tmp_vec = tmp_str.split("::").collect::<Vec<&str>>();
                        tmp_vec[tmp_vec.len() - 1].trim().to_string()
                    };
                    let field_info = get_field_info(
                        model_name_str.as_str(),
                        field_name.as_str(),
                        field_type.as_str(),
                    )
                    .unwrap();
                    trans_meta
                        .field_value_type_map
                        .insert(field_name.clone(), field_info.0.to_string());
                }
                // Add field name and Widget name to map.
                trans_meta
                    .field_type_map
                    .insert(field_name.clone(), field_type);
                //
                // Add field name and Widget html id to map.
                html_id_map.insert(
                    field_name.clone(),
                    get_html_id(model_name_str.as_str(), field_name.as_str()),
                );
                //
                // Delete field attributes.
                // ( To avoid conflicts with the compiler )
                field.attrs = Vec::new();
            }
        } else {
            panic!("Model: `{model_name_str}` => Expected a struct with named fields.")
        }
    }

    // Post processing.
    // *********************************************************************************************
    // Checking the name of ignored fields.
    for field_name in trans_meta.ignore_fields.iter() {
        if !trans_meta.fields_name.contains(field_name) {
            panic!(
                "Model: `{model_name_str}` => Model does not have an ignored field named `{field_name}`."
            )
        }
    }
    // trans_meta to Json-line.
    let trans_meta_json = match serde_json::to_string(&trans_meta) {
        Ok(json_line) => json_line,
        Err(err) => panic!("Model: `{model_name_str}` => {0:?}", err),
    };
    // html_id_map to Json-line.
    let html_id_map_json = match serde_json::to_string(&html_id_map) {
        Ok(json_line) => json_line,
        Err(err) => panic!("Model: `{model_name_str}` => {0:?}", err),
    };

    // Implementation of methods.
    // *********************************************************************************************
    let output = quote! {
        #ast

        /// All methods that directly depend on the macro.
        // *****************************************************************************************
        impl Main for #model_name_ident {
            /// Get model key.
            /// Hint: To access data in the cache.
            // -------------------------------------------------------------------------------------
            fn key() -> Result<String, Box<dyn std::error::Error>> {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])")?;
                Ok(format!(
                    "{}__{}__{}",
                    SERVICE_NAME.trim(),
                    re.replace_all(stringify!(#model_name_ident), "_$upper_chr"),
                    UNIQUE_APP_KEY.trim().to_string()
                )
                .to_lowercase())
            }

            /// Get a new model instance with custom settings.
            // -------------------------------------------------------------------------------------
            fn custom_default_to_json_val() -> Result<serde_json::Value, Box<dyn std::error::Error>>
            where
                Self: serde::de::DeserializeOwned + Sized,
            {
                let mut instance_json_val = serde_json::to_value(Self::custom_default())?;
                let html_id_map =
                    serde_json::from_str::<std::collections::HashMap<&str, &str>>(&#html_id_map_json)?;
                for (field_name, id_name) in html_id_map {
                    // Check field attributes.
                    if let Some(required) = instance_json_val.get(field_name).unwrap().get("required") {
                        let is_required = required.as_bool().unwrap();
                        if is_required {
                            if (instance_json_val.get(field_name).unwrap().get("disabled").unwrap().as_bool().unwrap()
                            || instance_json_val.get(field_name).unwrap().get("readonly").unwrap().as_bool().unwrap()
                            || instance_json_val.get(field_name).unwrap().get("is_hide").unwrap().as_bool().unwrap()) {
                                //
                                Err(format!(
                                    "Field: `{}` => Attribute required=true incompatible with \
                                        disabled=true or readonly=true or is_hide=true.",
                                    field_name
                                ))?
                            }
                        }
                    }
                    // Thumbnails sorting and validation.
                    if let Some(arr) = instance_json_val.get(field_name).unwrap().get("thumbnails") {
                        let mut arr = serde_json::from_value::<Vec<(String, u32)>>(arr.clone())?;
                        arr.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        let valid_size_names: [&str; 4] = ["xs", "sm", "md", "lg"];
                        for size in arr.iter() {
                            if !valid_size_names.contains(&size.0.as_str()) {
                                Err(format!(
                                    "Field: `{}` => \
                                        Valid size names - `xs`, `sm`, `md`, `lg`.",
                                    field_name
                                ))?
                            }
                        }
                        *instance_json_val
                            .get_mut(field_name)
                            .unwrap()
                            .get_mut("thumbnails")
                            .unwrap() = serde_json::json!(arr);
                    }
                    // Forbid the use of the `value` field attribute.
                    if let Some(val) = instance_json_val.get(field_name).unwrap().get("value") {
                        if !val.is_null() {
                            Err(format!(
                                "Field: `{}` => \
                                    For default values, use the `default` field attribute.",
                                field_name
                            ))?
                        }
                    }
                    // For dynamic field types, the `options` parameter must be an empty vector.
                    if let Some(field_type) = instance_json_val.get(field_name).unwrap().get("field_type") {
                        if field_type.as_str().unwrap().contains("Dyn") {
                            if let Some(options) = instance_json_val.get(field_name).unwrap().get("options") {
                                if options.as_array().unwrap().len() > 0 {
                                    Err(format!(
                                        "Field: `{}` => \
                                            For dynamic field types, the `options` parameter must be an empty vector.",
                                        field_name
                                    ))?
                                }
                            }
                        }
                    }
                    // Add `id` and `name`
                    *instance_json_val
                        .get_mut(field_name)
                        .unwrap()
                        .get_mut("id")
                        .unwrap() = serde_json::json!(id_name);
                    *instance_json_val
                        .get_mut(field_name)
                        .unwrap()
                        .get_mut("name")
                        .unwrap() = serde_json::json!(field_name);
                }
                //
                Ok(instance_json_val)
            }

            /// Generate metadata of Model.
            // -------------------------------------------------------------------------------------
            fn generate_metadata() -> Result<Meta, Box<dyn std::error::Error>>
            where
                Self: serde::de::DeserializeOwned + Sized,
            {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
                let mut meta = serde_json::from_str::<Meta>(&#trans_meta_json)?;
                let service_name: String = SERVICE_NAME.trim().to_string();
                // Add project name.
                meta.app_name = APP_NAME.trim().to_string();
                // Add unique project key.
                meta.unique_app_key = UNIQUE_APP_KEY.trim().to_string();
                // Add service name.
                meta.service_name = service_name.clone();
                // Add database name.
                if meta.database_name.is_empty() {
                    meta.database_name = format!(
                        "{}__{}__{}",
                        meta.app_name,
                        DATABASE_NAME.trim().to_string(),
                        meta.unique_app_key);
                }
                // Add a limit on the number of documents when querying the database.
                if meta.db_query_docs_limit == 0 {
                    meta.db_query_docs_limit = DB_QUERY_DOCS_LIMIT;
                }
                // Add collection name.
                meta.collection_name = format!(
                    "{}_{}",
                    service_name,
                    re.replace_all(&meta.model_name[..], "_$upper_chr")
                )
                .to_lowercase();
                // Add default_value_map
                let mut default_value_map = std::collections::HashMap::<String, serde_json::Value>::new();
                let model_json = Self::custom_default_to_json_val()?;
                for (field_name, field_type) in meta.field_type_map.iter() {
                    let default = if let Some(val) = model_json.get(field_name).unwrap().get("default") {
                            val.clone()
                    } else if let Some(val) = model_json.get(field_name).unwrap().get("checked") {
                            val.clone()
                    } else {
                        serde_json::json!(null)
                    };
                    default_value_map.insert(field_name.to_string(), default);
                    // Determine if there are fields of type AutoSlug and if they use a hash field as a source.
                    if !meta.is_use_hash_slug && field_type == "AutoSlug" {
                        let flag = model_json
                            .get(field_name)
                            .unwrap()
                            .get("slug_sources")
                            .unwrap()
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|item| item.as_str().unwrap())
                            .any(|item| item == "hash");
                            if flag {
                                meta.is_use_hash_slug = flag;
                            }
                    }
                }
                meta.default_value_map = default_value_map;
                meta.model_json = model_json;
                //
                Ok(meta)
            }

            /// Getter and Setter for field `hash`.
            // -------------------------------------------------------------------------------------
            fn hash(&self) -> String {
                self.hash.value.clone().unwrap_or_default()
            }
            fn set_hash(&mut self, value: String) {
                self.hash.value = Some(value);
            }

            /// ObjectId from hash field.
            // -------------------------------------------------------------------------------------
            fn obj_id(&self) -> Result<Option<mongodb::bson::oid::ObjectId>, Box<dyn std::error::Error>> {
                let hash = self.hash.value.clone().unwrap_or_default();
                if let Ok(obj_id) = mongodb::bson::oid::ObjectId::parse_str(hash.as_str()) {
                    return Ok(Some(obj_id));
                }
                Ok(None)
            }

            /// ObjectId to hash field.
            // -------------------------------------------------------------------------------------
            fn set_obj_id(&mut self, obj_id: mongodb::bson::oid::ObjectId) {
                self.hash.value = Some(obj_id.to_hex());
            }

            /// Getter and Setter for field `created_at`.
            // -------------------------------------------------------------------------------------
            fn created_at(&self) -> String {
                self.created_at.value.clone().unwrap_or_default()
            }
            fn set_created_at(&mut self, value: String) {
                self.created_at.value = Some(value);
            }

            /// Getter and Setter for field `updated_at`.
            /// ------------------------------------------------------------------------------------
            fn updated_at(&self) -> String {
                self.updated_at.value.clone().unwrap_or_default()
            }
            fn set_updated_at(&mut self, value: String) {
                self.updated_at.value = Some(value);
            }

            /// Serializing the model instance to serde_json::Value format.
            // -------------------------------------------------------------------------------------
            fn self_to_json_val(&self)
                -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                Ok(serde_json::to_value(self)?)
            }
        }

        /// A set of methods for custom validation.
        // *****************************************************************************************
        #add_trait_custom_valid

        /// Methods that are called at different stages when accessing the database.
        /// ****************************************************************************************
        #add_trait_hooks

         /// To populate the database with pre-created data.
        // *****************************************************************************************
        #add_trait_fixtures

        /// Caching information about Models for speed up work.
        // *****************************************************************************************
        impl Caching for #model_name_ident {}

        /// Validating Model fields for save and update.
        // *****************************************************************************************
        impl Validation for #model_name_ident {}

        /// Database Query API
        // *****************************************************************************************
        /// Output data converters for database queries.
        impl Converters for #model_name_ident {}
        /// Common database query methods.
        impl QCommons for #model_name_ident {}
        /// Query methods for a Model instance.
        impl QPaladins for #model_name_ident {}

    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(output)
}

// AUXILIARY STRUCTURES AND FUNCTIONS
// #################################################################################################
/// Transporting of metadate to implementation of methods.
// *************************************************************************************************
#[derive(Serialize)]
struct Meta {
    pub model_name: String,
    pub app_name: String,
    pub unique_app_key: String,
    pub service_name: String,
    pub database_name: String,
    pub db_query_docs_limit: u32,
    pub collection_name: String, // Field type map
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_doc: bool,
    pub is_up_doc: bool,
    pub is_del_doc: bool,
    pub is_use_add_valid: bool,
    pub is_use_hooks: bool,
    pub is_use_hash_slug: bool,
    // <field_name, field_value_type>
    pub field_value_type_map: std::collections::HashMap<String, String>,
    // <field_name, field_type>
    pub field_type_map: std::collections::HashMap<String, String>,
    // <field_name, default_value>
    pub default_value_map: std::collections::HashMap<String, serde_json::Value>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
    // Option maps for fields type `select` - <field_name, options>
    pub option_str_map: std::collections::HashMap<String, Vec<String>>,
    pub option_i32_map: std::collections::HashMap<String, Vec<i32>>,
    pub option_i64_map: std::collections::HashMap<String, Vec<i64>>,
    pub option_f64_map: std::collections::HashMap<String, Vec<f64>>,
    pub model_json: serde_json::Value,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
            app_name: String::new(),
            unique_app_key: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_query_docs_limit: 0_u32,
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_doc: true,
            is_up_doc: true,
            is_del_doc: true,
            is_use_add_valid: false,
            is_use_hooks: false,
            is_use_hash_slug: false,
            field_value_type_map: std::collections::HashMap::new(),
            field_type_map: std::collections::HashMap::new(),
            default_value_map: std::collections::HashMap::new(),
            ignore_fields: Vec::new(),
            option_str_map: std::collections::HashMap::new(),
            option_i32_map: std::collections::HashMap::new(),
            option_i64_map: std::collections::HashMap::new(),
            option_f64_map: std::collections::HashMap::new(),
            model_json: serde_json::json!(null),
        }
    }
}

/// Get field info.
// *************************************************************************************************
fn get_field_info<'a>(
    model_name: &'a str,
    field_name: &'a str,
    field_type: &'a str,
) -> Result<(&'a str, &'a str), Box<dyn std::error::Error>> {
    let info: (&'a str, &'a str) = match field_type {
        "CheckBox" => ("bool", "checkbox"),
        "InputColor" => ("String", "color"),
        "InputDate" => ("String", "date"),
        "InputDateTime" => ("String", "datetime"),
        "InputEmail" => ("String", "email"),
        "InputFile" => ("String", "file"),
        "InputImage" => ("String", "file"),
        "NumberI32" => ("i32", "number"),
        "NumberU32" => ("u32", "number"),
        "NumberI64" => ("i64", "number"),
        "NumberF64" => ("f64", "number"),
        "InputPassword" => ("String", "password"),
        "RadioText" => ("String", "radio"),
        "RadioI32" => ("i32", "radio"),
        "RadioU32" => ("u32", "radio"),
        "RadioI64" => ("i64", "radio"),
        "RadioF64" => ("f64", "radio"),
        "RangeI32" => ("i32", "range"),
        "RangeU32" => ("u32", "range"),
        "RangeI64" => ("i64", "range"),
        "RangeF64" => ("f64", "range"),
        "InputPhone" => ("String", "tel"),
        "InputText" => ("String", "text"),
        "Slug" => ("String", "text"),
        "InputUrl" => ("String", "url"),
        "InputIP" => ("String", "text"),
        "InputIPv4" => ("String", "text"),
        "InputIPv6" => ("String", "text"),
        "TextArea" => ("String", "textarea"),
        "SelectText" => ("String", "select"),
        "SelectTextDyn" => ("String", "select"),
        "SelectTextMult" => ("Vec<String>", "select"),
        "SelectTextMultDyn" => ("Vec<String>", "select"),
        "SelectI32" => ("i32", "select"),
        "SelectI32Dyn" => ("i32", "select"),
        "SelectI32Mult" => ("Vec<i32>", "select"),
        "SelectI32MultDyn" => ("Vec<i32>", "select"),
        "SelectU32" => ("u32", "select"),
        "SelectU32Dyn" => ("u32", "select"),
        "SelectU32Mult" => ("Vec<u32>", "select"),
        "SelectU32MultDyn" => ("Vec<u32>", "select"),
        "SelectI64" => ("i64", "select"),
        "SelectI64Dyn" => ("i64", "select"),
        "SelectI64Mult" => ("Vec<i64>", "select"),
        "SelectI64MultDyn" => ("Vec<i64>", "select"),
        "SelectF64" => ("f64", "select"),
        "SelectF64Dyn" => ("f64", "select"),
        "SelectF64Mult" => ("Vec<f64>", "select"),
        "SelectF64MultDyn" => ("Vec<f64>", "select"),
        "HiddenHash" => ("String", "text"),
        "HiddenDateTime" => ("String", "datetime"),
        _ => Err(format!(
            "Model: `{model_name}` > Field: `{field_name}` > Field type: `{field_type}` => \
            Invalid field type."
        ))?,
    };
    //
    Ok(info)
}

/// Get Html-ID for Field.
// *************************************************************************************************
fn get_html_id<'a>(model_name: &'a str, field_name: &'a str) -> String {
    let field_name_upper = field_name
        .split('_')
        .map(|word| word[0..1].to_uppercase() + &word[1..])
        .collect::<Vec<String>>()
        .join("");
    format!("{model_name}-{field_name_upper}")
}
