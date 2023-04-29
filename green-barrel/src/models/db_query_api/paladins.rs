//! Query methods for a Model instance.

use async_trait::async_trait;
use chrono::{format::ParseErrorKind, DateTime, Utc};
use image::imageops::FilterType::{Nearest, Triangle};
use mongodb::{
    bson::{doc, oid::ObjectId, ser::to_bson, spec::ElementType, Bson, Document},
    options::{DeleteOptions, FindOneOptions, InsertOneOptions, UpdateOptions},
    results::InsertOneResult,
    Client, Collection,
};
use rand::Rng;
use regex::Regex;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{json, Value};
use slug::slugify;
use std::{convert::TryFrom, error::Error, fs, fs::Metadata, path::Path};
use uuid::Uuid;

use crate::{
    meta_store::META_STORE,
    models::{
        caching::Caching,
        helpers::{FileData, ImageData},
        hooks::Hooks,
        output_data::{OutputData, OutputData2},
        validation::{AdditionalValidation, Validation},
        Main,
    },
};

#[async_trait(?Send)]
pub trait QPaladins: Main + Caching + Hooks + Validation + AdditionalValidation {
    /// Deleting a file in the database and in the file system.
    // *********************************************************************************************
    async fn delete_file(
        &self,
        coll: &Collection<Document>,
        model_name: &str,
        field_name: &str,
        file_default: Option<FileData>,
        image_default: Option<ImageData>,
    ) -> Result<(), Box<dyn Error>> {
        //
        let hash = self.hash();
        if !hash.is_empty() {
            let object_id = ObjectId::parse_str(hash.as_str())?;
            let filter = doc! {"_id": object_id};
            if let Some(document) = coll.find_one(filter.clone(), None).await? {
                // If `is_deleted=true` was passed incorrectly.
                if document.is_null(field_name) {
                    return Ok(());
                }
                // Delete the file information in the database.
                let file_doc = doc! {field_name: Bson::Null};
                let update = doc! { "$set": file_doc };
                coll.update_one(filter, update, None).await?;
                // Delete the orphaned file.
                if let Some(info_file) = document.get(field_name).unwrap().as_document() {
                    if let Some(file_default) = file_default {
                        let path_default = file_default.path;
                        let path = info_file.get_str("path")?;
                        if path != path_default {
                            let path = Path::new(path);
                            if path.is_file() {
                                fs::remove_file(path)?;
                            }
                        }
                    } else if let Some(image_default) = image_default {
                        let path_default = image_default.path;
                        let path = info_file.get_str("path")?;
                        if path != path_default {
                            let dir_path = Path::new(path).parent().unwrap();
                            if dir_path.is_dir() {
                                fs::remove_dir_all(dir_path)?;
                            }
                        }
                    }
                } else {
                    Err(format!(
                        "Model: `{model_name}` > Field: `{field_name}` ; \
                        Method: `delete_file()` => Document (info file) not found."
                    ))?
                }
            } else {
                Err(format!(
                    "Model: `{model_name}` > Field: `{field_name}` ; \
                    Method: `delete_file()` => Document not found."
                ))?
            }
        }
        //
        Ok(())
    }

    /// Get file info from database.
    // *********************************************************************************************
    async fn db_get_file_info(
        &self,
        coll: &Collection<Document>,
        field_name: &str,
    ) -> Result<Value, Box<dyn Error>> {
        //
        let hash = self.hash();
        if !hash.is_empty() {
            let object_id = ObjectId::parse_str(hash.as_str())?;
            let filter = doc! {"_id": object_id};
            if let Some(document) = coll.find_one(filter, None).await? {
                if let Some(doc) = document.get(field_name).unwrap().as_document() {
                    let result = serde_json::to_value(doc)?;
                    return Ok(result);
                }
            }
        }
        //
        Ok(json!(null))
    }

    /// Calculate the maximum size for a thumbnail.
    // *********************************************************************************************
    fn calculate_thumbnail_size(width: f64, height: f64, max_size: f64) -> (f64, f64) {
        if width > height {
            if width > max_size {
                return (max_size, (height * (max_size / width)).floor());
            }
        } else if height > max_size {
            return ((width * (max_size / height)).floor(), max_size);
        }
        (0.0, 0.0)
    }

    /// Checking the Model before queries the database.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// use chrono::Local;
    /// let tz = Some(Local::now().format("%z").to_string()); // or None
    ///
    /// let mut model_name = ModelName::new()?;
    /// let output_data = model_name.check(&client, &tz, None).await?;
    /// if !output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    async fn check(
        &mut self,
        client: &Client,
        params: Option<(bool, bool)>,
    ) -> Result<OutputData2, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get locks.
        let (is_save, is_slug_update) = params.unwrap_or((false, false));
        // Get metadata.
        let (
            model_name,
            choice_str_map,
            choice_i32_map,
            choice_i64_map,
            choice_f64_map,
            ignore_fields,
            collection_name,
            fields_name,
            database_name,
            is_use_add_valid,
            is_add_doc,
            is_up_doc,
            app_name,
            unique_app_key,
        ) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.model_name.clone(),
                    meta.choice_str_map.clone(),
                    meta.choice_i32_map.clone(),
                    meta.choice_i64_map.clone(),
                    meta.choice_f64_map.clone(),
                    meta.ignore_fields.clone(),
                    meta.collection_name.clone(),
                    meta.fields_name.clone(),
                    meta.database_name.clone(),
                    meta.is_use_add_valid,
                    meta.is_add_doc,
                    meta.is_up_doc,
                    meta.app_name.clone(),
                    meta.unique_app_key.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `check()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get model name.
        let model_name = model_name.as_str();
        // Determines the mode of accessing the database (create or update).
        let hash = &self.hash();
        let is_update: bool = !hash.is_empty();
        // User input error detection symptom.
        let mut is_err_symptom = false;
        // To block the reuse of previously saved files and images in the media directory.
        let regex_is_dated_path = Regex::new(r"(?:(?:/|\\)\d{4}\-\d{2}\-\d{2}\-barrel(?:/|\\))")?;
        // Access the collection.
        let coll = client
            .database(&database_name)
            .collection::<Document>(&collection_name);
        // Get preliminary data from model instance and use for final result.
        let mut final_model_json = self.self_to_json_val()?;
        // Document for the final result.
        let mut final_doc = Document::new();
        // Apply additional validation.
        if is_use_add_valid {
            let error_map = self.add_validation(client).await?;
            if !error_map.is_empty() {
                is_err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if let Some(final_field) = final_model_json.get_mut(field_name.clone()) {
                        Self::accumula_err(final_field, &err_msg);
                    } else {
                        Err(format!(
                            "Model: `{model_name}` ;  Method: `add_validation()` => \
                                The model has no field `{field_name}`."
                        ))?
                    }
                }
            }
        }
        // Check param `alert` in `hash` field.
        if !is_slug_update {
            let alert = final_model_json["hash"]["alert"]
                .as_str()
                .unwrap()
                .to_string();
            if !alert.is_empty() {
                is_err_symptom = true;
            }
            if is_save {
                if !is_update && !is_add_doc {
                    let msg = if !alert.is_empty() {
                        format!("{alert}<br>It is forbidden to perform saves!")
                    } else {
                        String::from("It is forbidden to perform saves!")
                    };
                    is_err_symptom = true;
                    *final_model_json
                        .get_mut("hash")
                        .unwrap()
                        .get_mut("alert")
                        .unwrap() = json!(msg);
                }
                if is_update && !is_up_doc {
                    let msg = if !alert.is_empty() {
                        format!("{alert}<br>It is forbidden to perform updates!")
                    } else {
                        String::from("It is forbidden to perform updates!")
                    };
                    is_err_symptom = true;
                    *final_model_json
                        .get_mut("hash")
                        .unwrap()
                        .get_mut("alert")
                        .unwrap() = json!(msg);
                }
            }
        }
        // Loop over fields for validation.
        for field_name in fields_name.iter() {
            // Don't check the `hash` field.
            if field_name == "hash" {
                continue;
            }
            // Get values for validation.
            let final_field = final_model_json.get_mut(field_name).unwrap();
            // Define conditional constants.
            let mut is_use_default = false;
            let mut const_value = {
                let mut tmp = json!(null);
                if let Some(val) = final_field.get("value") {
                    if (val.is_string() && val.as_str().unwrap().is_empty())
                        || (val.is_array() && val.as_array().unwrap().is_empty())
                    {
                        tmp = json!(null);
                    } else {
                        tmp = val.clone();
                    }
                };
                if tmp.is_null() {
                    if let Some(val) = final_field.get("default") {
                        let val = val.clone();
                        *final_field.get_mut("value").unwrap() = val.clone();
                        tmp = val;
                        is_use_default = true;
                    }
                }
                if tmp.is_null() && !is_use_default {
                    if let Some(val) = final_field.get("checked") {
                        tmp = val.clone();
                    }
                }
                tmp
            };
            let const_group = final_field["group"].as_i64().unwrap();
            //
            let is_required = if let Some(required) = final_field.get("required") {
                required.as_bool().unwrap()
            } else {
                false
            };
            let is_hide = final_field["is_hide"].as_bool().unwrap();
            //
            let field_type_string = final_field["field_type"].as_str().unwrap().to_string();
            let field_type = field_type_string.as_str();

            // Field validation.
            match const_group {
                // Validation of Text type fields.
                // *********************************************************************************
                /*
                "Color" | "Email" | "Password" | "Phone"
                | "Text" | "Hash" | "Url" | "IP"
                */
                1 => {
                    // When updating, we skip field password type.
                    if is_update && field_type == "Password" {
                        *final_field.get_mut("value").unwrap() = json!(null);
                        continue;
                    }
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    //
                    let curr_val = const_value.as_str().unwrap();
                    //
                    if final_field["input_type"].as_str().unwrap() == "radio"
                        && !choice_str_map
                            .get(field_name)
                            .unwrap()
                            .contains(&curr_val.to_string())
                    {
                        is_err_symptom = true;
                        Self::accumula_err(final_field, &t!("not_match_choices"));
                        continue;
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = if field_type != "Password" {
                        Bson::String(curr_val.to_string())
                    } else {
                        Bson::Null
                    };
                    // Validation field attribute `regex`.
                    if let Some(pattern) = final_field.get("regex") {
                        Self::regex_pattern_validation(curr_val, pattern.as_str().unwrap())
                            .unwrap_or_else(|_err| {
                                is_err_symptom = true;
                                let regex_err_msg =
                                    final_field["regex_err_msg"].as_str().unwrap().to_string();
                                if !is_hide {
                                    Self::accumula_err(final_field, &regex_err_msg);
                                } else {
                                    Err(format!(
                                        "Model: `{model_name}` > Field: `{field_name}` ; \
                                        Method: `check()` => {0:?}",
                                        regex_err_msg
                                    ))
                                    .unwrap()
                                }
                            });
                    }
                    // Validation in regular expression.
                    // Checking `minlength`.
                    if let Some(minlength) = final_field.get("minlength") {
                        Self::check_minlength(minlength.as_i64().unwrap() as usize, curr_val)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                if !is_hide {
                                    Self::accumula_err(final_field, &err.to_string());
                                } else {
                                    Err(format!(
                                        "Model: `{model_name}` > Field: `{field_name}` ; \
                                        Method: `check()` => {0:?}",
                                        err
                                    ))
                                    .unwrap()
                                }
                            });
                    }
                    // Checking `maxlength`.
                    if let Some(maxlength) = final_field.get("maxlength") {
                        Self::check_maxlength(maxlength.as_i64().unwrap() as usize, curr_val)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                if !is_hide {
                                    Self::accumula_err(final_field, &err.to_string());
                                } else {
                                    Err(format!(
                                        "Model: `{model_name}` > Field: `{field_name}` ; \
                                        Method: `check()` => {0:?}",
                                        err
                                    ))
                                    .unwrap()
                                }
                            });
                    }
                    // Validation of `unique`.
                    if let Some(unique) = final_field.get("unique") {
                        let is_unique = unique.as_bool().unwrap();
                        if field_type != "Password" && is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .await
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        Self::accumula_err(final_field, &err.to_string());
                                    } else {
                                        Err(format!(
                                            "Model: `{model_name}` > Field: `{field_name}` ; \
                                            Method: `check()` => {0:?}",
                                            err
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Validation in regular expression - Email, Url, IP, IPv4, IPv6.
                    Self::regex_validation(field_type, curr_val).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        if !is_hide {
                            Self::accumula_err(final_field, &err.to_string());
                        } else {
                            Err(format!(
                                "Model: `{model_name}` > Field: `{field_name}` ; \
                                Method: `check()` => {0:?}",
                                err
                            ))
                            .unwrap()
                        }
                    });
                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        match field_type {
                            "Password" => {
                                if !curr_val.is_empty() && !is_update {
                                    // Generate password hash and add to result document.
                                    let password_hash: String =
                                        Self::create_password_hash(curr_val)?;
                                    final_doc.insert(field_name, Bson::String(password_hash));
                                }
                            }
                            _ => {
                                // Insert result from other fields.
                                final_doc.insert(field_name, field_value_bson);
                            }
                        }
                    }
                }
                // Validation of Slug type fields.
                // *********************************************************************************
                // "Slug"
                2 => {
                    let mut slug = String::new();
                    let slug_sources = final_field
                        .get("slug_sources")
                        .unwrap()
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_str().unwrap())
                        .collect::<Vec<&str>>();
                    //
                    let tmp_model_json = self.self_to_json_val()?;
                    for field_name in slug_sources {
                        if let Some(value) = tmp_model_json.get(field_name).unwrap().get("value") {
                            if value.is_string() {
                                let text = value.as_str().unwrap().trim();
                                slug = format!("{}-{}", slug, text);
                            } else if const_value.is_i64() {
                                let num = value.as_i64().unwrap();
                                slug = format!("{}-{}", slug, num);
                            } else if const_value.is_f64() {
                                let num = value.as_f64().unwrap();
                                slug = format!("{}-{}", slug, num);
                            }
                        }
                    }
                    //
                    if slug.is_empty() && !const_value.is_null() {
                        slug = const_value.as_str().unwrap().trim().to_string();
                    }
                    // Validation, if the field is required and empty, accumulate the error.
                    if slug.is_empty() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if !is_save {
                        continue;
                    }
                    slug = slugify(slug);
                    *final_field.get_mut("value").unwrap() = json!(slug);
                    let field_value_bson = Bson::String(slug.clone());
                    // Validation of `unique`.
                    if final_field.get("unique").unwrap().as_bool().unwrap() {
                        Self::check_unique(hash, field_name, &field_value_bson, &coll)
                            .await
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &err.to_string());
                            });
                    }
                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                // Validation of date type fields.
                // *********************************************************************************
                // "Date" | "DateTime" | "HiddenDateTime"
                3 => {
                    // Don't check the `created_at`and updated_at fields.
                    if field_name == "created_at" || field_name == "updated_at" {
                        continue;
                    }
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    //
                    let curr_val = const_value.as_str().unwrap();
                    let tz = "+00:00";
                    // Create a Date object for the current value.
                    let curr_dt = {
                        let (val, err_msg, err_msg_2) = if field_type == "Date" {
                            (
                                format!("{curr_val}T00:00{tz}"),
                                t!("non_existent_date"),
                                format!(
                                    "{} {} {}",
                                    t!("incorrect_date"),
                                    t!("format", sample = "yyyy-mm-dd"),
                                    t!("example", sample = "1970-02-28")
                                ),
                            )
                        } else {
                            (
                                format!("{curr_val}{tz}"),
                                t!("non_existent_date_time"),
                                format!(
                                    "{} {} {}",
                                    t!("incorrect_date_time"),
                                    t!("format", sample = "yyyy-mm-ddThh:mm"),
                                    t!("example", sample = "1970-02-28T00:00")
                                ),
                            )
                        };
                        match DateTime::parse_from_str(&val, "%Y-%m-%dT%H:%M%z") {
                            Ok(dt) => DateTime::<Utc>::from(dt),
                            Err(error) => {
                                if error.kind() == ParseErrorKind::OutOfRange {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &err_msg);
                                    continue;
                                } else {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &err_msg_2);
                                    continue;
                                }
                            }
                        }
                    };
                    // Compare with `min`.
                    let min = final_field["min"].as_str().unwrap();
                    if !min.is_empty() {
                        // Get the minimum date object.
                        let min_dt = {
                            let (val, err_msg, err_msg_2) = if field_type == "Date" {
                                (
                                    format!("{min}T00:00{tz}"),
                                    "Non-existent date!",
                                    "Param min - Incorrect date format.\
                                    Example: 1970-02-28",
                                )
                            } else {
                                (
                                    format!("{curr_val}{tz}"),
                                    "Non-existent date or time!",
                                    "Param min - Incorrect date and time format.\
                                    Example: 1970-01-01T00:00",
                                )
                            };
                            match DateTime::parse_from_str(&val, "%Y-%m-%dT%H:%M%z") {
                                Ok(dt) => DateTime::<Utc>::from(dt),
                                Err(error) => {
                                    if error.kind() == ParseErrorKind::OutOfRange {
                                        Err(format!(
                                            "Model: `{model_name}` > Field: `{field_name}` > \
                                            Param: `min` ; Method: `check()` => {err_msg}"
                                        ))
                                        .unwrap()
                                    } else {
                                        Err(format!(
                                            "Model: `{model_name}` > Field: `{field_name}` ; \
                                            Method: `check()` => {err_msg_2}"
                                        ))
                                        .unwrap()
                                    }
                                }
                            }
                        };
                        // Match dates.
                        if curr_dt < min_dt {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("date_less_min"));
                            continue;
                        }
                    }
                    // Compare with `max`.
                    let max = final_field["max"].as_str().unwrap();
                    if !max.is_empty() {
                        // Get the maximum date object.
                        let max_dt = {
                            let (val, err_msg, err_msg_2) = if field_type == "Date" {
                                (
                                    format!("{max}T00:00{tz}"),
                                    "Non-existent date!",
                                    "Param max - Incorrect date format.\
                                    Example: 1970-02-28",
                                )
                            } else {
                                (
                                    format!("{curr_val}{tz}"),
                                    "Non-existent date or time!",
                                    "Param max - Incorrect date and time format.\
                                    Example: 1970-01-01T00:00",
                                )
                            };
                            match DateTime::parse_from_str(&val, "%Y-%m-%dT%H:%M%z") {
                                Ok(dt) => DateTime::<Utc>::from(dt),
                                Err(error) => {
                                    if error.kind() == ParseErrorKind::OutOfRange {
                                        Err(format!(
                                            "Model: `{model_name}` > Field: `{field_name}` > \
                                            Param: `max` ; Method: `check()` => {err_msg}"
                                        ))
                                        .unwrap()
                                    } else {
                                        Err(format!(
                                            "Model: `{model_name}` > Field: `{field_name}` ; \
                                            Method: `check()` => {err_msg_2}"
                                        ))
                                        .unwrap()
                                    }
                                }
                            }
                        };
                        // Match dates.
                        if curr_dt > max_dt {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("date_greater_max"));
                            continue;
                        }
                    }
                    // Create datetime in bson type.
                    let val_dt_bson = Bson::DateTime(curr_dt.into());
                    // Validation of `unique`
                    if final_field["unique"].as_bool().unwrap() {
                        Self::check_unique(hash, field_name, &val_dt_bson, &coll)
                            .await
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_unique"));
                            });
                    }
                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        final_doc.insert(field_name, val_dt_bson);
                    }
                }
                // Validation of `choice` type fields.
                // *********************************************************************************
                // "ChoiceText" | "ChoiceI32" | "ChoiceU32" | "ChoiceI64" | "ChoiceF64"
                4 => {
                    //
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get selected items.
                    match field_type {
                        "ChoiceText" => {
                            let val = const_value.as_str().unwrap().to_string();
                            let mut flag = true;
                            if choice_str_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceI32" => {
                            let val = i32::try_from(const_value.as_i64().unwrap())?;
                            let mut flag = true;
                            if choice_i32_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceU32" | "ChoiceI64" => {
                            let val = const_value.as_i64().unwrap();
                            let mut flag = true;
                            if choice_i64_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceF64" => {
                            let val = const_value.as_f64().unwrap();
                            let mut flag = true;
                            if choice_f64_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        _ => Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` ; \
                            Method: `check()` => Unsupported field type - `{field_type}`."
                        ))?,
                    }
                }
                //
                // "ChoiceTextDyn" | "ChoiceI32Dyn" | "ChoiceU32Dyn" | "ChoiceI64Dyn" | "ChoiceF64Dyn"
                5 => {
                    //
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get selected items.
                    match field_type {
                        "ChoiceTextDyn" => {
                            let val = const_value.as_str().unwrap().to_string();
                            let mut flag = true;
                            if choice_str_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceI32Dyn" => {
                            let val = i32::try_from(const_value.as_i64().unwrap())?;
                            let mut flag = true;
                            if choice_i32_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceU32Dyn" | "ChoiceI64Dyn" => {
                            let val = const_value.as_i64().unwrap();
                            let mut flag = true;
                            if choice_i64_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceF64Dyn" => {
                            let val = const_value.as_f64().unwrap();
                            let mut flag = true;
                            if choice_f64_map.get(field_name).unwrap().contains(&val) {
                                flag = true;
                            } else {
                                is_err_symptom = true;
                                Self::accumula_err(final_field, &t!("not_match_choices"));
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        _ => Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` ; \
                            Method: `check()` =>  Unsupported field type - `{field_type}`."
                        ))?,
                    }
                }
                //
                // "ChoiceTextMult" | "ChoiceI32Mult" | "ChoiceU32Mult" | "ChoiceI64Mult" | "ChoiceF64Mult"
                6 => {
                    //
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get selected items.
                    match field_type {
                        "ChoiceTextMult" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_str().unwrap().into())
                                .collect::<Vec<String>>();
                            let choices = choice_str_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceI32Mult" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| i32::try_from(item.as_i64().unwrap()).unwrap())
                                .collect::<Vec<i32>>();
                            let choices = choice_i32_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceU32Mult" | "ChoiceI64Mult" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_i64().unwrap())
                                .collect::<Vec<i64>>();
                            let choices = choice_i64_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceF64Mult" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_f64().unwrap())
                                .collect::<Vec<f64>>();
                            let choices = choice_f64_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        _ => Err(format!(
                            "Model: `{}` > Field: `{}` ; Method: `check()` => \
                                        Unsupported field type - `{}`.",
                            model_name, field_name, field_type
                        ))?,
                    }
                }
                //
                /*
                "ChoiceTextMultDyn" | "ChoiceI32MultDyn" | "ChoiceU32MultDyn"
                | "ChoiceI64MultDyn" | "ChoiceF64MultDyn"
                */
                7 => {
                    //
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("not_unique"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get selected items.
                    match field_type {
                        "ChoiceTextMultDyn" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_str().unwrap().into())
                                .collect::<Vec<String>>();
                            let choices = choice_str_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceI32MultDyn" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| i32::try_from(item.as_i64().unwrap()).unwrap())
                                .collect::<Vec<i32>>();
                            let choices = choice_i32_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceU32MultDyn" | "ChoiceI64MultDyn" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_i64().unwrap())
                                .collect::<Vec<i64>>();
                            let choices = choice_i64_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        "ChoiceF64MultDyn" => {
                            let val = const_value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|item| item.as_f64().unwrap())
                                .collect::<Vec<f64>>();
                            let choices = choice_f64_map.get(field_name).unwrap();
                            let mut flag = true;
                            for item in val.iter() {
                                if !choices.contains(item) {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("not_match_choices"));
                                    flag = false;
                                    break;
                                }
                            }
                            if is_save {
                                final_doc.insert(
                                    field_name,
                                    if flag { to_bson(&val)? } else { Bson::Null },
                                );
                            }
                        }
                        _ => Err(format!(
                            "Model: `{}` > Field: `{}` ; Method: `check()` => \
                                        Unsupported field type - `{}`.",
                            model_name, field_name, field_type
                        ))?,
                    }
                }
                // Validation of file type fields.
                // *********************************************************************************
                // "File"
                8 => {
                    //
                    if !is_save {
                        continue;
                    }
                    // Get data for validation.
                    let mut file_data = if !is_use_default && !const_value.is_null() {
                        serde_json::from_value::<FileData>(const_value.clone())?
                    } else {
                        FileData::default()
                    };
                    // Delete file.
                    if file_data.is_delete && is_update && !ignore_fields.contains(field_name) {
                        if !is_required || !file_data.path.is_empty() {
                            let file_default;
                            let val = final_field.get("default").unwrap();
                            if !val.is_null() {
                                file_default = serde_json::from_value::<FileData>(val.clone())?;
                                if file_data.path.is_empty() {
                                    const_value = val.clone();
                                    is_use_default = true;
                                }
                            } else {
                                file_default = FileData::default();
                            }
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                Some(file_default),
                                None,
                            )
                            .await?;
                        } else {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("upload_new_file"));
                        }
                    }
                    // Get the current information about file from database.
                    let curr_file_info = self.db_get_file_info(&coll, field_name).await?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if file_data.path.is_empty() {
                        if curr_file_info.is_null() {
                            if is_use_default && !const_value.is_null() {
                                file_data =
                                    serde_json::from_value::<FileData>(const_value.clone())?;
                            } else {
                                if is_required {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, &t!("required_field"));
                                }
                                if !is_update && !ignore_fields.contains(field_name) {
                                    final_doc.insert(field_name, Bson::Null);
                                }
                                continue;
                            }
                        } else {
                            *final_field.get_mut("value").unwrap() = curr_file_info;
                            continue;
                        }
                    }
                    //
                    if is_slug_update || regex_is_dated_path.is_match(file_data.path.as_str()) {
                        *final_field.get_mut("value").unwrap() = curr_file_info;
                        continue;
                    }
                    // Invalid if there is only one value.
                    if file_data.path.is_empty() {
                        Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` > \
                                Type: `FileData` ; Method: `check()` => \
                                An empty `path` field is not allowed.",
                        ))?
                    }
                    // Create path for validation of file.
                    let source_file_path = Path::new(file_data.path.as_str());
                    if !source_file_path.is_file() {
                        Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` ; Method: \
                                `check()` => File is missing - {0}",
                            file_data.path
                        ))?
                    }
                    // Create a new path and URL for the file.
                    {
                        let media_root = final_field["media_root"].as_str().unwrap();
                        let media_url = final_field["media_url"].as_str().unwrap();
                        let target_dir = final_field["target_dir"].as_str().unwrap();
                        let date_slug = format!("{}-barrel", Utc::now().format("%Y-%m-%d"));
                        let file_dir_path = format!("{media_root}/{target_dir}/{date_slug}");
                        let extension = {
                            let path = Path::new(file_data.path.as_str());
                            path.extension().unwrap().to_str().unwrap()
                        };
                        let mut new_file_name;
                        let mut new_file_path;
                        fs::create_dir_all(file_dir_path.clone())?;
                        loop {
                            new_file_name = format!("{}.{extension}", Uuid::new_v4());
                            new_file_path = format!("{file_dir_path}/{new_file_name}");
                            if !Path::new(&new_file_path).is_file() {
                                break;
                            }
                        }
                        fs::copy(source_file_path, new_file_path.clone())?;
                        if !is_use_default {
                            fs::remove_file(source_file_path)?;
                        }
                        file_data.name = new_file_name.clone();
                        file_data.path = new_file_path;
                        file_data.url =
                            format!("{media_url}/{target_dir}/{date_slug}/{new_file_name}");
                    }
                    //
                    let f_path = Path::new(file_data.path.as_str());
                    // Get file metadata.
                    let metadata: Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    file_data.size = metadata.len() as f64;
                    // Insert result.
                    if !ignore_fields.contains(field_name) {
                        // Add file data to controller.
                        *final_field.get_mut("value").unwrap() = serde_json::to_value(file_data)?;
                        //
                        if !is_err_symptom {
                            let value = final_field.get("value").unwrap();
                            let field_value_bson = to_bson(value)?;
                            final_doc.insert(field_name, field_value_bson);
                        }
                    } else {
                        *final_field.get_mut("value").unwrap() = json!(null);
                    }
                }
                //
                // "Image"
                9 => {
                    //
                    if !is_save {
                        continue;
                    }
                    // Get data for validation.
                    let mut image_data = if !is_use_default && !const_value.is_null() {
                        serde_json::from_value::<ImageData>(const_value.clone())?
                    } else {
                        ImageData::default()
                    };
                    // Delete image.
                    if image_data.is_delete && is_update && !ignore_fields.contains(field_name) {
                        if !is_required || !image_data.path.is_empty() {
                            let image_default;
                            let val = final_field.get("default").unwrap();
                            if !val.is_null() {
                                image_default = serde_json::from_value::<ImageData>(val.clone())?;
                                if image_data.path.is_empty() {
                                    const_value = val.clone();
                                    is_use_default = true;
                                }
                            } else {
                                image_default = ImageData::default();
                            }
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                None,
                                Some(image_default),
                            )
                            .await?;
                        } else {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("upload_new_file"));
                        }
                    }
                    // Get the current information about file from database.
                    let curr_file_info = self.db_get_file_info(&coll, field_name).await?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if image_data.path.is_empty() {
                        if curr_file_info.is_null() {
                            if is_use_default && !const_value.is_null() {
                                image_data =
                                    serde_json::from_value::<ImageData>(const_value.clone())?;
                            } else {
                                if is_required {
                                    is_err_symptom = true;
                                    Self::accumula_err(final_field, "Required field.");
                                }
                                if !is_update && !ignore_fields.contains(field_name) {
                                    final_doc.insert(field_name, Bson::Null);
                                }
                                continue;
                            }
                        } else {
                            *final_field.get_mut("value").unwrap() = curr_file_info;
                            continue;
                        }
                    }
                    //
                    if is_slug_update || regex_is_dated_path.is_match(image_data.path.as_str()) {
                        *final_field.get_mut("value").unwrap() = curr_file_info;
                        continue;
                    }
                    // Invalid if there is only one value.
                    if image_data.path.is_empty() {
                        Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` > \
                                Type: `FileData` ; Method: `check()` => \
                                An empty `path` field is not allowed.",
                        ))?
                    }
                    // Validation of file.
                    let source_img_path = Path::new(image_data.path.as_str());
                    if !source_img_path.is_file() {
                        Err(format!(
                            "Model: `{model_name}` > Field: `{field_name}` ; Method: \
                                `check()` => Image is missing - {0}",
                            image_data.path
                        ))?
                    }
                    // Create a new path and URL for the image.
                    let extension = source_img_path
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    let mut img_dir_path;
                    let img_dir_url;
                    {
                        let media_root = final_field["media_root"].as_str().unwrap();
                        let media_url = final_field["media_url"].as_str().unwrap();
                        let target_dir = final_field["target_dir"].as_str().unwrap();
                        let date_slug = format!("{}-barrel", Utc::now().format("%Y-%m-%d"));
                        let new_img_name = format!("main.{extension}");
                        let mut uuid;
                        loop {
                            uuid = Uuid::new_v4().to_string();
                            img_dir_path = format!("{media_root}/{target_dir}/{date_slug}/{uuid}");
                            if !Path::new(&img_dir_path).is_dir() {
                                fs::create_dir_all(img_dir_path.clone())?;
                                break;
                            }
                        }
                        let new_img_path = format!("{img_dir_path}/{new_img_name}");
                        fs::copy(source_img_path, new_img_path.clone())?;
                        if !is_use_default {
                            fs::remove_file(source_img_path)?;
                        }
                        image_data.name = new_img_name.clone();
                        image_data.path = new_img_path;
                        img_dir_url = format!("{media_url}/{target_dir}/{date_slug}/{uuid}");
                        image_data.url = format!("{img_dir_url}/{new_img_name}");
                    }
                    // Get image path.
                    let f_path = Path::new(image_data.path.as_str());
                    // Get file metadata.
                    let metadata: Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    image_data.size = metadata.len() as f64;
                    // Get image width and height.
                    let dimensions = image::image_dimensions(f_path)?;
                    image_data.width = dimensions.0 as f64;
                    image_data.height = dimensions.1 as f64;
                    // Create thumbnails.
                    let thumbnails = serde_json::from_value::<Vec<(String, u32)>>(
                        final_field.get("thumbnails").unwrap().clone(),
                    )?;
                    let filter_type = if final_field.get("is_quality").unwrap().as_bool().unwrap() {
                        Triangle
                    } else {
                        Nearest
                    };
                    if !thumbnails.is_empty() {
                        let mut img = image::open(f_path)?;
                        for max_size in thumbnails.iter() {
                            let thumbnail_size: (f64, f64) = Self::calculate_thumbnail_size(
                                image_data.width,
                                image_data.height,
                                max_size.1 as f64,
                            );
                            if thumbnail_size.0 > 0.0 && thumbnail_size.1 > 0.0 {
                                let width = thumbnail_size.0;
                                let height = thumbnail_size.1;
                                let thumb_name = format!("{}.{extension}", max_size.0);
                                let thumb_path = format!("{img_dir_path}/{thumb_name}");
                                let thumb_url = format!("{img_dir_url}/{thumb_name}");
                                img = img.resize_exact(width as u32, height as u32, filter_type);
                                match max_size.0.as_str() {
                                    "lg" => {
                                        img.save(thumb_path.clone())?;
                                        image_data.path_lg = thumb_path;
                                        image_data.url_lg = thumb_url;
                                    }
                                    "md" => {
                                        img.save(thumb_path.clone())?;
                                        image_data.path_md = thumb_path;
                                        image_data.url_md = thumb_url;
                                    }
                                    "sm" => {
                                        img.save(thumb_path.clone())?;
                                        image_data.path_sm = thumb_path;
                                        image_data.url_sm = thumb_url;
                                    }
                                    "xs" => {
                                        img.save(thumb_path.clone())?;
                                        image_data.path_xs = thumb_path;
                                        image_data.url_xs = thumb_url;
                                    }
                                    _ => Err(format!(
                                        "Model: `{model_name}` > Field: `{field_name}` > \
                                            Type: `ImageData` ; Method: `check()` => \
                                            Valid size names - `xs`, `sm`, `md`, `lg`."
                                    ))?,
                                }
                            };
                        }
                    }
                    // Insert result.
                    if !ignore_fields.contains(field_name) {
                        // Add file data to controller.
                        *final_field.get_mut("value").unwrap() = serde_json::to_value(image_data)?;
                        //
                        if !is_err_symptom {
                            let value = final_field.get("value").unwrap();
                            let field_value_bson = to_bson(value)?;
                            final_doc.insert(field_name, field_value_bson);
                        }
                    } else {
                        *final_field.get_mut("value").unwrap() = json!(null);
                    }
                }
                // Validation of number type fields.
                // *********************************************************************************
                //  "I32"
                10 => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get clean data.
                    let curr_val = i32::try_from(const_value.as_i64().unwrap())?;
                    //
                    if final_field["input_type"].as_str().unwrap() == "radio"
                        && !choice_i32_map.get(field_name).unwrap().contains(&curr_val)
                    {
                        is_err_symptom = true;
                        Self::accumula_err(final_field, &t!("not_match_choices"));
                        continue;
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Int32(curr_val);
                    // Validation of `unique`
                    let unique = final_field.get("unique");
                    if let Some(unique) = unique {
                        let is_unique = unique.as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .await
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        Self::accumula_err(final_field, &err.to_string());
                                    } else {
                                        Err(format!(
                                            "Model: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}",
                                            model_name, field_name, err
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Compare with `min`.
                    if let Some(min) = final_field.get("min") {
                        if !min.is_null() && curr_val < i32::try_from(min.as_i64().unwrap())? {
                            is_err_symptom = true;
                            Self::accumula_err(
                                final_field,
                                &t!("number_not_less_min", curr_num = curr_val, min_num = min),
                            );
                        }
                    }
                    // Compare with `max`.
                    if let Some(max) = final_field.get("max") {
                        if !max.is_null() && curr_val > i32::try_from(max.as_i64().unwrap())? {
                            is_err_symptom = true;
                            Self::accumula_err(
                                final_field,
                                &t!("number_not_greater_max", curr_num = curr_val, max_num = max),
                            );
                        }
                    }

                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                // "U32" | "I64"
                11 => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            Self::accumula_err(final_field, &t!("required_field"));
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get clean data.
                    let curr_val = const_value.as_i64().unwrap();
                    //
                    if final_field["input_type"].as_str().unwrap() == "radio"
                        && !choice_i64_map.get(field_name).unwrap().contains(&curr_val)
                    {
                        is_err_symptom = true;
                        Self::accumula_err(final_field, "Value does not match possible choices.");
                        continue;
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Int64(curr_val);
                    // Validation of `unique`.
                    let unique = final_field.get("unique");
                    if let Some(unique) = unique {
                        let is_unique = unique.as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .await
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        Self::accumula_err(final_field, &err.to_string());
                                    } else {
                                        Err(format!(
                                            "Model: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}",
                                            model_name, field_name, err
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Compare with `min`.
                    if let Some(min) = final_field.get("min") {
                        if !min.is_null() && curr_val < min.as_i64().unwrap() {
                            is_err_symptom = true;
                            let msg = format!(
                                "The number `{curr_val}` must not be less than min=`{min}`."
                            );
                            Self::accumula_err(final_field, &msg);
                        }
                    }
                    // Compare with `max`.
                    if let Some(max) = final_field.get("max") {
                        if !max.is_null() && curr_val > max.as_i64().unwrap() {
                            is_err_symptom = true;
                            let msg = format!(
                                "The number `{curr_val}` must not be greater than max=`{max}`."
                            );
                            Self::accumula_err(final_field, &msg);
                        }
                    }
                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                // "F64"
                12 => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            if !is_hide {
                                Self::accumula_err(final_field, "Required field.");
                            }
                        }
                        if is_save && !ignore_fields.contains(field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    if is_save && is_use_default {
                        *final_field.get_mut("value").unwrap() = const_value.clone();
                    }
                    // Get clean data.
                    let curr_val = const_value.as_f64().unwrap();
                    //
                    if final_field["input_type"].as_str().unwrap() == "radio"
                        && !choice_f64_map.get(field_name).unwrap().contains(&curr_val)
                    {
                        is_err_symptom = true;
                        Self::accumula_err(final_field, "Value does not match possible choices.");
                        continue;
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Double(curr_val);
                    // Validation of `unique`.
                    let unique = final_field.get("unique");
                    if let Some(unique) = unique {
                        let is_unique = unique.as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .await
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        Self::accumula_err(final_field, &err.to_string());
                                    } else {
                                        Err(format!(
                                            "Model: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}",
                                            model_name, field_name, err
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Compare with `min`.
                    if let Some(min) = final_field.get("min") {
                        if !min.is_null() && curr_val < min.as_f64().unwrap() {
                            is_err_symptom = true;
                            let msg = format!(
                                "The number `{curr_val}` must not be less than min=`{min}`."
                            );
                            Self::accumula_err(final_field, &msg);
                        }
                    }
                    // Compare with `max`.
                    if let Some(max) = final_field.get("max") {
                        if !max.is_null() && curr_val > max.as_f64().unwrap() {
                            is_err_symptom = true;
                            let msg = format!(
                                "The number `{curr_val}` must not be greater than max=`{max}`."
                            );
                            Self::accumula_err(final_field, &msg);
                        }
                    }
                    // Insert result.
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }

                // Validation of boolean type fields.
                // *********************************************************************************
                // "Bool"
                13 => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if const_value.is_null() && is_required {
                        is_err_symptom = true;
                        Self::accumula_err(final_field, "Required field.");
                        continue;
                    }

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if is_save && !is_err_symptom && !ignore_fields.contains(field_name) {
                        let is_checked = if !const_value.is_null() {
                            const_value.as_bool().unwrap()
                        } else {
                            false
                        };
                        *final_field.get_mut("checked").unwrap() = json!(is_checked);
                        let field_value_bson = Bson::Boolean(is_checked);
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                _ => Err(format!(
                    "Model: `{model_name}` > Field: `{field_name}` ; \
                    Method: `check()` => Unsupported field type - `{field_type}`."
                ))?,
            }
        }

        // Insert or update fields for timestamps `created_at` and `updated_at`.
        // -------------------------------------------------------------------------------------
        if !is_err_symptom {
            let dt = Utc::now();
            let dt_text = dt.format("%Y-%m-%dT%H:%M:%S%z").to_string();
            if is_update {
                // Get the `created_at` value from the database.
                let doc = {
                    let object_id = ObjectId::parse_str(hash)?;
                    let filter = doc! {"_id": object_id};
                    coll.find_one(filter, None).await?.unwrap()
                };
                let dt2 = doc.get("created_at").unwrap();
                let dt_text2 = dt2
                    .as_datetime()
                    .unwrap()
                    .to_chrono()
                    .format("%Y-%m-%dT%H:%M:%S%z")
                    .to_string();
                //
                *final_model_json
                    .get_mut("created_at")
                    .unwrap()
                    .get_mut("value")
                    .unwrap() = json!(dt_text2);
                self.set_created_at(dt_text2);
                // For update.
                if is_save {
                    final_doc.insert("created_at", dt2);
                    final_doc.insert("updated_at", Bson::DateTime(dt.into()));
                    *final_model_json
                        .get_mut("updated_at")
                        .unwrap()
                        .get_mut("value")
                        .unwrap() = json!(dt_text);
                    self.set_updated_at(dt_text);
                } else {
                    let dt = doc.get("updated_at").unwrap();
                    let dt_text = dt
                        .as_datetime()
                        .unwrap()
                        .to_chrono()
                        .format("%Y-%m-%dT%H:%M:%S%z")
                        .to_string();
                    if is_save {
                        final_doc.insert("updated_at", dt);
                    }
                    *final_model_json
                        .get_mut("updated_at")
                        .unwrap()
                        .get_mut("value")
                        .unwrap() = json!(dt_text);
                    self.set_updated_at(dt_text);
                }
            } else if is_save {
                // For create.
                final_doc.insert("created_at", Bson::DateTime(dt.into()));
                final_doc.insert("updated_at", Bson::DateTime(dt.into()));
                self.set_created_at(dt_text.clone());
                self.set_updated_at(dt_text.clone());
                *final_model_json
                    .get_mut("created_at")
                    .unwrap()
                    .get_mut("value")
                    .unwrap() = json!(dt_text);
                *final_model_json
                    .get_mut("updated_at")
                    .unwrap()
                    .get_mut("value")
                    .unwrap() = json!(dt_text);
            }
        }

        // If the validation is negative, delete the orphaned files.
        if is_save && is_err_symptom && !is_update {
            for field_name in fields_name.iter() {
                let field = final_model_json.get(field_name).unwrap();
                let field_type = field.get("field_type").unwrap().as_str().unwrap();
                //
                if field_type == "File" {
                    let value = field.get("value").unwrap();
                    let default = field.get("default").unwrap();
                    if !value.is_null() {
                        let file_data = serde_json::from_value::<FileData>(value.clone())?;
                        let file_data_default = if !default.is_null() {
                            serde_json::from_value::<FileData>(default.clone())?
                        } else {
                            FileData::default()
                        };
                        // Exclude files by default.
                        if file_data.path != file_data_default.path {
                            let path = Path::new(&file_data.path);
                            if path.is_file() {
                                fs::remove_file(path)?;
                            }
                            //
                            *final_model_json
                                .get_mut(field_name)
                                .unwrap()
                                .get_mut("value")
                                .unwrap() = json!(null);
                        }
                    }
                } else if field_type == "Image" {
                    let value = field.get("value").unwrap();
                    let default = field.get("default").unwrap();
                    if !value.is_null() {
                        let img_data = serde_json::from_value::<ImageData>(value.clone())?;
                        let img_data_default = if !default.is_null() {
                            serde_json::from_value::<ImageData>(default.clone())?
                        } else {
                            ImageData::default()
                        };
                        // Exclude files by default.
                        if img_data.path != img_data_default.path {
                            let dir_path = Path::new(&img_data.path).parent().unwrap();
                            if dir_path.is_dir() {
                                fs::remove_dir_all(dir_path)?;
                            }
                            //
                            *final_model_json
                                .get_mut(field_name)
                                .unwrap()
                                .get_mut("value")
                                .unwrap() = json!(null);
                        }
                    }
                }
            }
        }

        // Enrich the controller map with values for dynamic controllers.
        if is_save {
            Self::injection(
                client,
                app_name.as_str(),
                unique_app_key.as_str(),
                collection_name.as_str(),
                &mut final_model_json,
                &fields_name,
            )
            .await?;
        }

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputData2 {
            is_valid: !is_err_symptom,
            final_doc: Some(final_doc),
            final_model_json,
            fields_name: fields_name.clone(),
        })
    }

    /// Save to database as a new document or update an existing document.
    /// Hint: Used in conjunction with the `check()` method.
    ///
    /// # Example:
    ///
    /// ```
    /// use chrono::Local;
    /// let tz = Some(Local::now().format("%z").to_string()); // or None
    ///
    /// let mut model_name = ModelName::new()?;
    /// let output_data = model_name.save(&client, &tz, None, None).await?;
    /// if !output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    // *********************************************************************************************
    async fn save(
        &mut self,
        client: &Client,
        options_insert: Option<InsertOneOptions>,
        options_update: Option<UpdateOptions>,
    ) -> Result<OutputData2, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        //
        let mut stop_step: u8 = 1;
        //
        for step in 1_u8..=2_u8 {
            // Get checked data from the `check()` method.
            let mut verified_data = self.check(client, Some((true, step == 2))).await?;
            let is_no_error: bool = verified_data.is_valid();
            let final_doc = verified_data.get_doc().unwrap();
            let is_update: bool = !self.hash().is_empty();
            let (collection_name, is_use_hash_slug, database_name) = {
                // Get a key to access the metadata store.
                let key = Self::key()?;
                // Get metadata store.
                let store = META_STORE.lock().await;
                // Get metadata of Model.
                if let Some(meta) = store.get(&key) {
                    (
                        meta.collection_name.clone(),
                        meta.is_use_hash_slug,
                        meta.database_name.clone(),
                    )
                } else {
                    Err(format!(
                        "Model key: `{key}` ; Method: `save()` => \
                    Failed to get data from cache.",
                    ))?
                }
            };
            //
            let coll = client
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Having fields with a controller of Slug type.
            if !is_update && is_no_error && is_use_hash_slug {
                stop_step = 2;
            }
            // Save to database.
            // -------------------------------------------------------------------------------------
            if is_no_error {
                let hash_line;
                if is_update {
                    // Update document.
                    hash_line = self.hash();
                    let object_id = ObjectId::parse_str(hash_line.as_str())?;
                    let query = doc! {"_id": object_id};
                    let update = doc! {
                        "$set": final_doc.clone(),
                    };
                    // Run hook.
                    self.pre_update(client).await;
                    // Update doc.
                    coll.update_one(query, update, options_update.clone())
                        .await?;
                    // Run hook.
                    self.post_update(client).await;
                } else {
                    // Run hook.
                    self.pre_create(client).await;
                    // Create document.
                    let result: InsertOneResult = coll
                        .insert_one(final_doc.clone(), options_insert.clone())
                        .await?;
                    // Get hash-line.
                    hash_line = result.inserted_id.as_object_id().unwrap().to_hex();
                    // Add hash-line to model instance.
                    self.set_hash(hash_line.clone());
                    // Run hook.
                    self.post_create(client).await;
                }
                // Mute document.
                verified_data.set_doc(None);
                // Add hash-line to final_model_json.
                verified_data.set_hash(hash_line);
            }

            // Return result.
            // -------------------------------------------------------------------------------------
            if step == stop_step {
                return Ok(verified_data);
            }
        }
        //
        Err(format!(
            "Model key: `{}` > Method: `save()` => \
            !!!-Stub-!!!",
            Self::key()?
        ))?
    }

    /// Remove document from collection.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let mut user = User{...};
    /// let output_data = user.delete(&client, None).await?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    ///
    ///
    async fn delete(
        &self,
        client: &Client,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (model_name, database_name, collection_name, fields_name, is_del_doc) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.model_name.clone(),
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.fields_name.clone(),
                    meta.is_del_doc,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `delete()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = is_del_doc;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            String::from("It is forbidden to perform delete.")
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll = client
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Get Model hash  for ObjectId.
            let hash = self.hash();
            if hash.is_empty() {
                Err(format!(
                    "Model: `{model_name}` > Field: `hash` => \
                    An empty `hash` field is not allowed when deleting."
                ))?
            }
            let object_id = ObjectId::parse_str(hash.as_str())?;
            // Create query.
            let query = doc! {"_id": object_id};
            // Removeve files
            if let Some(document) = coll.find_one(query.clone(), None).await? {
                let model_json = self.self_to_json_val()?;
                //
                for field_name in fields_name.iter() {
                    if !document.is_null(field_name) {
                        let field = model_json.get(field_name).unwrap();
                        let field_type = field.get("field_type").unwrap().as_str().unwrap();
                        //
                        if field_type == "File" {
                            if let Some(info_file) = document.get(field_name).unwrap().as_document()
                            {
                                let path = info_file.get_str("path")?;
                                let default = field.get("default").unwrap();
                                //
                                let file_data_default = if !default.is_null() {
                                    serde_json::from_value::<FileData>(default.clone())?
                                } else {
                                    FileData::default()
                                };
                                // Exclude files by default.
                                if path != file_data_default.path {
                                    let path = Path::new(path);
                                    if path.is_file() {
                                        fs::remove_file(path)?;
                                    }
                                }
                            } else {
                                Err(format!(
                                    "Model: `{model_name}` > Field: `{field_name}` > \
                                    Method: `delete()` => Document (info file) not found."
                                ))?
                            }
                        } else if field_type == "Image" {
                            if let Some(info_file) = document.get(field_name).unwrap().as_document()
                            {
                                let path = info_file.get_str("path")?;
                                let default = field.get("default").unwrap();
                                //
                                let img_data_default = if !default.is_null() {
                                    serde_json::from_value::<ImageData>(default.clone())?
                                } else {
                                    ImageData::default()
                                };
                                // Exclude files by default.
                                if path != img_data_default.path {
                                    let dir_path = Path::new(path).parent().unwrap();
                                    if dir_path.is_dir() {
                                        fs::remove_dir_all(dir_path)?;
                                    }
                                }
                            } else {
                                Err(format!(
                                    "Model: `{model_name}` > Field: `{field_name}` > \
                                    Method: `delete()` => Document (info file) not found."
                                ))?
                            }
                        }
                    }
                }
            } else {
                Err(format!(
                    "Model: `{model_name}` ; Method: `delete()` => Document not found."
                ))?
            }
            // Run hook.
            self.pre_delete(client).await;
            // Execute query.
            coll.delete_one(query, options).await.is_ok()
        } else {
            false
        };
        // Run hook.
        if result_bool && err_msg.is_empty() {
            self.post_delete(client).await;
        }
        //
        let deleted_count = u64::from(result_bool);
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    // Operations with passwords.
    // *********************************************************************************************
    /// Generate password hash and add to result document.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User::new().await?;
    /// let password = user.password.get().unwrap();
    /// println!("{}", user.create_password_hash(&password)?);
    /// ```
    ///
    fn create_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@#$%^&+=*!~)(";
        const SALT_LEN: usize = 12;
        let mut rng = rand::thread_rng();
        let password: &[u8] = password.as_bytes();
        let salt: String = (0..SALT_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let salt: &[u8] = salt.as_bytes();
        let config = argon2::Config::default();
        let hash: String = argon2::hash_encoded(password, salt, &config)?;
        Ok(hash)
    }

    /// Match the password from the user to the password in the database.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User {...};
    /// let password = "12345678";
    /// assert!(user.create_password_hash(&client, password, None).await?);
    /// ```
    ///
    async fn verify_password(
        &self,
        client: &Client,
        password: &str,
        options: Option<FindOneOptions>,
    ) -> Result<bool, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, model_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.model_name.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `verify_password()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access the collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Get hash-line of Model.
        let hash = self.hash();
        if hash.is_empty() {
            Err(format!(
                "Model: `{model_name}` ; Method: `verify_password` => \
                An empty `hash` field is not allowed when updating."
            ))?
        }
        // Convert hash-line to ObjectId.
        let object_id = ObjectId::parse_str(hash.as_str())?;
        // Create a filter to search for a document.
        let filter = doc! {"_id": object_id};
        // An attempt to find the required document.
        let doc = coll.find_one(filter, options).await?;
        // We check that for the given `hash` a document is found in the database.
        if doc.is_none() {
            Err(format!(
                "Model: `{model_name}` ; Method: `verify_password` => \
                There is no document in the database for the current `hash` value."
            ))?
        }
        //
        let doc = doc.unwrap();
        // Check for the presence of the `password` field.
        let password_hash = doc.get("password");
        if password_hash.is_none() {
            Err(format!(
                "Model: `{model_name}` ; Method: `verify_password` => \
                The `password` field is missing."
            ))?
        }
        // Get password hash or empty string.
        let password_hash = password_hash.unwrap();
        //
        let password_hash = if password_hash.element_type() != ElementType::Null {
            password_hash.as_str().unwrap()
        } else {
            ""
        };
        // Password verification.
        Ok(argon2::verify_encoded(password_hash, password.as_bytes())?)
    }

    /// For replace or recover password.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User {...};
    /// let old_password = "12345678";
    /// // Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (
    /// // Size: 8-256
    /// let new_password = "UUbd+5KXw^756*uj";
    /// let output_data = user.update_password(&client, old_password, new_password, None, None).await?;
    /// if !output_data.is_valid()? {
    ///     println!("{}", output_data.err_msg()?);
    /// }
    /// ```
    ///
    async fn update_password(
        &self,
        client: &Client,
        old_password: &str,
        new_password: &str,
        options_find_old: Option<FindOneOptions>,
        options_update: Option<UpdateOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let mut result_bool = false;
        let mut err_msg = String::new();
        // Validation current password.
        if !self
            .verify_password(client, old_password, options_find_old)
            .await?
        {
            err_msg = String::from("The old password does not match.");
        } else {
            let (database_name, collection_name) = {
                // Get a key to access the metadata store.
                let key = Self::key()?;
                // Get metadata store.
                let store = META_STORE.lock().await;
                // Get metadata of Model.
                if let Some(meta) = store.get(&key) {
                    (meta.database_name.clone(), meta.collection_name.clone())
                } else {
                    Err(format!(
                        "Model key: `{key}` ; Method: `verify_password()` => \
                        Failed to get data from cache.",
                    ))?
                }
            };
            // Access the collection.
            let coll = client
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Get hash-line of Model.
            let hash = self.hash();
            // Convert hash-line to ObjectId.
            let object_id = ObjectId::parse_str(hash.as_str())?;
            // Create a filter to search for a document.
            let query = doc! {"_id": object_id};
            let new_password_hash = Self::create_password_hash(new_password)?;
            let doc = doc! {"password": new_password_hash};
            let update = doc! {
                "$set": doc,
            };
            // Update password.
            result_bool = coll
                .update_one(query, update, options_update)
                .await?
                .modified_count
                == 1;
            if !result_bool {
                err_msg = "An error occurred while updating the password.".to_string();
            }
        }
        //
        Ok(OutputData::UpdatePassword((result_bool, err_msg)))
    }
}
