//! # Macro
//! For converting structure to green-barrel model.

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::{
    parse2, parse_macro_input, Attribute, AttributeArgs,
    Data::Struct,
    DeriveInput,
    Fields::Named,
    Lit::{Bool, Float, Int, Str},
    Meta::{List, NameValue},
    MetaNameValue, NestedMeta,
    Type::Path,
};

// MODEL - MACRO FOR CONVERTING STRUCTURE TO GREEN-BARREL MODEL
// #################################################################################################
/// Macro for converting Structure to green-barrel Model.
/// The model can access the database.
/// The model can create, update, and delete documents in collections.
///
/// # Example:
///
/// ```
/// use mango_orm::*;
/// use metamorphose::Model;
/// use serde::{Deserialize, Serialize};
///
/// // Get settings of service/sub-application.
/// use crate::settings::{
///     default::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
///     PROJECT_NAME, UNIQUE_PROJECT_KEY,
/// };
///
/// #[Model(
///     is_del_docs = false,
///     ignore_fields = "confirm_password"
/// )]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct AdminProfile {
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputText",
///        label = "Username",
///        placeholder = "Enter your username",
///        unique = true,
///        required = true,
///        maxlength = 150,
///        hint = "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150"
///    )]
///    pub username: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputSlug",
///        label = "Slug",
///        unique = true,
///        readonly = true,
///        is_hide = true,
///        hint = "To create a human readable url",
///        slug_sources = r#"["hash", "username"]"#
///    )]
///    pub slug: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputText",
///        label = "First name",
///        placeholder = "Enter your First name",
///        maxlength = 150
///    )]
///    pub first_name: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputText",
///        label = "Last name",
///        placeholder = "Enter your Last name",
///        maxlength = 150
///    )]
///    pub last_name: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputEmail",
///        label = "E-mail",
///        placeholder = "Please enter your email",
///        required = true,
///        unique = true,
///        maxlength = 320,
///        hint = "Your actual E-mail"
///    )]
///    pub email: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputPhone",
///        label = "Phone number",
///        placeholder = "Please enter your phone number",
///        unique = true,
///        maxlength = 30,
///        hint = "Your actual phone number"
///    )]
///    pub phone: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputPassword",
///        label = "Password",
///        placeholder = "Enter your password",
///        required = true,
///        minlength = 8,
///        hint = "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
///    )]
///    pub password: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "inputPassword",
///        label = "Confirm password",
///        placeholder = "Repeat your password",
///        required = true,
///        minlength = 8,
///        hint = "Repeat your password"
///    )]
///    pub confirm_password: Option<String>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "checkBox",
///        label = "is staff?",
///        hint = "User can access the admin site?"
///    )]
///    pub is_staff: Option<bool>,
///    //
///    #[serde(default)]
///    #[field_attrs(
///        widget = "checkBox",
///        label = "is active?",
///        hint = "Is this an active account?"
///    )]
///    pub is_active: Option<bool>,
/// }
/// ```
///
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
    let model_name = &ast.ident;
    if model_name.to_string().len() > 31 {
        panic!(
            "Model: `{:?}` => Model name - Max size: 31 characters.",
            model_name
        )
    }
    let mut trans_meta = Meta {
        model_name: ast.ident.to_string(),
        ..Default::default()
    };
    let mut add_trait_custom_valid = quote! {impl AdditionalValidation for #model_name {}};
    let mut add_trait_hooks = quote! {impl Hooks for #model_name {}};
    let mut add_trait_generate_html = quote! {impl GenerateHtml for #model_name {}};

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
                            "Model: `{:?}` => Could not determine value for \
                            parameter `database`. Use the `&str` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("db_client_name") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        trans_meta.db_client_name = lit_str.value().trim().to_string();
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `db_client_name`. Use the `&str` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("db_query_docs_limit") {
                    if let syn::Lit::Int(lit_int) = &mnv.lit {
                        trans_meta.db_query_docs_limit = lit_int.base10_parse::<u32>().unwrap();
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `db_query_docs_limit`. Use the `&str` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_add_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_add_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_add_docs`. Use the `bool` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_up_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_up_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_up_docs`. Use the `bool` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_del_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_del_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_del_docs`. Use the `bool` type.",
                            model_name
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
                            "Model: `{:?}` => Could not determine value for \
                            parameter `ignore_fields`. Use the type `&str` in \
                            the format - <field_name, field_name>.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_use_add_valid") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        if lit_bool.value {
                            add_trait_custom_valid = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_use_add_valid`. Use the `bool` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_use_hooks") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        if lit_bool.value {
                            add_trait_hooks = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_use_hooks`. Use the `bool` type.",
                            model_name
                        )
                    }
                } else if mnv.path.is_ident("is_use_custom_html") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        if lit_bool.value {
                            add_trait_generate_html = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{:?}` => Could not determine value for \
                            parameter `is_use_custom_html`. Use the `bool` type.",
                            model_name
                        )
                    }
                }
            } else {
                panic!(
                    "Model: `{:?}` => syn::Meta::NameValue is missing.",
                    model_name
                )
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
                        panic!(
                            "Model: `{:?}` => The field named `hash` is reserved.",
                            model_name
                        )
                    } else if field_name == "created_at" {
                        panic!(
                            "Model: `{:?}` => The field named `created_at` is reserved.",
                            model_name
                        )
                    } else if field_name == "updated_at" {
                        panic!(
                            "Model: `{:?}` => The field named `updated_at` is reserved.",
                            model_name
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

                // Get field name.
                if let Some(ident) = &field.ident {
                    field_name = ident.to_string();
                    trans_meta.fields_name.push(field_name.clone());
                }
                // Get Widgets value type map.
                if let Path(ty) = &field.ty {
                    field_type = quote! {#ty}.to_string();
                    let field_type = get_widget_info(field_type.as_str()).unwrap();
                    trans_meta
                        .widget_value_type_map
                        .insert(field_name.clone(), field_type.0.to_string());
                }
                // Add field name and Widget name to the map.
                trans_meta
                    .widget_type_map
                    .insert(field_name.clone(), field_type);
                // Delete field attributes.
                // ( To avoid conflicts with the compiler )
                field.attrs = Vec::new();
            }
        } else {
            panic!(
                "Model: `{:?}` => Expected a struct with named fields.",
                model_name
            )
        }
    }

    // Post processing.
    // *********************************************************************************************
    // Checking the name of ignored fields.
    for field_name in trans_meta.ignore_fields.iter() {
        if !trans_meta.fields_name.contains(field_name) {
            panic!(
                "Model: `{:?}` => Model does not have an ignored field named `{}`.",
                model_name, field_name,
            )
        }
    }

    // trans_meta to Json-line.
    // ---------------------------------------------------------------------------------------------
    let trans_meta: String = match serde_json::to_string(&trans_meta) {
        Ok(json_string) => json_string,
        Err(err) => panic!("Model: `{:?}` => {}", model_name, err),
    };

    // Implementation of methods.
    // *********************************************************************************************
    let output = quote! {
        #ast

        /// All methods that directly depend on the macro.
        // *****************************************************************************************
        impl Main for #model_name {
            /// Get model key.
            /// Hint: To access data in the cache.
            // -------------------------------------------------------------------------------------
            fn key() -> Result<String, Box<dyn std::error::Error>> {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])")?;
                Ok(format!(
                    "{}__{}__{}",
                    SERVICE_NAME.trim(),
                    re.replace_all(stringify!(#model_name), "_$upper_chr"),
                    UNIQUE_PROJECT_KEY.trim().to_string()
                )
                .to_lowercase())
            }

            /// Get metadata of Model.
            // -------------------------------------------------------------------------------------
            fn meta() -> Result<Meta, Box<dyn std::error::Error>> {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
                let mut meta = serde_json::from_str::<Meta>(&#trans_meta)?;
                let service_name: String = SERVICE_NAME.trim().to_string();
                // Add project name.
                meta.project_name = PROJECT_NAME.trim().to_string();
                // Add unique project key.
                meta.unique_project_key = UNIQUE_PROJECT_KEY.trim().to_string();
                // Add service name.
                meta.service_name = service_name.clone();
                // Add database name.
                if meta.database_name.is_empty() {
                    meta.database_name = format!(
                        "{}__{}__{}",
                        meta.project_name,
                        DATABASE_NAME.trim().to_string(),
                        meta.unique_project_key);
                }
                // Add database client name.
                if meta.db_client_name.is_empty() {
                    meta.db_client_name = DB_CLIENT_NAME.trim().to_string();
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

                Ok(meta)
            }

            /// Getter and Setter for field `hash`.
            // -------------------------------------------------------------------------------------
            fn get_hash(&self) -> String {
                self.hash.clone().unwrap_or_default()
            }
            fn set_hash(&mut self, value: String) {
                self.hash = Some(value);
            }

            /// Getter and Setter for field `created_at`.
            // -------------------------------------------------------------------------------------
            fn get_created_at(&self) -> String {
                self.created_at.clone().unwrap_or_default()
            }
            fn set_created_at(&mut self, value: String) {
                self.created_at = Some(value);
            }

            /// Getter and Setter for field `updated_at`.
            /// ------------------------------------------------------------------------------------
            fn get_updated_at(&self) -> String {
                self.updated_at.clone().unwrap_or_default()
            }
            fn set_updated_at(&mut self, value: String) {
                self.updated_at = Some(value);
            }

            /// Serialize model to json-line.
            // -------------------------------------------------------------------------------------
            fn self_to_json(&self)
                -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
                Ok(serde_json::to_value(self)?)
            }
        }

        /// Rendering HTML-controls code for Form.
        // *****************************************************************************************
        #add_trait_generate_html

        /// A set of methods for custom validation.
        // *****************************************************************************************
        #add_trait_custom_valid

        /// Methods that are called at different stages when accessing the database.
        #add_trait_hooks

        /// Caching information about Models for speed up work.
        // *****************************************************************************************
        impl Caching for #model_name {}

        /// Validating Model fields for save and update.
        // *****************************************************************************************
        impl Validation for #model_name {}

        /// Database Query API
        // *****************************************************************************************
        /// Output data converters for database queries.
        impl Converters for #model_name {}
        /// Common database query methods.
        impl QCommons for #model_name {}
        /// Query methods for a Model instance.
        impl QPaladins for #model_name {}

        /// Helper methods for the admin panel.
        // *****************************************************************************************
        impl Administrator for #model_name {}

    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(output)
}

// AUXILIARY STRUCTURES AND FUNCTIONS
// #################################################################################################
/// Get ID for Widget.
// *************************************************************************************************
fn get_id(model_name: String, field_name: String) -> String {
    let field_name_upper = field_name
        .split('_')
        .map(|word| {
            let mut chr: Vec<char> = word.chars().collect();
            chr[0] = chr[0].to_uppercase().nth(0).unwrap();
            chr.into_iter().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("");
    format!("{}-{}", model_name, field_name_upper)
}

/// Transporting of metadate to implementation of methods.
// *************************************************************************************************
#[derive(Serialize)]
struct Meta {
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
    pub widget_value_type_map: std::collections::HashMap<String, String>,
    pub widget_type_map: std::collections::HashMap<String, String>,
    // <field_name, (widget_type, value)>
    pub default_value_map: std::collections::HashMap<String, (String, String)>,
    // List of field names that will not be saved to the database.
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
            widget_value_type_map: std::collections::HashMap::new(),
            widget_type_map: std::collections::HashMap::new(),
            default_value_map: std::collections::HashMap::new(),
            // List of field names that will not be saved to the database.
            ignore_fields: Vec::new(),
        }
    }
}

/// Get widget info.
// *************************************************************************************************
fn get_widget_info<'a>(
    widget_name: &'a str,
) -> Result<(&'a str, &'a str), Box<dyn std::error::Error>> {
    let info: (&'a str, &'a str) = match widget_name {
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
        "InputSlug" => ("String", "text"),
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
        "HiddenText" => ("String", "hidden"),
        "HiddenI32" => ("i32", "hidden"),
        "HiddenU32" => ("u32", "hidden"),
        "HiddenI64" => ("i64", "hidden"),
        "HiddenF64" => ("f64", "hidden"),
        _ => Err("Invalid widget type.")?,
    };
    //
    Ok(info)
}
