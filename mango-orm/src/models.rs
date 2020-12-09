//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).
//! `ToModel` - Model options and widget map for Form.

use crate::{
    forms::{FileData, ImageData, OutputData, Widget},
    store::{
        FormCache, DB_MAP_CLIENT_NAMES, FORM_CACHE, REGEX_IS_COLOR_CODE, REGEX_IS_DATE,
        REGEX_IS_DATETIME, REGEX_IS_PASSWORD,
    },
};
use rand::Rng;

// MODEL
// #################################################################################################
// Metadata
// ( Model parameters )
// *************************************************************************************************
#[derive(serde::Deserialize, Clone, Debug)]
pub struct Meta {
    pub model_name: String,
    pub service_name: String,
    pub database_name: String,
    pub db_client_name: String,
    pub collection_name: String,
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_docs: bool,
    pub is_up_docs: bool,
    pub is_del_docs: bool,
    pub map_field_type: std::collections::HashMap<String, String>,
    pub map_widget_type: std::collections::HashMap<String, String>,
    // <field_name, (widget_type, value)>
    pub map_default_values: std::collections::HashMap<String, (String, String)>,
    pub map_related_models:
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    // List of field names that will not be saved to the database
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_client_name: String::new(),
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_docs: true,
            is_up_docs: true,
            is_del_docs: true,
            map_field_type: std::collections::HashMap::new(),
            map_widget_type: std::collections::HashMap::new(),
            map_default_values: std::collections::HashMap::new(),
            map_related_models: std::collections::HashMap::new(),
            // List of field names that will not be saved to the database
            ignore_fields: Vec::new(),
        }
    }
}

// Model options and widget map for Form
// *************************************************************************************************
pub trait ToModel {
    // Getter and Setter for field `hash`
    // ---------------------------------------------------------------------------------------------
    fn get_hash(&self) -> Option<String>;
    fn set_hash(&mut self, value: String);

    // Converting `Self` to Document
    // ---------------------------------------------------------------------------------------------
    fn self_to_json(&self) -> Result<serde_json::value::Value, Box<dyn std::error::Error>>;

    // // Get a key to access Model data in the cache
    // ( key = collection name, alternatively, not to call `meta()` )
    // ---------------------------------------------------------------------------------------------
    fn key_store() -> Result<String, Box<dyn std::error::Error>>;

    // Get metadata of Model
    // ---------------------------------------------------------------------------------------------
    fn meta() -> Result<Meta, Box<dyn std::error::Error>>;

    // Get map of widgets for model fields
    // <field name, Widget>
    // ---------------------------------------------------------------------------------------------
    fn widgets() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>>;

    // Custom validation of model fields.
    // ( Intermediary between `check()` and `AdditionalValidation::add_validation()` )
    // ---------------------------------------------------------------------------------------------
    fn medium_add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>>;

    // Form caching
    // *********************************************************************************************
    // Get an widgets map for page template
    // ---------------------------------------------------------------------------------------------
    fn form_wig() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache
        let key: String = Self::key_store()?;
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        Ok(form_cache.unwrap().map_widgets.clone())
    }

    // Get Form attributes in Json format for page templates
    // ---------------------------------------------------------------------------------------------
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache
        let key: String = Self::key_store()?;
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there
        if form_cache.attrs_json.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let mut json_text = String::new();
            // Create Json-string
            for (field_name, widget) in map_widgets {
                let tmp: String = serde_json::to_string(&widget)?;
                if !json_text.is_empty() {
                    json_text = format!("{},\"{}\":{}", json_text, field_name, tmp);
                } else {
                    json_text = format!("\"{}\":{}", field_name, tmp);
                }
            }
            let mut form_cache: FormCache = form_cache.clone();
            // Update data
            form_cache.attrs_json = format!("{{{}}}", json_text);
            // Save data to cache
            form_store.insert(key, form_cache.clone());
            // Return result
            return Ok(form_cache.attrs_json);
        }
        Ok(form_cache.attrs_json.clone())
    }

    // Get Html Form of Model for page templates
    // ---------------------------------------------------------------------------------------------
    fn form_html() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache
        let key: String = Self::key_store()?;
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there
        if form_cache.controls_html.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let controls: String = Self::medium_to_html(&form_cache.meta.fields_name, map_widgets)?;
            let mut form_cache: FormCache = form_cache.clone();
            form_cache.controls_html = controls.clone();
            form_store.insert(key, form_cache.clone());
            return Ok(controls);
        }
        Ok(form_cache.controls_html.clone())
    }

    // Validation of database queries
    // *********************************************************************************************
    // Validation of `minlength`
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(format!("Exceeds limit, minlength={}.", minlength))?
        }
        Ok(())
    }

    // Validation of `maxlength`
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(format!("Exceeds limit, maxlength={}.", maxlength))?
        }
        Ok(())
    }

    // Accumulation of errors
    // ---------------------------------------------------------------------------------------------
    fn accumula_err(widget: &Widget, err: &String) -> Result<String, Box<dyn std::error::Error>> {
        let mut tmp = widget.error.clone();
        tmp = if !tmp.is_empty() {
            format!("{}<br>", tmp)
        } else {
            String::new()
        };
        Ok(format!("{}{}", tmp, err))
    }

    // Validation in regular expression (email, password, etc...)
    // ---------------------------------------------------------------------------------------------
    fn regex_validation(field_type: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match field_type {
            "inputEmail" => {
                if !validator::validate_email(value) {
                    Err("Invalid email address.")?
                }
            }
            "inputColor" => {
                if !REGEX_IS_COLOR_CODE.is_match(value) {
                    Err("Invalid Color code.")?
                }
            }
            "inputUrl" => {
                if !validator::validate_url(value) {
                    Err("Invalid Url.")?
                }
            }
            "inputIP" => {
                if !validator::validate_ip(value) {
                    Err("Invalid IP address.")?
                }
            }
            "inputIPv4" => {
                if !validator::validate_ip_v4(value) {
                    Err("Invalid IPv4 address.")?
                }
            }
            "inputIPv6" => {
                if !validator::validate_ip_v6(value) {
                    Err("Invalid IPv6 address.")?
                }
            }
            "inputPassword" => {
                if !REGEX_IS_PASSWORD.is_match(value) {
                    Err(
                        "Allowed characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br> \
                                 Minimum size 8 characters",
                    )?
                }
            }
            "inputDate" => {
                if !REGEX_IS_DATE.is_match(value) {
                    Err("Incorrect date format.<br>Example: 1970-02-28")?
                }
            }
            "inputDateTime" => {
                if !REGEX_IS_DATETIME.is_match(value) {
                    Err("Incorrect date and time format.<br>Example: 1970-02-28T00:00")?
                }
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    // Validation of `unique`
    // ---------------------------------------------------------------------------------------------
    fn check_unique(
        hash: &str,
        field_name: &str,
        bson_field_value: &mongodb::bson::Bson,
        coll: &mongodb::sync::Collection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let object_id = mongodb::bson::oid::ObjectId::with_string(hash);
        let mut filter = mongodb::bson::doc! { field_name.to_string() : bson_field_value };
        if let Ok(id) = object_id {
            // If the document is will updated
            filter = mongodb::bson::doc! {
                "$and".to_string() : [
                    { "_id" : { "$ne".to_string() : id } },
                    filter
                ]
            };
        }
        let count: i64 = coll.count_documents(filter, None)?;
        if count > 0 {
            Err("Is not unique.")?
        }
        Ok(())
    }

    // Password
    // *********************************************************************************************
    // Generate password hash and add to result document
    // ---------------------------------------------------------------------------------------------
    fn create_password_hash(field_value: &str) -> Result<String, Box<dyn std::error::Error>> {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@#$%^&+=*!~)(";
        const SALT_LEN: usize = 12;
        let mut rng = rand::thread_rng();
        let password: &[u8] = field_value.as_bytes();
        let salt: String = (0..SALT_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let salt: &[u8] = salt.as_bytes();
        let config = argon2::Config::default();
        let hash: String = argon2::hash_encoded(password, salt, &config)?;
        Ok(hash)
    }

    // Post processing database queries
    // *********************************************************************************************
    // Get Hash-line
    // ---------------------------------------------------------------------------------------------
    fn to_hash(
        map_widgets: &std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut errors = String::new();
        for (field_name, widget) in map_widgets {
            let tmp = if !errors.is_empty() {
                format!("{} ; ", errors)
            } else {
                String::new()
            };
            if !widget.error.is_empty() {
                errors = format!("{}Field: `{}` - {}", tmp, field_name, widget.error);
            }
        }
        if !errors.is_empty() {
            Err(errors.replace("<br>", " | "))?
        }
        Ok(map_widgets.get(&"hash".to_owned()).unwrap().value.clone())
    }

    // Rendering HTML-controls code for Form
    // ( Intermediary between `check()` and `HtmlControls::to_html()` )
    // ---------------------------------------------------------------------------------------------
    fn medium_to_html(
        fields_name: &Vec<String>,
        map_widgets: std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>>;

    // Database Query API
    // *********************************************************************************************
    // Save to database as a new document or
    // update an existing document.
    // (Returns the hash-line of the identifier)
    // ---------------------------------------------------------------------------------------------
    fn check(&self) -> Result<OutputData, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache
        let key: String = Self::key_store()?;
        //
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add the `FormCache` structure to the cache for the current model if it is not there
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model
            let meta: Meta = Self::meta()?;
            let map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store
            form_store.insert(key.clone(), new_form_cache);
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Get model metadata from cache
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client for current model
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock()?;
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        // Get model name
        let model_name: &str = meta.model_name.as_str();
        // User input error detection symptom
        let mut is_err_symptom = false;
        // Determines the mode of accessing the database (insert or update)
        let hash = self.get_hash().unwrap_or_default();
        let hash = hash.as_str();
        let is_update: bool = !hash.is_empty();
        // Get a list of fields that should not be included in the document
        let ignore_fields: Vec<&str> = meta
            .ignore_fields
            .iter()
            .map(|item| item.as_str())
            .collect();
        // Access the collection
        let coll: mongodb::sync::Collection = client_cache
            .database(&meta.database_name)
            .collection(&meta.collection_name);
        // Get preliminary data from the model
        let pre_json: serde_json::value::Value = self.self_to_json()?;
        // Document for the final result
        let mut final_doc = mongodb::bson::document::Document::new();

        // Validation of field by attributes (maxlength, unique, min, max, etc...)
        // -----------------------------------------------------------------------------------------
        let fields_name: Vec<&str> = meta.fields_name.iter().map(|item| item.as_str()).collect();
        let mut final_map_widgets: std::collections::HashMap<String, Widget> =
            form_cache.map_widgets.clone();
        // Apply additional validation
        {
            let error_map = self.medium_add_validation()?;
            if !error_map.is_empty() {
                is_err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if !fields_name.contains(&field_name) {
                        Err(format!(
                            "Model: `{}` >  Method: `add_validation()` : \
                                            The `{}` field is missing from the model.",
                            model_name, field_name
                        ))?
                    }
                    if let Some(widget) = final_map_widgets.get_mut(&field_name.to_owned()) {
                        widget.error = Self::accumula_err(&widget, &err_msg.to_string())?;
                    }
                }
            }
        }
        // Loop over fields for validation
        for field_name in fields_name {
            // Don't check the `hash` field
            if field_name == "hash" {
                continue;
            }
            // Get field value for validation
            let pre_json_value: Option<&serde_json::value::Value> = pre_json.get(field_name);
            // Check field value
            if pre_json_value.is_none() {
                Err(format!(
                    "Model: `{}` > Field: `{}` > Method: `check()` : This field is missing.",
                    model_name, field_name
                ))?
            }
            //
            let pre_json_value: &serde_json::value::Value = pre_json_value.unwrap();
            let final_widget: &mut Widget = final_map_widgets.get_mut(field_name).unwrap();
            let widget_type: &str = &final_widget.widget.clone()[..];
            // Field validation
            match widget_type {
                // Validation of text type fields
                // ---------------------------------------------------------------------------------
                "checkBoxText" | "radioText" | "inputColor" | "inputEmail" | "inputPassword"
                | "inputPhone" | "inputText" | "inputUrl" | "inputIP" | "inputIPv4"
                | "inputIPv6" | "textArea" | "selectText" => {
                    // Get field value for validation
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        if widget_type != "inputPassword" {
                            final_widget.value = clean_data.clone();
                        } else {
                            final_widget.value = String::new();
                        }
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default
                            if !is_update && widget_type != "inputPassword" {
                                if !final_widget.value.is_empty() {
                                    field_value = final_widget.value.trim().to_string();
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    final_widget.value = String::new();
                                    continue;
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = if widget_type != "inputPassword" {
                        mongodb::bson::Bson::String(field_value.clone())
                    } else {
                        mongodb::bson::Bson::Null
                    };
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression
                    // Checking `minlength`, `maxlength`, `min length`, `max length`
                    // -----------------------------------------------------------------------------
                    Self::check_minlength(final_widget.minlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                        },
                    );
                    Self::check_maxlength(final_widget.maxlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                        },
                    );
                    // Validation of range (`min` <> `max`)
                    // ( Hint: The `validate_length()` method did not
                    // provide the desired result )
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.minlength.clone() as f64;
                    let max: f64 = final_widget.maxlength.clone() as f64;
                    let len: f64 = field_value.encode_utf16().count() as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            len,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Length {} is out of range (min={} <> max={}).",
                            len, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if widget_type != "inputPassword" && final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation in regular expression (email, password, etc...)
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        match widget_type {
                            "inputPassword" => {
                                if !is_update && !field_value.is_empty() {
                                    // Generate password hash and add to result document
                                    let hash: String = Self::create_password_hash(field_value)?;
                                    final_doc.insert(
                                        field_name.to_string(),
                                        mongodb::bson::Bson::String(hash),
                                    );
                                }
                            }
                            _ => {
                                // Insert result from other fields
                                final_doc.insert(field_name.to_string(), bson_field_value);
                            }
                        }
                    }
                }
                "inputFile" => {
                    // Get field value for validation
                    let mut field_value: FileData = if !pre_json_value.is_null() {
                        let clean_data: FileData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        FileData::default()
                    };
                    // Define flags to check
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Primary `FileData` validation
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        panic!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Check the `path` and `url` attributes in the `default` field parameter.",
                            model_name, field_name
                        );
                    }
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    if is_emty_path && is_emty_url {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default
                                if !final_widget.value.is_empty() {
                                    field_value = serde_json::from_str(final_widget.value.trim())?;
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    continue;
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                    // Create path for validation of file
                    let path: String = field_value.path.clone();
                    let f_path = std::path::Path::new(path.as_str());
                    if !f_path.exists() || !f_path.is_file() {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                                `check()` : File is missing - {}",
                            model_name, field_name, path
                        ))?
                    }
                    // Get file metadata
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes
                    field_value.size = metadata.len() as u32;
                    // Get file name
                    field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Insert result
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = mongodb::bson::ser::to_bson(&field_value.clone())?;
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "inputImage" => {
                    // Get field value for validation
                    let mut field_value: ImageData = if !pre_json_value.is_null() {
                        let clean_data: ImageData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        ImageData::default()
                    };
                    // Define flags to check
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Primary `FileData` validation
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        panic!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Check the `path` and `url` attributes in the `default` field parameter.",
                            model_name, field_name
                        );
                    }
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    if is_emty_path && is_emty_url {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default
                                if !final_widget.value.is_empty() {
                                    field_value = serde_json::from_str(final_widget.value.trim())?;
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    continue;
                                } else {
                                    continue;
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                    // Create path for validation of file
                    let path: String = field_value.path.clone();
                    let f_path = std::path::Path::new(path.as_str());
                    if !f_path.exists() || !f_path.is_file() {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                                `check()` : File is missing - {}",
                            model_name, field_name, path
                        ))?
                    }
                    // Get file metadata
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes
                    field_value.size = metadata.len() as u32;
                    // Get file name
                    field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Get image width and height
                    let dimensions: (u32, u32) = image::image_dimensions(path)?;
                    field_value.width = dimensions.0;
                    field_value.height = dimensions.1;
                    // Insert result
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = mongodb::bson::ser::to_bson(&field_value.clone())?;
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "inputDate" | "inputDateTime" => {
                    // Get field value for validation
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        final_widget.value = clean_data.clone();
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default
                                if !final_widget.value.is_empty() {
                                    field_value = final_widget.value.trim().to_string();
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    final_widget.value = String::new();
                                    continue;
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                    if is_err_symptom {
                        continue;
                    }
                    // Create Date and Time Object
                    // -----------------------------------------------------------------------------
                    // Date to DateTime
                    let dt_value: chrono::DateTime<chrono::Utc> = {
                        let field_value: String = if widget_type == "inputDate" {
                            format!("{}T00:00", field_value.to_string())
                        } else {
                            field_value.to_string()
                        };
                        chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::parse_from_str(&field_value, "%Y-%m-%dT%H:%M")?,
                            chrono::Utc,
                        )
                    };
                    // Create dates for `min` and `max` attributes values to
                    // check, if the value of user falls within the range
                    // between these dates
                    if final_widget.min != "0".to_string() && final_widget.max != "0".to_string() {
                        // Validation in regular expression (min)
                        Self::regex_validation(widget_type, final_widget.min.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        // Validation in regular expression (max)
                        Self::regex_validation(widget_type, final_widget.max.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        if is_err_symptom {
                            continue;
                        }
                        // Date to DateTime (min)
                        let dt_min: chrono::DateTime<chrono::Utc> = {
                            let min_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", final_widget.min.clone())
                            } else {
                                final_widget.min.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &min_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        // Date to DateTime (max)
                        let dt_max: chrono::DateTime<chrono::Utc> = {
                            let max_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", final_widget.max.clone())
                            } else {
                                final_widget.max.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &max_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        // Check hit in range (min <> max)
                        if dt_value < dt_min || dt_value > dt_max {
                            is_err_symptom = true;
                            final_widget.error = Self::accumula_err(
                                &final_widget,
                                &"Date out of range between `min` and` max`.".to_owned(),
                            )
                            .unwrap();
                            continue;
                        }
                    }
                    // Create datetime in bson type
                    // -----------------------------------------------------------------------------
                    let dt_value_bson = mongodb::bson::Bson::DateTime(dt_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &dt_value_bson, &coll).unwrap_or_else(
                            |err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            },
                        );
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), dt_value_bson);
                    }
                }
                "checkBoxI32" | "radioI32" | "numberI32" | "rangeI32" | "selectI32" => {
                    // Get field value for validation
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !final_widget.value.is_empty() {
                                    field_value =
                                        Some(final_widget.value.trim().parse::<i64>().unwrap());
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    final_widget.value = String::new();
                                    continue;
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: i32 = field_value.unwrap() as i32;
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Int32(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "checkBoxU32" | "radioU32" | "numberU32" | "rangeU32" | "selectU32"
                | "checkBoxI64" | "radioI64" | "numberI64" | "rangeI64" | "selectI64" => {
                    // Get field value for validation
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !final_widget.value.is_empty() {
                                    field_value =
                                        Some(final_widget.value.trim().parse::<i64>().unwrap());
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    final_widget.value = String::new();
                                    continue;
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: i64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Int64(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "checkBoxF64" | "radioF64" | "numberF64" | "rangeF64" | "selectF64" => {
                    // Get field value for validation
                    let mut field_value: Option<f64> = pre_json_value.as_f64();
                    // Define field state flag
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !final_widget.value.is_empty() {
                                    field_value =
                                        Some(final_widget.value.trim().parse::<f64>().unwrap());
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    final_widget.value = String::new();
                                    continue;
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: f64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Double(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value.clone();
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "checkBoxBool" => {
                    // Get field value for validation
                    // -----------------------------------------------------------------------------
                    let field_value: bool = if pre_json_value.is_null() {
                        false
                    } else {
                        true
                    };
                    // In case of an error, return the current
                    // state of the field to the user (client)
                    final_widget.checked = field_value.clone();
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Boolean(field_value);
                    // Validation of `unique`
                    // ( For a particularly exceptional case )
                    // -----------------------------------------------------------------------------
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                _ => Err(format!(
                    "Model: `{}` > Field: `{}` > Method: \
                                                `check()` : Unsupported data type.",
                    model_name, field_name
                ))?,
            }

            // Insert or update fields for timestamps `created_at` and `updated_at`
            // -------------------------------------------------------------------------------------
            if !is_err_symptom {
                let dt: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                if !is_update {
                    final_doc.insert("created_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                    final_doc.insert("updated_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                } else {
                    final_doc.insert("updated_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                }
            }
        }

        // Return result
        // -----------------------------------------------------------------------------------------
        Ok(OutputData::Check((
            !is_err_symptom,
            meta.fields_name.clone(),
            final_map_widgets,
            final_doc,
        )))
    }

    // Checking the Model before queries the database
    // ---------------------------------------------------------------------------------------------
    fn save(&mut self) -> Result<OutputData, Box<dyn std::error::Error>> {
        // Get checked data from the `check()` method
        let verified_data: OutputData = self.check()?;
        let is_no_error: bool = verified_data.bool()?;
        // Get access to the cache
        let key: String = Self::key_store()?;
        let form_store: std::sync::MutexGuard<'_, std::collections::HashMap<String, FormCache>> =
            FORM_CACHE.lock().unwrap();
        let form_cache: Option<&FormCache> = form_store.get(&key[..]);
        let form_cache: &FormCache = form_cache.unwrap();
        // Get metadata from cache
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client from cache
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock().unwrap();
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        // Get widget map
        let mut final_map_widgets: std::collections::HashMap<String, Widget> =
            verified_data.wig()?;
        let is_update: bool = !self.get_hash().unwrap_or_default().is_empty();
        let coll: mongodb::sync::Collection = client_cache
            .database(&meta.database_name)
            .collection(&meta.collection_name);

        // Save to database
        if is_no_error {
            let final_doc = verified_data.doc()?;
            if !is_update {
                let result: mongodb::results::InsertOneResult = coll.insert_one(final_doc, None)?;
                self.set_hash(result.inserted_id.as_object_id().unwrap().to_hex());
            } else if !final_doc.is_empty() {
                let object_id: mongodb::bson::oid::ObjectId =
                    mongodb::bson::oid::ObjectId::with_string(
                        self.get_hash().unwrap_or_default().as_str(),
                    )?;
                let query: mongodb::bson::document::Document =
                    mongodb::bson::doc! {"_id": object_id};
                let update: mongodb::bson::document::Document = mongodb::bson::doc! {
                    "$set".to_string() :
                    mongodb::bson::Bson::Document(final_doc),
                };
                coll.update_one(query, update, None)?;
            }
        }

        // Add hash-line (for document identification)
        let hash = self.get_hash().unwrap_or_default();
        if !hash.is_empty() {
            final_map_widgets.get_mut(&"hash".to_owned()).unwrap().value = hash.clone();
        }

        // Return result
        Ok(OutputData::Save((
            is_no_error,
            hash,
            meta.fields_name.clone(),
            final_map_widgets,
        )))
    }
}

// Methods for additional validation.
// **For custom use, add the Model attribute `is_use_add_valid = true`.
// ( Remember to use for validate of ignored fields )
// *************************************************************************************************
pub trait AdditionalValidation {
    // Default implementation as a stub
    // ---------------------------------------------------------------------------------------------
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // .insert("field_name", "Error message")
        let error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        Ok(error_map)
    }
}
