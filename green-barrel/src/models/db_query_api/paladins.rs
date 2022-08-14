//! Query methods for a Model instance.

use mongodb::{
    bson::{doc, document::Document, oid::ObjectId, spec::ElementType, Bson},
    options::{DeleteOptions, FindOneOptions, InsertOneOptions, UpdateOptions},
    results::InsertOneResult,
    sync::Collection,
};
use rand::Rng;
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{json, value::Value};
use slug::slugify;
use std::{convert::TryFrom, error::Error, fs, path::Path};
use uuid::Uuid;

use crate::{
    helpers::structures::{FileData, ImageData},
    models::{
        caching::Caching,
        hooks::Hooks,
        output_data::{OutputData, OutputDataCheck},
        validation::{AdditionalValidation, Validation},
        Main, Meta,
    },
};

pub trait QPaladins: Main + Caching + Hooks + Validation + AdditionalValidation {
    /// Deleting a file in the database and in the file system.
    // *********************************************************************************************
    fn delete_file(
        &self,
        coll: &Collection,
        model_name: &str,
        field_name: &str,
        widget_default_value: &str,
        is_image: bool,
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
                    if is_image {
                        let default_path = if !widget_default_value.is_empty() {
                            serde_json::from_str::<ImageData>(widget_default_value)?.path
                        } else {
                            String::new()
                        };
                        let path = info_file.get_str("path")?;
                        if path != default_path {
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
                    } else {
                        let default_path = if !widget_default_value.is_empty() {
                            serde_json::from_str::<FileData>(widget_default_value)?.path
                        } else {
                            String::new()
                        };
                        let path = info_file.get_str("path")?;
                        if path != default_path {
                            let path = Path::new(path);
                            if path.exists() {
                                fs::remove_file(path)?;
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
    ) -> Result<String, Box<dyn Error>> {
        //
        let hash = self.get_hash();
        let mut result = String::new();
        if !hash.is_empty() {
            let object_id = ObjectId::with_string(hash.as_str())?;
            let filter = doc! {"_id": object_id};
            if let Some(document) = coll.find_one(filter, None)? {
                if let Some(doc) = document.get(field_name).unwrap().as_document() {
                    result = serde_json::to_string(doc)?;
                }
            }
        }
        //
        Ok(result)
    }

    /// Calculate the maximum size for a thumbnail.
    // *********************************************************************************************
    fn calculate_thumbnail_size(width: u32, height: u32, max_size: u32) -> (u32, u32) {
        if width > height {
            if width > max_size {
                return (
                    max_size,
                    (height as f32 * (max_size as f32 / width as f32)).floor() as u32,
                );
            }
        } else {
            if height > max_size {
                return (
                    (width as f32 * (max_size as f32 / height as f32)).floor() as u32,
                    max_size,
                );
            }
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
        let mut is_err_symptom =
            if is_save && ((!is_update && !meta.is_add_docs) || (is_update && !meta.is_up_docs)) {
                true
            } else {
                false
            };
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
        {
            let error_map = self.add_validation()?;
            if !error_map.is_empty() {
                is_err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if !fields_name.contains(&field_name) {
                        Err(format!(
                            "\n\nModel: `{}` ;  Method: `add_validation()` => \
                            The `{}` field is missing from the model.\n\n",
                            model_name, field_name
                        ))?
                    }
                    if let Some(widget) = final_model_json.get_mut(field_name) {
                        *widget.get_mut("error").unwrap() =
                            json!(Self::accumula_err(&widget, err_msg)?);
                    }
                }
            }
        }

        // Loop over fields for validation.
        for field_name in fields_name {
            //
            let mut final_widget = final_model_json.get_mut(field_name).unwrap();
            // Don't check the `hash` field.
            if field_name == "hash" {
                //
                if is_err_symptom {
                    if !meta.is_add_docs {
                        *final_widget.get_mut("alert").unwrap() =
                            json!("It is forbidden to perform saves.");
                    } else if !meta.is_up_docs {
                        *final_widget.get_mut("alert").unwrap() =
                            json!("It is forbidden to perform updates.");
                    }
                }
                continue;
            }
            // Get widget value for validation.
            let mut final_value = final_widget.get_mut("value").unwrap();
            let mut final_default = final_widget.get_mut("default").unwrap();
            let final_required = final_widget.get("required").unwrap().as_bool().unwrap();
            let final_is_hide = final_widget.get("is_hide").unwrap().as_bool().unwrap();
            let widget_name = final_widget.get("widget").unwrap().as_str().unwrap();

            // Field validation.
            match widget_name {
                // Validation of Text type fields.
                // *********************************************************************************
                "radioText" | "inputColor" | "inputEmail" | "inputPassword" | "inputPhone"
                | "inputText" | "inputUrl" | "inputIP" | "inputIPv4" | "inputIPv6" | "textArea"
                | "hiddenText" => {
                    // When updating, we skip field password type.
                    if is_update && widget_name == "inputPassword" {
                        *final_value = serde_json::Value::Null;
                        continue;
                    }

                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if final_value.is_null() {
                        if widget_name != "inputPassword" && !final_default.is_null() {
                            *final_value = final_default.clone();
                        } else {
                            if final_required {
                                is_err_symptom = true;
                                if !final_is_hide {
                                    *final_widget.get_mut("error").unwrap() = json!(
                                        Self::accumula_err(&final_widget, "Required field.")?
                                    );
                                } else {
                                    Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` > Widget: {} > \
                                        is_hide: `true` ; Method: `check()` => \
                                        Hiding required fields is not allowed.\n\n",
                                        model_name, field_name, widget_name
                                    ))?
                                }
                            }
                            if !ignore_fields.contains(&field_name) {
                                final_doc.insert(field_name, Bson::Null);
                            }
                            continue;
                        }
                    }
                    // Used to validation uniqueness and in the final result.
                    let field_value_bson = if widget_name != "inputPassword" {
                        Bson::String(final_value.as_str().unwrap().to_string())
                    } else {
                        Bson::Null
                    };

                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(field_value, final_widget.pattern.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                if !final_is_hide {
                                    final_widget.error =
                                        Self::accumula_err(&final_widget, &err.to_string())
                                            .unwrap();
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
                    // Checking `minlength`, `maxlength`, `min length`, `max length`.
                    // -----------------------------------------------------------------------------
                    Self::check_minlength(final_widget.minlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        },
                    );
                    Self::check_maxlength(final_widget.maxlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        },
                    );

                    // Validation of range (`min` <> `max`).
                    // Hint: The `validate_length()` method did not
                    // provide the desired result.
                    // -----------------------------------------------------------------------------
                    let min = final_widget.minlength.clone();
                    let max = final_widget.maxlength.clone();
                    let len = field_value.encode_utf16().count();
                    if max > 0_usize && (len < min || len > max) {
                        is_err_symptom = true;
                        let msg = format!(
                            "Length {} is out of range (min={} <-> max={}).",
                            len, min, max
                        );
                        if !widget_type.contains("hidden") && !final_widget.is_hide {
                            final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                        } else {
                            Err(format!(
                                "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => {}\n\n",
                                model_name, field_name, msg
                            ))?
                        }
                    }

                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    if widget_type != "inputPassword" && final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                if !widget_type.contains("hidden") && !final_widget.is_hide {
                                    final_widget.error =
                                        Self::accumula_err(&final_widget, &err.to_string())
                                            .unwrap();
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

                    // Validation in regular expression (email, password, etc...).
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        if !widget_type.contains("hidden") && !final_widget.is_hide {
                            final_widget.error =
                                Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                        match widget_type {
                            "inputPassword" => {
                                if !field_value.is_empty() {
                                    if !is_update {
                                        // Generate password hash and add to result document.
                                        let password_hash: String =
                                            Self::create_password_hash(field_value)?;
                                        final_doc.insert(field_name, Bson::String(password_hash));
                                    }
                                }
                            }
                            _ => {
                                // Insert result from other fields.
                                final_doc.insert(field_name, bson_field_value);
                            }
                        }
                    }
                }
                // Validation of Slug type fields.
                // *********************************************************************************
                "inputSlug" => {
                    let mut slug_str = String::new();
                    for field in final_widget.slug_sources.iter() {
                        let value = pre_json.get(field).unwrap();
                        if value.is_string() {
                            let text = value.as_str().unwrap().trim().to_string();
                            slug_str = format!("{}-{}", slug_str, text);
                        } else if value.is_i64() {
                            let num = value.as_i64().unwrap();
                            slug_str = format!("{}-{}", slug_str, num);
                        } else if value.is_f64() {
                            let num = value.as_f64().unwrap();
                            slug_str = format!("{}-{}", slug_str, num);
                        }
                    }
                    //
                    if slug_str.is_empty() {
                        slug_str = if !pre_json_value.is_null() {
                            pre_json_value.as_str().unwrap().trim().to_string()
                        } else {
                            String::new()
                        };
                    }
                    // Validation, if the field is required and empty, accumulate the error.
                    if slug_str.is_empty() {
                        if !final_widget.value.is_empty() {
                            slug_str = final_widget.value.clone();
                        } else {
                            if final_widget.required {
                                is_err_symptom = true;
                                if !final_widget.is_hide {
                                    final_widget.error = Self::accumula_err(
                                        &final_widget,
                                        &"Required field.".to_owned(),
                                    )
                                    .unwrap();
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
                    }
                    //
                    slug_str = slugify(slug_str);
                    final_widget.value = slug_str.clone();
                    let bson_field_value = Bson::String(slug_str.clone());
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            slug_str.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Insert result.
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                // Validation of date type fields.
                // *********************************************************************************
                "inputDate" | "inputDateTime" => {
                    // Don't check the `created_at`and updated_at fields.
                    if field_name == "created_at" || field_name == "updated_at" {
                        continue;
                    }
                    // Get field value for validation.
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = clean_data.clone();
                        clean_data
                    } else {
                        String::new()
                    };

                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if !final_widget.value.is_empty() {
                            field_value = final_widget.value.clone();
                        } else {
                            if final_widget.required {
                                is_err_symptom = true;
                                if !final_widget.is_hide {
                                    final_widget.error = Self::accumula_err(
                                        &final_widget,
                                        &"Required field.".to_owned(),
                                    )
                                    .unwrap();
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
                    }
                    // Convert to &str
                    let field_value: &str = field_value.as_str();

                    // Validation in regular expression.
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                    if is_err_symptom {
                        continue;
                    }

                    // Create Date and Time Object.
                    // -----------------------------------------------------------------------------
                    // Date to DateTime.
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
                    // between these dates.
                    if !final_widget.min.is_empty() && !final_widget.max.is_empty() {
                        // Validation in regular expression (min).
                        Self::regex_validation(widget_type, final_widget.min.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        // Validation in regular expression (max).
                        Self::regex_validation(widget_type, final_widget.max.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        if is_err_symptom {
                            continue;
                        }
                        // Date to DateTime (min).
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
                        // Date to DateTime (max).
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
                        // Check hit in range (min <> max).
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

                    // Create datetime in bson type.
                    // -----------------------------------------------------------------------------
                    let dt_value_bson = Bson::DateTime(dt_value);
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

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, dt_value_bson);
                    }
                }
                // Validation of `select` type fields.
                // *********************************************************************************
                "selectText" | "selectI32" | "selectU32" | "selectI64" | "selectF64" => {
                    //
                    let mut tmp_json_text = serde_json::to_string(&pre_json_value)?;
                    let mut tmp_value;
                    // Get selected items.
                    for _ in 0..=1 {
                        // Get selected items.
                        if !pre_json_value.is_null() && !tmp_json_text.is_empty() {
                            final_doc.insert(
                                field_name,
                                match widget_type {
                                    "selectText" => {
                                        let val = pre_json_value.as_str().unwrap().to_string();
                                        final_widget.value = val.clone();
                                        if val.is_empty() && final_widget.required {
                                            is_err_symptom = true;
                                            final_widget.error = Self::accumula_err(
                                                &final_widget,
                                                &"Required field.".to_owned(),
                                            )
                                            .unwrap();
                                        }
                                        Bson::String(val)
                                    }
                                    "selectI32" => {
                                        let val = i32::try_from(pre_json_value.as_i64().unwrap())?;
                                        final_widget.value = val.to_string();
                                        Bson::Int32(val)
                                    }
                                    "selectU32" | "selectI64" => {
                                        let val = pre_json_value.as_i64().unwrap();
                                        final_widget.value = val.to_string();
                                        Bson::Int64(val)
                                    }
                                    "selectF64" => {
                                        let val = pre_json_value.as_f64().unwrap();
                                        final_widget.value = val.to_string();
                                        Bson::Double(val)
                                    }
                                    _ => Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                        Unsupported widget type - `{}`.\n\n",
                                        model_name, field_name, widget_type
                                    ))?,
                                },
                            );
                            break;
                        } else {
                            if !final_widget.value.is_empty() {
                                if widget_type == "selectText" {
                                    tmp_value = serde_json::to_value(final_widget.value.clone())?;
                                } else {
                                    tmp_value = serde_json::from_str(final_widget.value.as_str())?;
                                }
                                tmp_json_text = serde_json::to_string(&tmp_value)?;
                                pre_json_value = &tmp_value;
                            } else {
                                if final_widget.required {
                                    is_err_symptom = true;
                                    if !final_widget.is_hide {
                                        final_widget.error = Self::accumula_err(
                                            &final_widget,
                                            &"Required field.".to_owned(),
                                        )
                                        .unwrap();
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
                                break;
                            }
                        }
                    }
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.value.is_empty() && !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                }
                //
                "selectTextDyn" | "selectI32Dyn" | "selectU32Dyn" | "selectI64Dyn"
                | "selectF64Dyn" => {
                    //
                    let tmp_json_text = serde_json::to_string(&pre_json_value)?;
                    // Get selected items.
                    if !pre_json_value.is_null() && !tmp_json_text.is_empty() {
                        final_doc.insert(
                            field_name,
                            match widget_type {
                                "selectTextDyn" => {
                                    let val = pre_json_value.as_str().unwrap().to_string();
                                    final_widget.value = val.clone();
                                    if val.is_empty() && final_widget.required {
                                        is_err_symptom = true;
                                        final_widget.error = Self::accumula_err(
                                            &final_widget,
                                            &"Required field.".to_owned(),
                                        )
                                        .unwrap();
                                    }
                                    Bson::String(val)
                                }
                                "selectI32Dyn" => {
                                    let val = i32::try_from(pre_json_value.as_i64().unwrap())?;
                                    final_widget.value = val.to_string();
                                    Bson::Int32(val)
                                }
                                "selectU32Dyn" | "selectI64Dyn" => {
                                    let val = pre_json_value.as_i64().unwrap();
                                    final_widget.value = val.to_string();
                                    Bson::Int64(val)
                                }
                                "selectF64Dyn" => {
                                    let val = pre_json_value.as_f64().unwrap();
                                    final_widget.value = val.to_string();
                                    Bson::Double(val)
                                }
                                _ => Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                    Unsupported widget type - `{}`.\n\n",
                                    model_name, field_name, widget_type
                                ))?,
                            },
                        );
                    } else {
                        if final_widget.required {
                            is_err_symptom = true;
                            if !final_widget.is_hide {
                                final_widget.error = Self::accumula_err(
                                    &final_widget,
                                    &"Required field.".to_owned(),
                                )
                                .unwrap();
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
                        final_widget.value = String::new();
                    }
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.value.is_empty() && !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                }
                //
                "selectTextMult" | "selectI32Mult" | "selectU32Mult" | "selectI64Mult"
                | "selectF64Mult" => {
                    //
                    let mut tmp_json_text = serde_json::to_string(&pre_json_value)?;
                    let mut tmp_value;
                    // Get selected items.
                    for _ in 0..=1 {
                        if !pre_json_value.is_null() && tmp_json_text != "[]" {
                            final_doc.insert(
                                field_name,
                                match widget_type {
                                    "selectTextMult" => {
                                        let val = pre_json_value
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|item| Bson::String(item.as_str().unwrap().into()))
                                            .collect::<Vec<Bson>>();
                                        Bson::Array(val)
                                    }
                                    "selectI32Mult" => Bson::Array(
                                        pre_json_value
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|item| {
                                                Bson::Int32(
                                                    i32::try_from(item.as_i64().unwrap()).unwrap(),
                                                )
                                            })
                                            .collect::<Vec<Bson>>(),
                                    ),
                                    "selectU32Mult" | "selectI64Mult" => Bson::Array(
                                        pre_json_value
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|item| Bson::Int64(item.as_i64().unwrap()))
                                            .collect::<Vec<Bson>>(),
                                    ),
                                    "selectF64Mult" => Bson::Array(
                                        pre_json_value
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|item| Bson::Double(item.as_f64().unwrap()))
                                            .collect::<Vec<Bson>>(),
                                    ),
                                    _ => Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                            Unsupported widget type - `{}`.\n\n",
                                        model_name, field_name, widget_type
                                    ))?,
                                },
                            );
                            final_widget.value = tmp_json_text;
                            break;
                        } else {
                            if !final_widget.value.is_empty() {
                                tmp_value = serde_json::from_str(final_widget.value.as_str())?;
                                tmp_json_text = serde_json::to_string(&tmp_value)?;
                                pre_json_value = &tmp_value;
                            } else {
                                if final_widget.required {
                                    is_err_symptom = true;
                                    if !final_widget.is_hide {
                                        final_widget.error = Self::accumula_err(
                                            &final_widget,
                                            &"Required field.".to_owned(),
                                        )
                                        .unwrap();
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
                                break;
                            }
                        }
                    }
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.value.is_empty() && !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                }
                //
                "selectTextMultDyn" | "selectI32MultDyn" | "selectU32MultDyn"
                | "selectI64MultDyn" | "selectF64MultDyn" => {
                    //
                    let tmp_json_text = serde_json::to_string(&pre_json_value)?;
                    // Get selected items.
                    if !pre_json_value.is_null() && tmp_json_text != "[]" {
                        final_doc.insert(
                            field_name,
                            match widget_type {
                                "selectTextMultDyn" => {
                                    let val = pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| Bson::String(item.as_str().unwrap().into()))
                                        .collect::<Vec<Bson>>();
                                    Bson::Array(val)
                                }
                                "selectI32MultDyn" => Bson::Array(
                                    pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| {
                                            Bson::Int32(
                                                i32::try_from(item.as_i64().unwrap()).unwrap(),
                                            )
                                        })
                                        .collect::<Vec<Bson>>(),
                                ),
                                "selectU32MultDyn" | "selectI64MultDyn" => Bson::Array(
                                    pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| Bson::Int64(item.as_i64().unwrap()))
                                        .collect::<Vec<Bson>>(),
                                ),
                                "selectF64MultDyn" => Bson::Array(
                                    pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| Bson::Double(item.as_f64().unwrap()))
                                        .collect::<Vec<Bson>>(),
                                ),
                                _ => Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` => \
                                        Unsupported widget type - `{}`.\n\n",
                                    model_name, field_name, widget_type
                                ))?,
                            },
                        );
                        final_widget.value = tmp_json_text;
                    } else {
                        if final_widget.required {
                            is_err_symptom = true;
                            if !final_widget.is_hide {
                                final_widget.error = Self::accumula_err(
                                    &final_widget,
                                    &"Required field.".to_owned(),
                                )
                                .unwrap();
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
                        final_widget.value = String::new();
                    }
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.value.is_empty() && !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                }
                // Validation of file type fields.
                // *********************************************************************************
                "inputFile" => {
                    //
                    let mut is_delete = false;
                    // Get field value for validation.
                    let mut _field_value: FileData = if !pre_json_value.is_null() {
                        let obj_str = pre_json_value.as_str().unwrap();
                        if let Some(is_del) = serde_json::from_str::<
                            serde_json::map::Map<String, serde_json::Value>,
                        >(obj_str)
                        .unwrap()
                        .get("is_delete")
                        {
                            is_delete = is_del.as_bool().unwrap();
                        }
                        serde_json::from_str::<FileData>(obj_str)?
                    } else {
                        FileData::default()
                    };
                    // Delete file.
                    if is_delete && is_update && !ignore_fields.contains(&field_name) {
                        if !final_widget.required
                            || ((!_field_value.path.is_empty() && !_field_value.url.is_empty())
                                || !final_widget.value.is_empty())
                        {
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                final_widget.value.as_str(),
                                false,
                            )?;
                        } else {
                            is_err_symptom = true;
                            if !final_widget.is_hide {
                                final_widget.error = Self::accumula_err(
                                    &final_widget,
                                    &"Upload a new file to delete the previous one.".to_owned(),
                                )
                                .unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                        Method: `check()` => \
                                        Upload a new file to delete the previous one.\n\n",
                                    model_name, field_name
                                ))?
                            }
                        }
                    }
                    // Get the current information about file from database.
                    let curr_info_file: String = self.db_get_file_info(&coll, field_name)?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if _field_value.path.is_empty() && _field_value.url.is_empty() {
                        if curr_info_file.is_empty() {
                            if !final_widget.value.is_empty() {
                                // Get default value.
                                _field_value = serde_json::from_str(final_widget.value.trim())?;
                            } else {
                                if final_widget.required {
                                    is_err_symptom = true;
                                    if !final_widget.is_hide {
                                        final_widget.error = Self::accumula_err(
                                            &final_widget,
                                            &"Required field.".to_owned(),
                                        )
                                        .unwrap();
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                        Method: `check()` => Required field.\n\n",
                                            model_name, field_name
                                        ))?
                                    }
                                }
                                if !is_update && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, Bson::Null);
                                }
                                continue;
                            }
                        } else {
                            final_widget.value = curr_info_file;
                            continue;
                        }
                    }
                    //
                    // Flags to check.
                    let is_emty_path = _field_value.path.is_empty();
                    let is_emty_url = _field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                            `check()` => Incorrectly filled field. \
                            Example: (for default): {{\"path\":\"./media/resume.docx\",\"url\":\"/media/resume.docx\"}} ;\
                            Example: (from client side): {{\"path\":\"\",\"url\":\"\",\"is_delete\":true}}\n\n",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let f_path = std::path::Path::new(_field_value.path.as_str());
                    if !f_path.exists() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => File is missing - {}\n\n",
                            model_name, field_name, _field_value.path
                        ))?
                    }
                    if !f_path.is_file() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => The path does not lead to a file - {}\n\n",
                            model_name, field_name, _field_value.path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    _field_value.size = metadata.len() as u32;
                    // Get file name.
                    _field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Insert result.
                    if !ignore_fields.contains(&field_name) {
                        // Add file data to widget.
                        final_widget.value = serde_json::to_string(&_field_value)?;
                        //
                        if !is_err_symptom {
                            let bson_field_value =
                                mongodb::bson::ser::to_bson(&_field_value.clone())?;
                            final_doc.insert(field_name, bson_field_value);
                        }
                    } else {
                        final_widget.value = String::new();
                    }
                }
                //
                "inputImage" => {
                    //
                    let mut is_delete = false;
                    let is_create_thumbnails: bool = !final_widget.thumbnails.is_empty();
                    // Get field value for validation.
                    let mut field_value: ImageData = if !pre_json_value.is_null() {
                        let obj_str = pre_json_value.as_str().unwrap();
                        if let Some(is_del) = serde_json::from_str::<
                            serde_json::map::Map<String, serde_json::Value>,
                        >(obj_str)
                        .unwrap()
                        .get("is_delete")
                        {
                            is_delete = is_del.as_bool().unwrap();
                        }
                        serde_json::from_str::<ImageData>(obj_str)?
                    } else {
                        ImageData::default()
                    };
                    // Delete file.
                    if is_delete && is_update && !ignore_fields.contains(&field_name) {
                        if !final_widget.required
                            || ((!field_value.path.is_empty() && !field_value.url.is_empty())
                                || !final_widget.value.is_empty())
                        {
                            self.delete_file(
                                &coll,
                                model_name,
                                field_name,
                                final_widget.value.as_str(),
                                true,
                            )?;
                        } else {
                            is_err_symptom = true;
                            if !final_widget.is_hide {
                                final_widget.error = Self::accumula_err(
                                    &final_widget,
                                    &"Upload a new file to delete the previous one.".to_owned(),
                                )
                                .unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                        Method: `check()` => \
                                        Upload a new file to delete the previous one.\n\n",
                                    model_name, field_name
                                ))?
                            }
                        }
                    }
                    // Get the current information about file from database.
                    let curr_info_file: String = self.db_get_file_info(&coll, field_name)?;
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if field_value.path.is_empty() && field_value.url.is_empty() {
                        if curr_info_file.is_empty() {
                            if !final_widget.value.is_empty() {
                                // Get default value.
                                field_value = serde_json::from_str(final_widget.value.trim())?;
                                // Copy the default image to the default section.
                                if is_create_thumbnails {
                                    let new_file_name = Uuid::new_v4().to_string();
                                    let path = Path::new(field_value.path.as_str());
                                    let parent = path.parent().unwrap().to_str().unwrap();
                                    let extension =
                                        path.extension().unwrap().to_str().unwrap().to_string();
                                    fs::create_dir_all(format!("{}/default", parent))?;
                                    let new_default_path = format!(
                                        "{}/default/{}.{}",
                                        parent, new_file_name, extension
                                    );
                                    fs::copy(
                                        Path::new(field_value.path.as_str()),
                                        Path::new(new_default_path.as_str()),
                                    )?;
                                    field_value.path = new_default_path;
                                    //
                                    let url = Path::new(field_value.url.as_str());
                                    let parent = url.parent().unwrap().to_str().unwrap();
                                    field_value.url = format!(
                                        "{}/default/{}.{}",
                                        parent, new_file_name, extension
                                    );
                                }
                            } else {
                                if final_widget.required {
                                    is_err_symptom = true;
                                    if !final_widget.is_hide {
                                        final_widget.error = Self::accumula_err(
                                            &final_widget,
                                            &"Required field.".to_owned(),
                                        )
                                        .unwrap();
                                    } else {
                                        Err(format!(
                                            "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                            Method: `check()` => Required field.\n\n",
                                            model_name, field_name
                                        ))?
                                    }
                                }
                                if !is_update && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, Bson::Null);
                                }
                                continue;
                            }
                        } else {
                            final_widget.value = curr_info_file;
                            continue;
                        }
                    }
                    // Flags to check.
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                            `check()` => Incorrectly filled field. \
                            Example: (for default): {{\"path\":\"./media/no_photo.jpg\",\"url\":\"/media/no_photo.jpg\"}} ;\
                            Example: (from client side): {{\"path\":\"\",\"url\":\"\",\"is_delete\":true}}\n\n",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let f_path = std::path::Path::new(field_value.path.as_str());
                    if !f_path.exists() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => File is missing - {}\n\n",
                            model_name, field_name, field_value.path
                        ))?
                    }
                    if !f_path.is_file() {
                        Err(format!(
                            "\n\nModel: `{}` > Field: `{}` ; Method: \
                                `check()` => The path does not lead to a file - {}\n\n",
                            model_name, field_name, field_value.path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    field_value.size = metadata.len() as u32;
                    // Get file name
                    field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Get image width and height.
                    let dimensions: (u32, u32) = image::image_dimensions(f_path)?;
                    field_value.width = dimensions.0;
                    field_value.height = dimensions.1;
                    // Generate sub-size images.
                    if is_create_thumbnails {
                        let mut img = image::open(f_path)?;
                        for max_size in final_widget.thumbnails.iter() {
                            let thumbnail_size: (u32, u32) = Self::calculate_thumbnail_size(
                                dimensions.0,
                                dimensions.1,
                                max_size.1,
                            );
                            if thumbnail_size.0 > 0 && thumbnail_size.1 > 0 {
                                let width = thumbnail_size.0;
                                let height = thumbnail_size.1;
                                let thumb_name = format!("{}_{}", max_size.0, field_value.name);
                                let thumb_path = field_value
                                    .path
                                    .clone()
                                    .replace(field_value.name.as_str(), thumb_name.as_str());
                                let thumb_url = field_value
                                    .url
                                    .clone()
                                    .replace(field_value.name.as_str(), thumb_name.as_str());
                                img = img.resize_exact(
                                    width,
                                    height,
                                    image::imageops::FilterType::Triangle,
                                );
                                match max_size.0.as_str() {
                                    "lg" => {
                                        img.save(thumb_path.clone())?;
                                        field_value.path_lg = thumb_path;
                                        field_value.url_lg = thumb_url;
                                    }
                                    "md" => {
                                        img.save(thumb_path.clone())?;
                                        field_value.path_md = thumb_path;
                                        field_value.url_md = thumb_url;
                                    }
                                    "sm" => {
                                        img.save(thumb_path.clone())?;
                                        field_value.path_sm = thumb_path;
                                        field_value.url_sm = thumb_url;
                                    }
                                    "xs" => {
                                        img.save(thumb_path.clone())?;
                                        field_value.path_xs = thumb_path;
                                        field_value.url_xs = thumb_url;
                                    }
                                    _ => Err(format!(
                                        "\n\nModel: `{}` > Field: `{}` ; Method: \
                                            `check()` => Valid size names -\
                                             `xs`, `sm`, `md`, `lg`.\n\n",
                                        model_name, field_name
                                    ))?,
                                }
                            };
                        }
                    }
                    // Insert result.
                    if !ignore_fields.contains(&field_name) {
                        // Add image data to widget.
                        final_widget.value = serde_json::to_string(&field_value)?;
                        //
                        if !is_err_symptom {
                            let bson_field_value =
                                mongodb::bson::ser::to_bson(&field_value.clone())?;
                            final_doc.insert(field_name, bson_field_value);
                        }
                    } else {
                        final_widget.value = String::new();
                    }
                }
                // Validation of number type fields.
                // *********************************************************************************
                "radioI32" | "numberI32" | "rangeI32" | "hiddenI32" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();

                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if pre_json_value.is_null() {
                        if !final_widget.value.is_empty() {
                            field_value = Some(final_widget.value.clone().parse::<i64>()?);
                        } else {
                            if final_widget.required {
                                is_err_symptom = true;
                                if !widget_type.contains("hidden") && !final_widget.is_hide {
                                    final_widget.error = Self::accumula_err(
                                        &final_widget,
                                        &"Required field.".to_owned(),
                                    )
                                    .unwrap();
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
                    }
                    // Get clean data.
                    let field_value = i32::try_from(field_value.unwrap())?;
                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.value = field_value.to_string();
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = Bson::Int32(field_value);
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field: `{}` ; Method: `check()` -> {}\n\n",
                                    model_name,
                                    field_name,
                                    err.to_string()
                                ))
                                .unwrap()
                            }
                        });
                    }
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
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: i32 = final_widget.min.parse().unwrap_or_default();
                    let max: i32 = final_widget.max.parse().unwrap_or_default();
                    if (min != 0_i32 || max != 0_i32) && (field_value < min || field_value > max) {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <-> max={}).",
                            field_value, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                "radioU32" | "numberU32" | "rangeU32" | "radioI64" | "numberI64" | "rangeI64"
                | "hiddenU32" | "hiddenI64" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();

                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if pre_json_value.is_null() {
                        if !final_widget.value.is_empty() {
                            field_value = Some(final_widget.value.clone().parse::<i64>()?);
                        } else {
                            if final_widget.required {
                                is_err_symptom = true;
                                if !widget_type.contains("hidden") && !final_widget.is_hide {
                                    final_widget.error = Self::accumula_err(
                                        &final_widget,
                                        &"Required field.".to_owned(),
                                    )
                                    .unwrap();
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
                    }
                    // Get clean data.
                    let field_value: i64 = field_value.unwrap();
                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.value = field_value.to_string();
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = Bson::Int64(field_value);
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: i64 = final_widget.min.parse().unwrap_or_default();
                    let max: i64 = final_widget.max.parse().unwrap_or_default();
                    if (min != 0_i64 || max != 0_i64) && (field_value < min || field_value > max) {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <-> max={}).",
                            field_value, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                "radioF64" | "numberF64" | "rangeF64" | "hiddenF64" => {
                    // Get field value for validation.
                    let mut field_value: Option<f64> = pre_json_value.as_f64();

                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible ).
                    // -----------------------------------------------------------------------------
                    if pre_json_value.is_null() {
                        if !final_widget.value.is_empty() {
                            field_value = Some(final_widget.value.clone().parse::<f64>()?);
                        } else {
                            if final_widget.required {
                                is_err_symptom = true;
                                if !widget_type.contains("hidden") && !final_widget.is_hide {
                                    final_widget.error = Self::accumula_err(
                                        &final_widget,
                                        &"Required field.".to_owned(),
                                    )
                                    .unwrap();
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
                    }
                    // Get clean data.
                    let field_value: f64 = field_value.unwrap();

                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.value = field_value.to_string();
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = Bson::Double(field_value);
                    // Field attribute check - `pattern`.
                    // -----------------------------------------------------------------------------
                    if !final_widget.pattern.is_empty() {
                        Self::regex_pattern_validation(
                            final_widget.value.as_str(),
                            final_widget.pattern.as_str(),
                        )
                        .unwrap_or_else(|err| {
                            is_err_symptom = true;
                            if !widget_type.contains("hidden") && !final_widget.is_hide {
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
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
                    if final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap_or_default();
                    let max: f64 = final_widget.max.parse().unwrap_or_default();
                    if (min != 0_f64 || max != 0_f64) && (field_value < min || field_value > max) {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <-> max={}).",
                            field_value, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }

                // Validation of boolean type fields.
                // *********************************************************************************
                "checkBox" => {
                    // Get field value for validation.
                    let field_value: bool = if !pre_json_value.is_null() {
                        pre_json_value.as_bool().unwrap()
                    } else {
                        // Validation, if the field is required and empty, accumulate the error.
                        // ( The default value is used whenever possible )
                        if final_widget.required {
                            is_err_symptom = true;
                            if !final_widget.is_hide {
                                final_widget.error = Self::accumula_err(
                                    &final_widget,
                                    &"You must definitely choose.".to_owned(),
                                )
                                .unwrap();
                            } else {
                                Err(format!(
                                    "\n\nModel: `{}` > Field (hidden): `{}` ; \
                                        Method: `check()` => \
                                        Hiding required fields is not allowed.\n\n",
                                    model_name, field_name
                                ))?
                            }
                            false
                        } else {
                            // Apply the value default.
                            final_widget.checked
                        }
                    };
                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.checked = field_value.clone();

                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = Bson::Boolean(field_value);
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                _ => Err(format!(
                    "Model: `{}` > Field: `{}` ; Method: `check()` => \
                     Unsupported widget type - `{}`.",
                    model_name, field_name, widget_type
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
                        final_widget_map.get_mut("updated_at").unwrap().value = dt_text.clone();
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
                    final_widget_map.get_mut("created_at").unwrap().value = dt_text.clone();
                    self.set_created_at(dt_text);
                } else if is_save {
                    // For create.
                    final_doc.insert("created_at", Bson::DateTime(dt));
                    final_doc.insert("updated_at", Bson::DateTime(dt));
                    self.set_created_at(dt_text.clone());
                    self.set_updated_at(dt_text.clone());
                    final_widget_map.get_mut("created_at").unwrap().value = dt_text.clone();
                    final_widget_map.get_mut("updated_at").unwrap().value = dt_text;
                }
            }
        }

        // If the validation is negative, delete the orphaned files.
        if is_err_symptom && !is_update {
            let default_value_map = meta.default_value_map;
            for (field, widget) in final_widget_map.iter_mut() {
                match widget.widget.as_str() {
                    "inputFile" if !widget.value.is_empty() => {
                        let default_value = default_value_map.get(field).unwrap().1.as_str();
                        let default_path = if !default_value.is_empty() {
                            serde_json::from_str::<FileData>(default_value)?.path
                        } else {
                            String::new()
                        };
                        let current = serde_json::from_str::<FileData>(widget.value.as_str())?;
                        // Exclude files by default.
                        if current.path != default_path {
                            let path = Path::new(&current.path);
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                            widget.value = String::new();
                        }
                    }
                    "inputImage" if !widget.value.is_empty() => {
                        let default_value = default_value_map.get(field).unwrap().1.as_str();
                        let default_path = if !default_value.is_empty() {
                            serde_json::from_str::<ImageData>(default_value)?.path
                        } else {
                            String::new()
                        };
                        let current = serde_json::from_str::<ImageData>(widget.value.as_str())?;
                        // Exclude files by default.
                        if current.path != default_path {
                            let path = Path::new(&current.path);
                            if path.exists() {
                                fs::remove_file(path)?;
                            }
                            // Remove thumbnails.
                            let size_names: [&str; 4] = ["lg", "md", "sm", "xs"];
                            for size_name in size_names {
                                let path = match size_name {
                                    "lg" => current.path_lg.clone(),
                                    "md" => current.path_md.clone(),
                                    "sm" => current.path_sm.clone(),
                                    "xs" => current.path_xs.clone(),
                                    _ => String::new(),
                                };
                                if !path.is_empty() {
                                    let path = Path::new(path.as_str());
                                    if path.exists() {
                                        fs::remove_file(path)?;
                                    }
                                }
                            }
                            widget.value = String::new();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Enrich the widget map with values for dynamic widgets.
        Self::vitaminize(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            &client_cache,
            &final_model_json,
            &meta.fields_name,
        )?;

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputDataCheck::from(
            !is_err_symptom,
            Some(final_doc),
            final_model_json,
            meta.service_name,
            meta.model_name,
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
        // Run hooks.
        if self.get_hash().is_empty() {
            self.pre_create();
        } else {
            self.pre_update();
        }
        //
        let mut stop_step: u8 = 0;
        //
        for num in 0_u8..=1_u8 {
            // Get checked data from the `check()` method.
            let mut verified_data = self.check(Some(true))?;
            let is_no_error: bool = verified_data.is_valid();
            let final_doc = verified_data.get_doc().unwrap();
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
                let wig_name = String::from("inputSlug");
                let hash = String::from("hash");
                for val in model_cache.widget_map.values() {
                    if val.widget == wig_name && val.slug_sources.contains(&hash) {
                        stop_step = 1;
                        break;
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
                    // Update doc.
                    coll.update_one(query, update, options_update.clone())?;
                    // Run hook.
                    if stop_step == 0 {
                        self.post_update();
                    }
                } else {
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
                let mut final_widget_map = verified_data.to_wig();
                final_widget_map.get_mut("hash").unwrap().value = hash_line;
                verified_data.set_wig(final_widget_map);
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
            "Model: `{}` > Method: `save` => \
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
        // Run hook.
        self.pre_delete();
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
                for (field_name, widget_name) in meta.widget_type_map.iter() {
                    if !document.is_null(field_name) {
                        match widget_name.as_str() {
                            "inputFile" => {
                                if let Some(info_file) =
                                    document.get(field_name).unwrap().as_document()
                                {
                                    let path = info_file.get_str("path")?;
                                    let default_value =
                                        meta.default_value_map.get(field_name).unwrap().1.as_str();
                                    let default_path = if !default_value.is_empty() {
                                        serde_json::from_str::<FileData>(default_value)?.path
                                    } else {
                                        String::new()
                                    };
                                    if path != default_path {
                                        let path = Path::new(path);
                                        if path.exists() {
                                            fs::remove_file(path)?;
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
                            "inputImage" => {
                                if let Some(info_file) =
                                    document.get(field_name).unwrap().as_document()
                                {
                                    let path = info_file.get_str("path")?;
                                    let default_value =
                                        meta.default_value_map.get(field_name).unwrap().1.as_str();
                                    let default_path = if !default_value.is_empty() {
                                        serde_json::from_str::<ImageData>(default_value)?.path
                                    } else {
                                        String::new()
                                    };
                                    if path != default_path {
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
                                } else {
                                    Err(format!(
                                        "Model: `{}` > Field: `{}` > \
                                         Method: `delete()` => Document (info file) not found.",
                                        meta.model_name, field_name
                                    ))?
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                Err(format!(
                    "Model: `{}` ; Method: `delete()` => Document not found.",
                    meta.model_name
                ))?
            }
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
