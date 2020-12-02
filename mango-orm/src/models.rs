//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).
//! `ToModel` - Model options and widget map for Form.

use crate::{
    forms::{OutputData, OutputType, Widget},
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

    // Get Json-line
    // ---------------------------------------------------------------------------------------------
    fn to_json(
        map_widgets: &std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut json_text = String::new();
        for (field_name, widget) in map_widgets {
            let widget_json = serde_json::to_string(&widget).unwrap();
            if !json_text.is_empty() {
                json_text = format!("{},\"{}\":{}", json_text, field_name, widget_json);
            } else {
                json_text = format!("\"{}\":{}", field_name, widget_json);
            }
        }
        Ok(format!("{{{}}}", json_text))
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
    fn check(&self, output_format: OutputType) -> Result<OutputData, Box<dyn std::error::Error>> {
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
        let mut err_symptom = false;
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
        let mut map_widgets: std::collections::HashMap<String, Widget> =
            form_cache.map_widgets.clone();
        // Apply additional validation
        {
            let error_map = self.medium_add_validation()?;
            if !error_map.is_empty() {
                err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if !fields_name.contains(&field_name) {
                        Err(format!(
                            "Model: `{}` >  Method: `add_validation()` : \
                                            The `{}` field is missing from the model.",
                            model_name, field_name
                        ))?
                    }
                    if let Some(widget) = map_widgets.get_mut(&field_name.to_owned()) {
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
            let widget: &mut Widget = map_widgets.get_mut(field_name).unwrap();
            let widget_type: &str = &widget.widget.clone()[..];
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
                            widget.value = clean_data.clone();
                        } else {
                            widget.value = String::new();
                        }
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if widget.required {
                            err_symptom = true;
                            widget.error =
                                Self::accumula_err(&widget, &"Required field.".to_owned()).unwrap();
                            widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default
                            if !is_update && widget_type != "inputPassword" {
                                if !widget.value.is_empty() {
                                    field_value = widget.value.trim().to_string();
                                    widget.value = String::new();
                                } else if !err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    widget.value = String::new();
                                    continue;
                                } else {
                                    widget.value = String::new();
                                    continue;
                                }
                            } else {
                                widget.value = String::new();
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
                    Self::check_minlength(widget.minlength, field_value).unwrap_or_else(|err| {
                        err_symptom = true;
                        widget.error = Self::accumula_err(&widget, &err.to_string()).unwrap();
                    });
                    Self::check_maxlength(widget.maxlength, field_value).unwrap_or_else(|err| {
                        err_symptom = true;
                        widget.error = Self::accumula_err(&widget, &err.to_string()).unwrap();
                    });
                    // Validation of range (`min` <> `max`)
                    // ( Hint: The `validate_length()` method did not
                    // provide the desired result )
                    // -----------------------------------------------------------------------------
                    let min: f64 = widget.minlength.clone() as f64;
                    let max: f64 = widget.maxlength.clone() as f64;
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
                        err_symptom = true;
                        let msg = format!(
                            "Length {} is out of range (min={} <> max={}).",
                            len, min, max
                        );
                        widget.error = Self::accumula_err(&widget, &msg).unwrap();
                    }
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if widget_type != "inputPassword" && widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation in regular expression (email, password, etc...)
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        err_symptom = true;
                        widget.error = Self::accumula_err(&widget, &err.to_string()).unwrap();
                    });
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
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
                "inputDate" | "inputDateTime" => {
                    // Get field value for validation
                    let mut field_value: String = if pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        widget.value = clean_data.clone();
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if widget.required {
                            err_symptom = true;
                            widget.error =
                                Self::accumula_err(&widget, &"Required field.".to_owned()).unwrap();
                            widget.value = String::new();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default
                                if !widget.value.is_empty() {
                                    field_value = widget.value.trim().to_string();
                                    widget.value = String::new();
                                } else if !err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    widget.value = String::new();
                                    continue;
                                } else {
                                    widget.value = String::new();
                                    continue;
                                }
                            } else {
                                widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        err_symptom = true;
                        widget.error = Self::accumula_err(&widget, &err.to_string()).unwrap();
                    });
                    if err_symptom {
                        continue;
                    }
                    // Create Date and Time Object
                    // -----------------------------------------------------------------------------
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
                    if !widget.min.is_empty() && !widget.max.is_empty() {
                        let dt_min: chrono::DateTime<chrono::Utc> = {
                            let min_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", widget.min.clone())
                            } else {
                                widget.min.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &min_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        let dt_max: chrono::DateTime<chrono::Utc> = {
                            let max_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", widget.max.clone())
                            } else {
                                widget.max.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &max_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        if dt_value < dt_min || dt_value > dt_max {
                            err_symptom = true;
                            widget.error = Self::accumula_err(
                                &widget,
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
                    if widget.unique {
                        Self::check_unique(hash, field_name, &dt_value_bson, &coll).unwrap_or_else(
                            |err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            },
                        );
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), dt_value_bson);
                    }
                }
                "checkBoxI32" | "radioI32" | "numberI32" | "rangeI32" | "selectI32" => {
                    // Get field value for validation
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if widget.required {
                            err_symptom = true;
                            widget.error =
                                Self::accumula_err(&widget, &"Required field.".to_owned()).unwrap();
                            widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !widget.value.is_empty() {
                                    field_value = Some(widget.value.trim().parse::<i64>().unwrap());
                                    widget.value = String::new();
                                } else if !err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    widget.value = String::new();
                                    continue;
                                } else {
                                    widget.value = String::new();
                                    continue;
                                }
                            } else {
                                widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: i32 = field_value.unwrap() as i32;
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Int32(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = widget.min.parse().unwrap();
                    let max: f64 = widget.max.parse().unwrap();
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
                        err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        widget.error = Self::accumula_err(&widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
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
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if widget.required {
                            err_symptom = true;
                            widget.error =
                                Self::accumula_err(&widget, &"Required field.".to_owned()).unwrap();
                            widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !widget.value.is_empty() {
                                    field_value = Some(widget.value.trim().parse::<i64>().unwrap());
                                    widget.value = String::new();
                                } else if !err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    widget.value = String::new();
                                    continue;
                                } else {
                                    widget.value = String::new();
                                    continue;
                                }
                            } else {
                                widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: i64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Int64(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = widget.min.parse().unwrap();
                    let max: f64 = widget.max.parse().unwrap();
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
                        err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        widget.error = Self::accumula_err(&widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name.to_string(), bson_field_value);
                    }
                }
                "checkBoxF64" | "radioF64" | "numberF64" | "rangeF64" | "selectF64" => {
                    // Get field value for validation
                    let mut field_value: Option<f64> = pre_json_value.as_f64();
                    // Define field state flag
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if widget.required {
                            err_symptom = true;
                            widget.error =
                                Self::accumula_err(&widget, &"Required field.".to_owned()).unwrap();
                            widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !is_update {
                                if !widget.value.is_empty() {
                                    field_value = Some(widget.value.trim().parse::<f64>().unwrap());
                                    widget.value = String::new();
                                } else if !err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc
                                        .insert(field_name.to_string(), mongodb::bson::Bson::Null);
                                    widget.value = String::new();
                                    continue;
                                } else {
                                    widget.value = String::new();
                                    continue;
                                }
                            } else {
                                widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data
                    let field_value: f64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client)
                        widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Double(field_value);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    if widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`)
                    // -----------------------------------------------------------------------------
                    let min: f64 = widget.min.parse().unwrap();
                    let max: f64 = widget.max.parse().unwrap();
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
                        err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        widget.error = Self::accumula_err(&widget, &msg).unwrap();
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
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
                    widget.checked = field_value.clone();
                    // Used to validation uniqueness and in the final result
                    let bson_field_value = mongodb::bson::Bson::Boolean(field_value);
                    // Validation of `unique`
                    // ( For a particularly exceptional case )
                    // -----------------------------------------------------------------------------
                    if widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                err_symptom = true;
                                widget.error =
                                    Self::accumula_err(&widget, &err.to_string()).unwrap();
                            });
                    }
                    // Insert result
                    // -----------------------------------------------------------------------------
                    if !err_symptom && !ignore_fields.contains(&field_name) {
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
            if !err_symptom {
                let dt: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                if !is_update {
                    final_doc.insert("created_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                    final_doc.insert("updated_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                } else {
                    final_doc.insert("updated_at".to_string(), mongodb::bson::Bson::DateTime(dt));
                }
            }
        }

        // Post processing
        // -----------------------------------------------------------------------------------------
        let result: OutputData = match output_format {
            // Get Hash-line
            OutputType::Hash => {
                let data: String = Self::to_hash(&map_widgets)?;
                OutputData::Hash((data, !err_symptom, final_doc))
            }
            // Get Attribute Map
            OutputType::Wig => OutputData::Wig((map_widgets, !err_symptom, final_doc)),
            // Get Json-line
            OutputType::Json => {
                let data: String = Self::to_json(&map_widgets)?;
                OutputData::Json((data, !err_symptom, final_doc))
            }
            // Get Html-line
            OutputType::Html => {
                let data: String = Self::medium_to_html(&meta.fields_name, map_widgets)?;
                OutputData::Html((data, !err_symptom, final_doc))
            }
        };

        Ok(result)
    }

    // Checking the Model before queries the database
    // ---------------------------------------------------------------------------------------------
    fn save(
        &mut self,
        output_format: OutputType,
    ) -> Result<OutputData, Box<dyn std::error::Error>> {
        //
        let verified_data: OutputData = self.check(OutputType::Wig)?;
        //
        let key: String = Self::key_store()?;
        let form_store: std::sync::MutexGuard<'_, std::collections::HashMap<String, FormCache>> =
            FORM_CACHE.lock().unwrap();
        let form_cache: Option<&FormCache> = form_store.get(&key[..]);
        let form_cache: &FormCache = form_cache.unwrap();
        //
        let meta: &Meta = &form_cache.meta;
        //
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock().unwrap();
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        //
        let mut map_widgets: std::collections::HashMap<String, Widget> = verified_data.wig();
        let is_update: bool = !self.get_hash().unwrap_or_default().is_empty();
        let coll: mongodb::sync::Collection = client_cache
            .database(&meta.database_name)
            .collection(&meta.collection_name);

        // Save to database
        if verified_data.bool() {
            let final_doc = verified_data.doc();
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

        // Add hash-line
        if self.get_hash().is_some() {
            map_widgets.get_mut(&"hash".to_owned()).unwrap().value =
                self.get_hash().unwrap_or_default();
        }

        // Post processing
        let result: OutputData = match output_format {
            // Get Hash-line
            OutputType::Hash => {
                let data: String = Self::to_hash(&map_widgets)?;
                OutputData::Hash((data, verified_data.bool(), verified_data.doc()))
            }
            // Get Attribute Map
            OutputType::Wig => {
                OutputData::Wig((map_widgets, verified_data.bool(), verified_data.doc()))
            }
            // Get Json-line
            OutputType::Json => {
                let data: String = Self::to_json(&map_widgets)?;
                OutputData::Json((data, verified_data.bool(), verified_data.doc()))
            }
            // Get Html-line
            OutputType::Html => {
                let data: String = Self::medium_to_html(&meta.fields_name, map_widgets)?;
                OutputData::Html((data, verified_data.bool(), verified_data.doc()))
            }
        };

        Ok(result)
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
