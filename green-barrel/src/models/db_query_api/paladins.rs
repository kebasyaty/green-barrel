//! Query methods for a Model instance.
use mongodb::{
    bson::{doc, document::Document, oid::ObjectId, spec::ElementType, Bson},
    options::{DeleteOptions, FindOneOptions, InsertOneOptions, UpdateOptions},
    results::InsertOneResult,
    sync::Collection,
};
use rand::Rng;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{json, Value};
use slug::slugify;
use std::{convert::TryFrom, error::Error, fs, path::Path};
use uuid::Uuid;

use crate::models::{
    caching::Caching,
    helpers::{FileData, ImageData, Meta},
    hooks::Hooks,
    output_data::{OutputData, OutputDataCheck},
    validation::{AdditionalValidation, Validation},
    Main,
};

pub trait QPaladins: Main + Caching + Hooks + Validation + AdditionalValidation {
    /// Deleting a file in the database and in the file system.
    // *********************************************************************************************
    fn delete_file(
        &self,
        coll: &Collection,
        model_name: &str,
        field_name: &str,
        file_default: Option<FileData>,
        image_default: Option<ImageData>,
    ) -> Result<(), Box<dyn Error>> {
        //
        let hash = self.get_hash();
        if !hash.is_empty() {
            let object_id = ObjectId::with_string(hash.as_str())?;
            let filter = doc! {"_id": object_id};
            if let Some(document) = coll.find_one(filter.clone(), None)? {
                // If `is_deleted=true` was passed incorrectly.
                if document.is_null(field_name) {
                    return Ok(());
                }
                // Delete the file information in the database.
                let file_doc = doc! {field_name: Bson::Null};
                let update = doc! { "$set": file_doc };
                coll.update_one(filter, update, None)?;
                // Delete the orphaned file.
                if let Some(info_file) = document.get(field_name).unwrap().as_document() {
                    if let Some(file_default) = file_default {
                        let path_default = file_default.path;
                        let path = info_file.get_str("path")?;
                        if path != path_default {
                            let path = Path::new(path);
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                        }
                    } else if let Some(image_default) = image_default {
                        let path_default = image_default.path;
                        let path = info_file.get_str("path")?;
                        if path != path_default {
                            let path = Path::new(path);
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                            // Remove thumbnails.
                            let size_names: [&str; 4] = ["lg", "md", "sm", "xs"];
                            for size_name in size_names {
                                let key_name = format!("path_{}", size_name);
                                let path = info_file.get_str(key_name.as_str())?;
                                if !path.is_empty() {
                                    let path = Path::new(path);
                                    if path.exists() {
                                        fs::remove_file(path)?;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    Err(format!(
                        "Model: `{}` > Field: `{}` ; Method: `delete_file()` => \
                            Document (info file) not found.",
                        model_name, field_name
                    ))?
                }
            } else {
                Err(format!(
                    "Model: `{}` > Field: `{}` ; Method: `delete_file()` => \
                        Document not found.",
                    model_name, field_name
                ))?
            }
        }
        //
        Ok(())
    }

    /// Get file info from database.
    // *********************************************************************************************
    fn db_get_file_info(
        &self,
        coll: &Collection,
        field_name: &str,
    ) -> Result<Value, Box<dyn Error>> {
        //
        let hash = self.get_hash();
        if !hash.is_empty() {
            let object_id = ObjectId::with_string(hash.as_str())?;
            let filter = doc! {"_id": object_id};
            if let Some(document) = coll.find_one(filter, None)? {
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
    fn calculate_thumbnail_size(width: u32, height: u32, max_size: u32) -> (u32, u32) {
        if width > height && width > max_size {
            return (
                max_size,
                (height as f32 * (max_size as f32 / width as f32)).floor() as u32,
            );
        } else if height > max_size {
            return (
                (width as f32 * (max_size as f32 / height as f32)).floor() as u32,
                max_size,
            );
        }
        (0, 0)
    }

    /// Checking the Model before queries the database.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name  = ModelName {...}
    /// let output_data = model_name.check(None)?;
    /// if !output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    fn check(&mut self, is_save: Option<bool>) -> Result<OutputDataCheck, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        //
        let is_save = is_save.unwrap_or(false);
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        // Get model name.
        let model_name: &str = meta.model_name.as_str();
        // Determines the mode of accessing the database (insert or update).
        let hash = &self.get_hash();
        let is_update: bool = !hash.is_empty();
        // User input error detection symptom.
        let mut is_err_symptom = false;
        // Get a list of fields that should not be included in the document.
        let ignore_fields = meta
            .ignore_fields
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<&str>>();
        // Access the collection.
        let coll: Collection = client_cache
            .database(&meta.database_name)
            .collection(&meta.collection_name);
        // Get preliminary data from model instance and use for final result.
        let mut final_model_json = self.self_to_json()?;
        // Document for the final result.
        let mut final_doc = Document::new();

        // Validation of field by attributes (maxlength, unique, min, max, etc...).
        // -----------------------------------------------------------------------------------------
        let fields_name = meta
            .fields_name
            .iter()
            .map(|item| item.as_str())
            .collect::<Vec<&str>>();

        // Apply additional validation.
        if meta.is_use_add_valid {
            let error_map = self.add_validation()?;
            if !error_map.is_empty() {
                is_err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if let Some(final_field) = final_model_json.get_mut(field_name) {
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, err_msg));
                    } else {
                        Err(format!(
                            "\n\nModel: `{}` ;  Method: `add_validation()` => \
                                The model has no field `{}`.\n\n",
                            model_name, field_name
                        ))?
                    }
                }
            }
        }

        // Loop over fields for validation.
        for field_name in fields_name {
            // Don't check the `hash` field.
            if field_name == "hash" {
                //
                if !is_update && !meta.is_add_docs {
                    if is_save {
                        is_err_symptom = true;
                    }
                    *final_model_json
                        .get_mut(field_name)
                        .unwrap()
                        .get_mut("alert")
                        .unwrap() = json!("It is forbidden to perform saves.");
                }
                if is_update && !meta.is_up_docs {
                    if is_save {
                        is_err_symptom = true;
                    }
                    *final_model_json
                        .get_mut(field_name)
                        .unwrap()
                        .get_mut("alert")
                        .unwrap() = json!("It is forbidden to perform updates.");
                }
                continue;
            }
            // Get values for validation.
            let final_field = final_model_json.get_mut(field_name).unwrap();
            // Define conditional constants.
            let const_value = if final_field.get("value").is_some() {
                final_field.get("value").unwrap().clone()
            } else {
                json!(null)
            };
            let const_default = if final_field.get("default").is_some() {
                final_field.get("default").unwrap().clone()
            } else {
                json!(null)
            };
            let is_required = final_field
                .get("required")
                .unwrap()
                .as_bool()
                .unwrap()
                .clone();
            let is_hide = final_field
                .get("is_hide")
                .unwrap()
                .as_bool()
                .unwrap()
                .clone();
            let field_type = final_field
                .get("field_type")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let field_type = field_type.as_str();

            // Field validation.
            match field_type {
                // Validation of Text type fields.
                // *********************************************************************************
                "RadioText" | "InputColor" | "InputEmail" | "InputPassword" | "InputPhone"
                | "InputText" | "HiddenHash" | "InputUrl" | "InputIP" | "InputIPv4"
                | "InputIPv6" | "TextArea" => {
                    // When updating, we skip field password type.
                    if is_update && field_type == "InputPassword" {
                        *final_field.get_mut("value").unwrap() = json!(null);
                        continue;
                    }

                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    let val_str = const_value.as_str();
                    //
                    if const_value.is_null() || (val_str.is_some() && val_str.unwrap().is_empty()) {
                        if field_type != "InputPassword" && !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    //
                    let curr_val = const_value.as_str().unwrap();
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = if field_type != "InputPassword" {
                        Bson::String(curr_val.to_string())
                    } else {
                        Bson::Null
                    };

                    // Validation field attribute `pattern`.
                    // -----------------------------------------------------------------------------
                    let pattern = final_field.get("pattern");
                    if pattern.is_some() {
                        Self::regex_pattern_validation(
                            curr_val,
                            pattern.unwrap().as_str().unwrap(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, &err.to_string()));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        });
                    }

                    // Validation in regular expression.
                    // Checking `minlength`.
                    // -----------------------------------------------------------------------------
                    let minlength = final_field.get("minlength");
                    if minlength.is_some() {
                        Self::check_minlength(
                            minlength.unwrap().as_i64().unwrap() as usize,
                            curr_val,
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, &err.to_string()));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        });
                    }
                    // Checking `maxlength`.
                    let maxlength = final_field.get("maxlength");
                    if maxlength.is_some() {
                        Self::check_maxlength(
                            maxlength.unwrap().as_i64().unwrap() as usize,
                            curr_val,
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, &err.to_string()));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        });
                    }

                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    let unique = final_field.get("unique");
                    if unique.is_some() {
                        let is_unique = unique.unwrap().as_bool().unwrap();
                        if field_type != "InputPassword" && is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, &err.to_string())
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}\n\n",
                                            model_name,
                                            field_name,
                                            err.to_string()
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }

                    // Validation in regular expression (email, password, etc...).
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(field_type, curr_val).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        if !is_hide {
                            *final_field.get_mut("error").unwrap() =
                                json!(Self::accumula_err(&final_field, &err.to_string()));
                        } else {
                            Err(format!(
                                "Model: `{}` > Field: `{}` ; Method: `check()` => {}",
                                model_name,
                                field_name,
                                err.to_string()
                            ))
                            .unwrap()
                        }
                    });

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        match field_type {
                            "InputPassword" => {
                                if !curr_val.is_empty() {
                                    if !is_update {
                                        // Generate password hash and add to result document.
                                        let password_hash: String =
                                            Self::create_password_hash(curr_val)?;
                                        final_doc.insert(field_name, Bson::String(password_hash));
                                    }
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
                "AutoSlug" => {
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
                    let tmp_model_json = self.self_to_json()?;
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
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, "Required field."));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                        Method: `check()` => \
                                        Hiding required fields is not allowed.\n\n",
                                    model_name, field_name
                                ))?
                            }
                        }
                        if !ignore_fields.contains(&field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    //
                    slug = slugify(slug);
                    *final_field.get_mut("value").unwrap() = json!(slug);
                    let field_value_bson = Bson::String(slug.clone());
                    // Validation of `unique`.
                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    let is_unique = final_field.get("unique").unwrap().as_bool().unwrap();
                    if is_unique {
                        Self::check_unique(hash, field_name, &field_value_bson, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, &err.to_string()));
                            });
                    }
                    // Insert result.
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                // Validation of date type fields.
                // *********************************************************************************
                "InputDate" | "InputDateTime" | "HiddenDateTime" => {
                    // Don't check the `created_at`and updated_at fields.
                    if field_name == "created_at" || field_name == "updated_at" {
                        continue;
                    }
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if const_value.is_null() {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    //
                    let curr_val = const_value.as_str().unwrap();

                    // Validation in regular expression.
                    // -----------------------------------------------------------------------------
                    if let Err(err) = Self::regex_validation(field_type, curr_val) {
                        is_err_symptom = true;
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &err.to_string()));
                        continue;
                    }

                    // Create Date and Time Object.
                    // -----------------------------------------------------------------------------
                    // Date to DateTime.
                    let dt_val: chrono::DateTime<chrono::Utc> = {
                        let val = if field_type == "InputDate" {
                            format!("{}T00:00", curr_val)
                        } else {
                            curr_val.to_string()
                        };
                        chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::parse_from_str(&val, "%Y-%m-%dT%H:%M")?,
                            chrono::Utc,
                        )
                    };
                    // Create dates for `min` and `max` attributes values to
                    // check, if the value of user falls within the range
                    // between these dates.
                    let min = final_field.get("min").unwrap().as_str().unwrap();
                    let max = final_field.get("max").unwrap().as_str().unwrap();
                    if !min.is_empty() && !max.is_empty() {
                        // Validation in regular expression (min).
                        if let Err(err) = Self::regex_validation(field_type, min) {
                            is_err_symptom = true;
                            *final_field.get_mut("error").unwrap() =
                                json!(Self::accumula_err(&final_field, &err.to_string()));
                            continue;
                        }
                        // Validation in regular expression (max).
                        if let Err(err) = Self::regex_validation(field_type, max) {
                            is_err_symptom = true;
                            *final_field.get_mut("error").unwrap() =
                                json!(Self::accumula_err(&final_field, &err.to_string()));
                            continue;
                        }
                        // Date to DateTime (min).
                        let dt_min: chrono::DateTime<chrono::Utc> = {
                            let min_val: String = if field_type == "InputDate" {
                                format!("{}T00:00", min)
                            } else {
                                min.to_string()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(&min_val, "%Y-%m-%dT%H:%M")?,
                                chrono::Utc,
                            )
                        };
                        // Date to DateTime (max).
                        let dt_max: chrono::DateTime<chrono::Utc> = {
                            let max_val: String = if field_type == "InputDate" {
                                format!("{}T00:00", max)
                            } else {
                                max.to_string()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(&max_val, "%Y-%m-%dT%H:%M")?,
                                chrono::Utc,
                            )
                        };
                        // Check hit in range (min <> max).
                        if dt_val < dt_min || dt_val > dt_max {
                            is_err_symptom = true;
                            *final_field.get_mut("error").unwrap() = json!(Self::accumula_err(
                                &final_field,
                                "Date out of range between `min` and` max`."
                            ));
                            continue;
                        }
                    }

                    // Create datetime in bson type.
                    // -----------------------------------------------------------------------------
                    let dt_val_bson = Bson::DateTime(dt_val);

                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    let is_unique = final_field.get("unique").unwrap().as_bool().unwrap();
                    if is_unique {
                        Self::check_unique(hash, field_name, &dt_val_bson, &coll).unwrap_or_else(
                            |err| {
                                is_err_symptom = true;
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, &err.to_string()));
                            },
                        );
                    }

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, dt_val_bson);
                    }
                }
                // Validation of `select` type fields.
                // *********************************************************************************
                "SelectText" | "SelectI32" | "SelectU32" | "SelectI64" | "SelectF64" => {
                    //
                    let check_enpty_str = const_value.as_str();
                    //
                    if const_value.is_null()
                        || (check_enpty_str.is_some() && check_enpty_str.unwrap().is_empty())
                    {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Get selected items.
                    final_doc.insert(
                        field_name,
                        match field_type {
                            "SelectText" => {
                                let val = const_value.as_str().unwrap();
                                Bson::String(val.to_string())
                            }
                            "SelectI32" => {
                                let val = i32::try_from(const_value.as_i64().unwrap())?;
                                Bson::Int32(val)
                            }
                            "SelectU32" | "SelectI64" => {
                                let val = const_value.as_i64().unwrap();
                                Bson::Int64(val)
                            }
                            "SelectF64" => {
                                let val = const_value.as_f64().unwrap();
                                Bson::Double(val)
                            }
                            _ => Err(format!(
                                "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                    Unsupported widget type - `{}`.\n\n",
                                model_name, field_name, field_type
                            ))?,
                        },
                    );
                }
                //
                "SelectTextDyn" | "SelectI32Dyn" | "SelectU32Dyn" | "SelectI64Dyn"
                | "SelectF64Dyn" => {
                    //
                    let check_enpty_str = const_value.as_str();
                    //
                    if const_value.is_null()
                        || (check_enpty_str.is_some() && check_enpty_str.unwrap().is_empty())
                    {
                        if is_required {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, "Required field."));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                        Field: `is_hide` = `true` ; Method: `check()` => \
                                        Hiding required fields is not allowed.\n\n",
                                    model_name, field_name, field_type
                                ))?
                            }
                        }
                        if !ignore_fields.contains(&field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    // Get selected items.
                    final_doc.insert(
                        field_name,
                        match field_type {
                            "SelectTextDyn" => {
                                let val = const_value.as_str().unwrap().to_string();
                                Bson::String(val)
                            }
                            "SelectI32Dyn" => {
                                let val = i32::try_from(const_value.as_i64().unwrap())?;
                                Bson::Int32(val)
                            }
                            "SelectU32Dyn" | "SelectI64Dyn" => {
                                let val = const_value.as_i64().unwrap();
                                Bson::Int64(val)
                            }
                            "SelectF64Dyn" => {
                                let val = const_value.as_f64().unwrap();
                                Bson::Double(val)
                            }
                            _ => Err(format!(
                                "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                    Unsupported widget type - `{}`.\n\n",
                                model_name, field_name, field_type
                            ))?,
                        },
                    );
                }
                //
                "SelectTextMult" | "SelectI32Mult" | "SelectU32Mult" | "SelectI64Mult"
                | "SelectF64Mult" => {
                    //
                    let check_enpty_arr = const_value.as_array();
                    //
                    if const_value.is_null()
                        || (check_enpty_arr.is_some() && check_enpty_arr.unwrap().is_empty())
                    {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Get selected items.
                    final_doc.insert(
                        field_name,
                        match field_type {
                            "SelectTextMult" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::String(item.as_str().unwrap().into()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectI32Mult" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| {
                                        Bson::Int32(i32::try_from(item.as_i64().unwrap()).unwrap())
                                    })
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectU32Mult" | "SelectI64Mult" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::Int64(item.as_i64().unwrap()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectF64Mult" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::Double(item.as_f64().unwrap()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            _ => Err(format!(
                                "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                    Unsupported widget type - `{}`.\n\n",
                                model_name, field_name, field_type
                            ))?,
                        },
                    );
                }
                //
                "SelectTextMultDyn" | "SelectI32MultDyn" | "SelectU32MultDyn"
                | "SelectI64MultDyn" | "SelectF64MultDyn" => {
                    //
                    let check_enpty_arr = const_value.as_array();
                    //
                    if const_value.is_null()
                        || (check_enpty_arr.is_some() && check_enpty_arr.unwrap().is_empty())
                    {
                        if is_required {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, "Required field."));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                        Field: `is_hide` = `true` ; Method: `check()` => \
                                        Hiding required fields is not allowed.\n\n",
                                    model_name, field_name, field_type
                                ))?
                            }
                        }
                        if !ignore_fields.contains(&field_name) {
                            final_doc.insert(field_name, Bson::Null);
                        }
                        continue;
                    }
                    // Get selected items.
                    final_doc.insert(
                        field_name,
                        match field_type {
                            "SelectTextMultDyn" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::String(item.as_str().unwrap().into()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectI32MultDyn" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| {
                                        Bson::Int32(i32::try_from(item.as_i64().unwrap()).unwrap())
                                    })
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectU32MultDyn" | "SelectI64MultDyn" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::Int64(item.as_i64().unwrap()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            "SelectF64MultDyn" => Bson::Array(
                                const_value
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|item| Bson::Double(item.as_f64().unwrap()))
                                    .collect::<Vec<Bson>>(),
                            ),
                            _ => Err(format!(
                                "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                    Unsupported widget type - `{}`.\n\n",
                                model_name, field_name, field_type
                            ))?,
                        },
                    );
                }
                // Validation of file type fields.
                // *********************************************************************************
                "InputFile" => {
                    // Get data for validation.
                    let mut file_data = if !const_value.is_null() {
                        serde_json::from_value::<FileData>(const_value.clone())?
                    } else {
                        FileData::default()
                    };
                    // Delete file.
                    if file_data.is_delete && is_update && !ignore_fields.contains(&field_name) {
                        if !is_required
                            || ((!file_data.path.is_empty() && !file_data.url.is_empty())
                                || !const_default.is_null())
                        {
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                Some(serde_json::from_value(const_default.clone())?),
                                None,
                            )?;
                        } else {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() = json!(Self::accumula_err(
                                    &final_field,
                                    "Upload a new file to delete the previous one."
                                ));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                        Field: `is_hide` = `true` ; Method: `check()` => \
                                        Upload a new file to delete the previous one.\n\n",
                                    model_name, field_name, field_type
                                ))?
                            }
                        }
                    }
                    // Get the current information about file from database.
                    let curr_file_info = self.db_get_file_info(&coll, field_name)?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if file_data.path.is_empty() && file_data.url.is_empty() {
                        if curr_file_info.is_null() {
                            if !const_default.is_null() {
                                *final_field.get_mut("value").unwrap() = const_default.clone();
                            } else {
                                if is_required {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, "Required field.")
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                                Field: `is_hide` = `true` ; Method: `check()` => \
                                                Hiding required fields is not allowed.\n\n",
                                            model_name, field_name, field_type
                                        ))?
                                    }
                                }
                                if !is_update && !ignore_fields.contains(&field_name) {
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
                    // Flags to check.
                    let is_emty_path = file_data.path.is_empty();
                    let is_emty_url = file_data.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` > Type: `FileData` ; Method: \
                            `check()` => Required `path` and `url` fields.\n\n",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let f_path = std::path::Path::new(file_data.path.as_str());
                    if !f_path.exists() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => File is missing - {}\n\n",
                            model_name, field_name, file_data.path
                        ))?
                    }
                    if !f_path.is_file() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => The path does not lead to a file - {}\n\n",
                            model_name, field_name, file_data.path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    file_data.size = u32::try_from(metadata.len())?;
                    // Get file name.
                    file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Insert result.
                    if !ignore_fields.contains(&field_name) {
                        // Add file data to widget.
                        *final_field.get_mut("value").unwrap() = serde_json::to_value(file_data)?;
                        //
                        if !is_err_symptom {
                            let value = final_field.get("value").unwrap();
                            let bson_field_value = mongodb::bson::ser::to_bson(value)?;
                            final_doc.insert(field_name, bson_field_value);
                        }
                    } else {
                        *final_field.get_mut("value").unwrap() = json!(null);
                    }
                }
                //
                "InputImage" => {
                    // Get data for validation.
                    let mut image_data = if !const_value.is_null() {
                        serde_json::from_value::<ImageData>(const_value.clone())?
                    } else {
                        ImageData::default()
                    };
                    // Delete file.
                    if image_data.is_delete && is_update && !ignore_fields.contains(&field_name) {
                        if !is_required
                            || ((!image_data.path.is_empty() && !image_data.url.is_empty())
                                || !const_default.is_null())
                        {
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                Some(serde_json::from_value(const_default.clone())?),
                                None,
                            )?;
                        } else {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() = json!(Self::accumula_err(
                                    &final_field,
                                    "Upload a new file to delete the previous one."
                                ));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                        Field: `is_hide` = `true` ; Method: `check()` => \
                                        Upload a new file to delete the previous one.\n\n",
                                    model_name, field_name, field_type
                                ))?
                            }
                        }
                    }
                    // Get the current information about file from database.
                    let curr_file_info = self.db_get_file_info(&coll, field_name)?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    let thumbnails = serde_json::from_value::<Vec<(String, u32)>>(
                        final_field.get("thumbnails").unwrap().clone(),
                    )?;
                    //
                    if image_data.path.is_empty() && image_data.url.is_empty() {
                        if curr_file_info.is_null() {
                            if !const_default.is_null() {
                                *final_field.get_mut("value").unwrap() = const_default.clone();
                                // Copy the default image to the default section.
                                if !thumbnails.is_empty() {
                                    let new_file_name = Uuid::new_v4().to_string();
                                    let path = Path::new(image_data.path.as_str());
                                    let parent = path.parent().unwrap().to_str().unwrap();
                                    let extension =
                                        path.extension().unwrap().to_str().unwrap().to_string();
                                    fs::create_dir_all(format!("{}/default", parent))?;
                                    let new_default_path = format!(
                                        "{}/default/{}.{}",
                                        parent, new_file_name, extension
                                    );
                                    fs::copy(
                                        Path::new(image_data.path.as_str()),
                                        Path::new(new_default_path.as_str()),
                                    )?;
                                    image_data.path = new_default_path;
                                    //
                                    let url = Path::new(image_data.url.as_str());
                                    let parent = url.parent().unwrap().to_str().unwrap();
                                    image_data.url = format!(
                                        "{}/default/{}.{}",
                                        parent, new_file_name, extension
                                    );
                                }
                            } else {
                                if is_required {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, "Required field.")
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                                Field: `is_hide` = `true` ; Method: `check()` => \
                                                Hiding required fields is not allowed.\n\n",
                                            model_name, field_name, field_type
                                        ))?
                                    }
                                }
                                if !is_update && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, Bson::Null);
                                }
                                continue;
                            }
                        } else {
                            *final_field.get_mut("value").unwrap() = curr_file_info;
                            continue;
                        }
                    }
                    // Flags to check.
                    let is_emty_path = image_data.path.is_empty();
                    let is_emty_url = image_data.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` > Type: `FileData` ; Method: \
                            `check()` => Required `path` and `url` fields.\n\n",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let f_path = std::path::Path::new(image_data.path.as_str());
                    if !f_path.exists() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => File is missing - {}\n\n",
                            model_name, field_name, image_data.path
                        ))?
                    }
                    if !f_path.is_file() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => The path does not lead to a file - {}\n\n",
                            model_name, field_name, image_data.path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    image_data.size = u32::try_from(metadata.len())?;
                    // Get file name
                    image_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Get image width and height.
                    let dimensions: (u32, u32) = image::image_dimensions(f_path)?;
                    image_data.width = dimensions.0;
                    image_data.height = dimensions.1;
                    // Generate sub-size images.
                    if !thumbnails.is_empty() {
                        let mut img = image::open(f_path)?;
                        for max_size in thumbnails.iter() {
                            let thumbnail_size: (u32, u32) = Self::calculate_thumbnail_size(
                                dimensions.0,
                                dimensions.1,
                                max_size.1,
                            );
                            if thumbnail_size.0 > 0 && thumbnail_size.1 > 0 {
                                let width = thumbnail_size.0;
                                let height = thumbnail_size.1;
                                let thumb_name = format!("{}_{}", max_size.0, image_data.name);
                                let thumb_path = image_data
                                    .path
                                    .clone()
                                    .replace(image_data.name.as_str(), thumb_name.as_str());
                                let thumb_url = image_data
                                    .url
                                    .clone()
                                    .replace(image_data.name.as_str(), thumb_name.as_str());
                                img = img.resize_exact(
                                    width,
                                    height,
                                    image::imageops::FilterType::Triangle,
                                );
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
                                        "\n\nModel: `{}` > Field: `{}` > Type: `ImageData` ; \
                                            Method: `check()` => Valid size names -\
                                            `xs`, `sm`, `md`, `lg`.\n\n",
                                        model_name, field_name
                                    ))?,
                                }
                            };
                        }
                    }
                    // Insert result.
                    if !ignore_fields.contains(&field_name) {
                        // Add file data to widget.
                        *final_field.get_mut("value").unwrap() = serde_json::to_value(image_data)?;
                        //
                        if !is_err_symptom {
                            let value = final_field.get("value").unwrap();
                            let field_value_bson = mongodb::bson::ser::to_bson(value)?;
                            final_doc.insert(field_name, field_value_bson);
                        }
                    } else {
                        *final_field.get_mut("value").unwrap() = json!(null);
                    }
                }
                // Validation of number type fields.
                // *********************************************************************************
                "RadioI32" | "NumberI32" | "RangeI32" => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if const_value.is_null() {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Get clean data.
                    let curr_val = i32::try_from(const_value.as_i64().unwrap())?;
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Int32(curr_val);
                    // Validation of `unique`
                    // -----------------------------------------------------------------------------
                    let unique = final_field.get("unique");
                    if unique.is_some() {
                        let is_unique = unique.unwrap().as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, &err.to_string())
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}\n\n",
                                            model_name,
                                            field_name,
                                            err.to_string()
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min = final_field.get("min").unwrap();
                    if !min.is_null() && curr_val < i32::try_from(min.as_i64().unwrap())? {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be less than min=`{}`.",
                            curr_val, min
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }
                    //
                    let max = final_field.get("max").unwrap();
                    if !max.is_null() && curr_val > i32::try_from(max.as_i64().unwrap())? {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be greater than max=`{}`.",
                            curr_val, max
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                "RadioU32" | "NumberU32" | "RangeU32" | "RadioI64" | "NumberI64" | "RangeI64" => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if const_value.is_null() {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Get clean data.
                    let curr_val = const_value.as_i64().unwrap();
                    // Check for unsigned type
                    if field_type.contains("U") && curr_val < 0 {
                        *final_field.get_mut("error").unwrap() = json!(Self::accumula_err(
                            &final_field,
                            "The number must not be less than zero."
                        ));
                        continue;
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Int64(curr_val);
                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    let unique = final_field.get("unique");
                    if unique.is_some() {
                        let is_unique = unique.unwrap().as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, &err.to_string())
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}\n\n",
                                            model_name,
                                            field_name,
                                            err.to_string()
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min = final_field.get("min").unwrap();
                    if !min.is_null() && curr_val < min.as_i64().unwrap() {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be less than min=`{}`.",
                            curr_val, min
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }
                    //
                    let max = final_field.get("max").unwrap();
                    if !max.is_null() && curr_val > max.as_i64().unwrap() {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be greater than max=`{}`.",
                            curr_val, max
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                "RadioF64" | "NumberF64" | "RangeF64" => {
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if const_value.is_null() {
                        if !const_default.is_null() {
                            *final_field.get_mut("value").unwrap() = const_default.clone();
                        } else {
                            if is_required {
                                is_err_symptom = true;
                                if !is_hide {
                                    *final_field.get_mut("error").unwrap() =
                                        json!(Self::accumula_err(&final_field, "Required field."));
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, field_type
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Get clean data.
                    let curr_val = const_value.as_f64().unwrap();
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = Bson::Double(curr_val);
                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    let unique = final_field.get("unique");
                    if unique.is_some() {
                        let is_unique = unique.unwrap().as_bool().unwrap();
                        if is_unique {
                            Self::check_unique(hash, field_name, &field_value_bson, &coll)
                                .unwrap_or_else(|err| {
                                    is_err_symptom = true;
                                    if !is_hide {
                                        *final_field.get_mut("error").unwrap() = json!(
                                            Self::accumula_err(&final_field, &err.to_string())
                                        );
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field: `{}` ; \
                                                Method: `check()` => {}\n\n",
                                            model_name,
                                            field_name,
                                            err.to_string()
                                        ))
                                        .unwrap()
                                    }
                                });
                        }
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min = final_field.get("min").unwrap();
                    if !min.is_null() && curr_val < min.as_f64().unwrap() {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be less than min=`{}`.",
                            curr_val, min
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }
                    //
                    let max = final_field.get("max").unwrap();
                    if !max.is_null() && curr_val > max.as_f64().unwrap() {
                        is_err_symptom = true;
                        let msg = format!(
                            "The number `{}` must not be greater than max=`{}`.",
                            curr_val, max
                        );
                        *final_field.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&final_field, &msg));
                    }
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, field_value_bson);
                    }
                }

                // Validation of boolean type fields.
                // *********************************************************************************
                "CheckBox" => {
                    // Get field value for validation.
                    let checked_json_val = final_field.get("checked").unwrap();
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if checked_json_val.is_null() {
                        if is_required {
                            is_err_symptom = true;
                            if !is_hide {
                                *final_field.get_mut("error").unwrap() =
                                    json!(Self::accumula_err(&final_field, "Required field."));
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` > Field type: {} > \
                                            Field: `is_hide` = `true` ; Method: `check()` => \
                                            Hiding required fields is not allowed.\n\n",
                                    model_name, field_name, field_type
                                ))?
                            }
                        }
                        continue;
                    }

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let checked = checked_json_val.as_bool().unwrap();
                        let field_value_bson = Bson::Boolean(checked);
                        final_doc.insert(field_name, field_value_bson);
                    }
                }
                _ => Err(format!(
                    "Model: `{}` > Field: `{}` ; Method: `check()` => \
                     Unsupported widget type - `{}`.",
                    model_name, field_name, field_type
                ))?,
            }

            // Insert or update fields for timestamps `created_at` and `updated_at`.
            // -------------------------------------------------------------------------------------
            if !is_err_symptom {
                let dt: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                let dt_text: String = dt.to_rfc3339()[..19].into();
                if is_update {
                    // For update.
                    if is_save {
                        final_doc.insert("updated_at", Bson::DateTime(dt));
                        *final_model_json
                            .get_mut("updated_at")
                            .unwrap()
                            .get_mut("value")
                            .unwrap() = json!(dt_text);
                        self.set_updated_at(dt_text);
                    }
                    // Get the `created_at` value from the database.
                    let doc = {
                        let object_id = ObjectId::with_string(hash)?;
                        let filter = doc! {"_id": object_id};
                        coll.find_one(filter, None)?.unwrap()
                    };
                    let dt = doc.get("created_at").unwrap();
                    let dt_text = dt.as_datetime().unwrap().to_rfc3339()[..19].to_string();
                    //
                    final_doc.insert("created_at", dt);
                    *final_model_json
                        .get_mut("created_at")
                        .unwrap()
                        .get_mut("value")
                        .unwrap() = json!(dt_text);
                    self.set_created_at(dt_text);
                } else if is_save {
                    // For create.
                    final_doc.insert("created_at", Bson::DateTime(dt));
                    final_doc.insert("updated_at", Bson::DateTime(dt));
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
        }

        // If the validation is negative, delete the orphaned files.
        if is_save && is_err_symptom && !is_update {
            for field_name in meta.fields_name.iter() {
                let field = final_model_json.get(field_name).unwrap();
                let field_type = field.get("field_type").unwrap().as_str().unwrap();
                //
                if field_type == "InputFile" {
                    let value = field.get("value").unwrap();
                    let default_value = field.get("default").unwrap();
                    if !value.is_null() && !default_value.is_null() {
                        let file_data = serde_json::from_value::<FileData>(value.clone())?;
                        let file_data_default =
                            serde_json::from_value::<FileData>(default_value.clone())?;
                        // Exclude files by default.
                        if file_data.path != file_data_default.path {
                            let path = Path::new(&file_data.path);
                            if path.exists() {
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
                } else if field_type == "InputImage" {
                    let value = field.get("value").unwrap();
                    let default_value = field.get("default").unwrap();
                    if !value.is_null() && !default_value.is_null() {
                        let img_data = serde_json::from_value::<ImageData>(value.clone())?;
                        let img_data_default =
                            serde_json::from_value::<ImageData>(default_value.clone())?;
                        // Exclude files by default.
                        if img_data.path != img_data_default.path {
                            let path = Path::new(&img_data.path);
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                            // Remove thumbnails.
                            let size_names: [&str; 4] = ["lg", "md", "sm", "xs"];
                            for size_name in size_names {
                                let path = match size_name {
                                    "lg" => img_data.path_lg.clone(),
                                    "md" => img_data.path_md.clone(),
                                    "sm" => img_data.path_sm.clone(),
                                    "xs" => img_data.path_xs.clone(),
                                    _ => Err("")?,
                                };
                                if !path.is_empty() {
                                    let path = Path::new(&path);
                                    if path.exists() {
                                        fs::remove_file(path)?;
                                    }
                                }
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

        // Enrich the widget map with values for dynamic widgets.
        Self::vitaminize(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            &client_cache,
            &mut final_model_json,
            &meta.fields_name,
        )?;

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputDataCheck::from(
            !is_err_symptom,
            Some(final_doc),
            final_model_json,
            meta.fields_name,
        ))
    }

    /// Save to database as a new document or update an existing document.
    /// Hint: Used in conjunction with the `check()` method.
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name  = ModelName {...}
    /// let output_data = model_name.save(None, None)?;
    /// if !output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    // *********************************************************************************************
    fn save<'a>(
        &mut self,
        options_insert: Option<InsertOneOptions>,
        options_update: Option<UpdateOptions>,
    ) -> Result<OutputDataCheck, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        //
        let mut stop_step: u8 = 0;
        //
        for num in 0_u8..=1_u8 {
            // Get checked data from the `check()` method.
            let mut verified_data = self.check(Some(true))?;
            let is_no_error: bool = verified_data.is_valid();
            let final_doc = verified_data.get_doc().unwrap();
            let mut final_model_json = verified_data.get_model_json();
            // Get cached Model data.
            let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
            // Get Model metadata.
            let meta: Meta = model_cache.meta;
            //
            let is_update: bool = !self.get_hash().is_empty();
            //
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Having fields with a widget of inputSlug type.
            if is_no_error && !is_update {
                let target_field_type = "AutoSlug";
                let hash = "hash";
                for (field_name, field_type) in meta.controller_type_map.iter() {
                    if field_type == &target_field_type {
                        let slug_sources = final_model_json
                            .get(field_name)
                            .unwrap()
                            .get("slug_sources")
                            .unwrap()
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|item| item.as_str().unwrap())
                            .collect::<Vec<&str>>();
                        if slug_sources.contains(&hash) {
                            stop_step = 1;
                            break;
                        }
                    }
                }
            }

            // Save to database.
            // -------------------------------------------------------------------------------------
            if is_no_error {
                let hash_line;
                if is_update {
                    // Update document.
                    hash_line = self.get_hash();
                    let object_id = ObjectId::with_string(hash_line.as_str())?;
                    let query = doc! {"_id": object_id.clone()};
                    let update = doc! {
                        "$set": final_doc.clone(),
                    };
                    // Run hook.
                    self.pre_update();
                    // Update doc.
                    coll.update_one(query, update, options_update.clone())?;
                    // Run hook.
                    self.post_update();
                } else {
                    // Run hook.
                    self.pre_create();
                    // Create document.
                    let result: InsertOneResult =
                        coll.insert_one(final_doc.clone(), options_insert.clone())?;
                    // Get hash-line.
                    hash_line = result.inserted_id.as_object_id().unwrap().to_hex();
                    // Add hash-line to model instance.
                    self.set_hash(hash_line.clone());
                    // Run hook.
                    self.post_create();
                }
                // Mute document.
                verified_data.set_doc(None);
                // Add hash-line to final widget map.
                *final_model_json
                    .get_mut("hash")
                    .unwrap()
                    .get_mut("value")
                    .unwrap() = json!(hash_line);
                verified_data.set_model_json(final_model_json);
            }

            // Return result.
            // -------------------------------------------------------------------------------------
            if num == stop_step {
                return Ok(verified_data);
            }
        }
        //
        let meta = Self::meta()?;
        Err(format!(
            "Model: `{}` > Method: `save()` => \
                !!!-Stub-!!!",
            meta.model_name
        ))?
    }

    /// Remove document from collection.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...}
    /// let output_data = model_name.delete(None)?;
    /// if !output_data.is_valid()? {
    ///     println!("{}", output_data.err_msg()?);
    /// }
    /// ```
    ///
    fn delete(&self, options: Option<DeleteOptions>) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
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
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Get Model hash  for ObjectId.
            let hash = self.get_hash();
            if hash.is_empty() {
                Err(format!(
                    "Model: `{}` > Field: `hash` => \
                        An empty `hash` field is not allowed when deleting.",
                    meta.model_name
                ))?
            }
            let object_id = ObjectId::with_string(hash.as_str())?;
            // Create query.
            let query = doc! {"_id": object_id};
            // Removeve files
            if let Some(document) = coll.find_one(query.clone(), None)? {
                let model_json = self.self_to_json()?;
                //
                for field_name in meta.fields_name.iter() {
                    if !document.is_null(field_name) {
                        let field = model_json.get(field_name).unwrap();
                        let field_type = field.get("field_type").unwrap().as_str().unwrap();
                        //
                        if field_type == "InputFile" {
                            if let Some(info_file) = document.get(field_name).unwrap().as_document()
                            {
                                let path = info_file.get_str("path")?;
                                let default_value = field.get("default").unwrap();
                                //
                                if !default_value.is_null() {
                                    let file_data_default =
                                        serde_json::from_value::<FileData>(default_value.clone())?;
                                    // Exclude files by default.
                                    if path != file_data_default.path {
                                        let path = Path::new(path);
                                        if path.exists() {
                                            fs::remove_file(path)?;
                                        }
                                    }
                                }
                            } else {
                                Err(format!(
                                    "Model: `{}` > Field: `{}` > \
                                        Method: `delete()` => Document (info file) not found.",
                                    meta.model_name, field_name
                                ))?
                            }
                        } else if field_type == "InputImage" {
                            if let Some(info_file) = document.get(field_name).unwrap().as_document()
                            {
                                let path = info_file.get_str("path")?;
                                let default_value = field.get("default").unwrap();
                                //
                                if !default_value.is_null() {
                                    let img_data_default =
                                        serde_json::from_value::<ImageData>(default_value.clone())?;
                                    // Exclude files by default.
                                    if path != img_data_default.path {
                                        let path = Path::new(path);
                                        if path.exists() {
                                            fs::remove_file(path)?;
                                        }
                                        // Remove thumbnails.
                                        let size_names: [&str; 4] = ["lg", "md", "sm", "xs"];
                                        for size_name in size_names.iter() {
                                            let key_name = format!("path_{}", size_name);
                                            let path = info_file.get_str(key_name.as_str())?;
                                            if !path.is_empty() {
                                                let path = Path::new(path);
                                                if path.exists() {
                                                    fs::remove_file(path)?;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                Err(format!(
                                    "Model: `{}` > Field: `{}` > \
                                        Method: `delete()` => Document (info file) not found.",
                                    meta.model_name, field_name
                                ))?
                            }
                        }
                    }
                }
            } else {
                Err(format!(
                    "Model: `{}` ; Method: `delete()` => Document not found.",
                    meta.model_name
                ))?
            }
            // Run hook.
            self.pre_delete();
            // Execute query.
            coll.delete_one(query, options).is_ok()
        } else {
            false
        };
        // Run hook.
        if result_bool && err_msg.is_empty() {
            self.post_delete();
        }
        //
        let deleted_count = if result_bool { 1_i64 } else { 0_i64 };
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
    /// let user = UserProfile {...};
    /// let field_value = user.password;
    /// println!("{}", user_profile.create_password_hash(field_value)?);
    /// ```
    ///
    fn create_password_hash(field_value: &str) -> Result<String, Box<dyn Error>> {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@#$%^&+=*!~)(";
        const SALT_LEN: usize = 12;
        let mut rng = rand::thread_rng();
        let password: &[u8] = field_value.as_bytes();
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
    /// let user_profile = UserProfile {...};
    /// let password = "12345678";
    /// assert!(user_profile.create_password_hash(password, None)?);
    /// ```
    ///
    fn verify_password(
        &self,
        password: &str,
        options: Option<FindOneOptions>,
    ) -> Result<bool, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        // Access the collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Get hash-line of Model.
        let hash = self.get_hash();
        if hash.is_empty() {
            Err(format!(
                "Model: `{}` ; Method: `verify_password` => \
                    An empty `hash` field is not allowed when updating.",
                meta.model_name
            ))?
        }
        // Convert hash-line to ObjectId.
        let object_id = ObjectId::with_string(hash.as_str())?;
        // Create a filter to search for a document.
        let filter = doc! {"_id": object_id};
        // An attempt to find the required document.
        let doc = coll.find_one(filter, options)?;
        // We check that for the given `hash` a document is found in the database.
        if doc.is_none() {
            Err(format!(
                "Model: `{}` ; Method: `verify_password` => \
                    There is no document in the database for the current `hash` value.",
                meta.model_name
            ))?
        }
        //
        let doc = doc.unwrap();
        // Check for the presence of the `password` field.
        let password_hash = doc.get("password");
        if password_hash.is_none() {
            Err(format!(
                "Model: `{}` ; Method: `verify_password` => \
                    The `password` field is missing.",
                meta.model_name
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
    /// let user = UserProfile {...};
    /// let old_password = "12345678";
    /// // Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (
    /// // Size: 8-256
    /// let new_password = "UUbd+5KXw^756*uj";
    /// let output_data = user.update_password(old_password, new_password, None)?;
    /// if !output_data.is_valid()? {
    ///     println!("{}", output_data.err_msg()?);
    /// }
    /// ```
    ///
    fn update_password(
        &self,
        old_password: &str,
        new_password: &str,
        options_find_old: Option<FindOneOptions>,
        options_update: Option<UpdateOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        //
        let mut result_bool = false;
        let mut err_msg = String::new();
        // Validation current password.
        if !self.verify_password(old_password, options_find_old)? {
            err_msg = String::from("The old password does not match.");
        } else {
            // Get cached Model data.
            let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
            // Get Model metadata.
            let meta: Meta = model_cache.meta;
            // Access the collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Get hash-line of Model.
            let hash = self.get_hash();
            // Convert hash-line to ObjectId.
            let object_id = ObjectId::with_string(hash.as_str())?;
            // Create a filter to search for a document.
            let query = doc! {"_id": object_id};
            let new_password_hash = Self::create_password_hash(new_password)?;
            let doc = doc! {"password": new_password_hash};
            let update = doc! {
                "$set": doc,
            };
            // Update password.
            result_bool = coll
                .update_one(query, update, options_update)?
                .modified_count
                == 1_i64;
            if !result_bool {
                err_msg = format!("An error occurred while updating the password.")
            }
        }
        //
        Ok(OutputData::UpdatePassword((result_bool, err_msg)))
    }
}
