//! # Query methods for a Model instance.
//!
//! Trait:
//! `QPaladin` - Database query methods directly related to the Model instance.
//! Methods:
//! `check` - Checking the Model before queries the database.
//! `save` - Save to database as a new document or update an existing document.
//! `delete` - Remove document from collection.
//! `create_password_hash` - Generate password hash and add to result document.
//! `verify_password` - Match the password from the user to the password in the database.
//! `update_password` - For replace or recover password.
//!

use crate::{
    forms::{output_data::OutputDataForm, FileData, ImageData, Widget},
    models::{caching::CachingModel, Meta, ToModel},
};
use rand::Rng;

pub trait QPaladins: ToModel + CachingModel {
    // Checking the Model before queries the database.
    // ---------------------------------------------------------------------------------------------
    fn check(&self) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        // Get model name.
        let model_name: &str = meta.model_name.as_str();
        // User input error detection symptom.
        let mut is_err_symptom = if !meta.is_add_docs || !meta.is_up_docs {
            true
        } else {
            false
        };
        // Determines the mode of accessing the database (insert or update).
        let hash = self.get_hash().unwrap_or_default();
        let hash = hash.as_str();
        let is_update: bool = !hash.is_empty();
        // Get a list of fields that should not be included in the document.
        let ignore_fields: Vec<&str> = meta
            .ignore_fields
            .iter()
            .map(|item| item.as_str())
            .collect();
        // Access the collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(&meta.database_name)
            .collection(&meta.collection_name);
        // Get preliminary data from the model.
        let pre_json: serde_json::value::Value = self.self_to_json()?;
        // Document for the final result.
        let mut final_doc = mongodb::bson::document::Document::new();

        // Validation of field by attributes (maxlength, unique, min, max, etc...).
        // -----------------------------------------------------------------------------------------
        let fields_name: Vec<&str> = meta.fields_name.iter().map(|item| item.as_str()).collect();
        let mut final_map_widgets: std::collections::HashMap<String, Widget> =
            form_cache.map_widgets.clone();
        // Apply additional validation.
        {
            let error_map = self.add_validation()?;
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
        // Loop over fields for validation.
        for field_name in fields_name {
            // Don't check the `hash` field.
            if field_name == "hash" {
                if is_err_symptom {
                    let final_widget: &mut Widget = final_map_widgets.get_mut(field_name).unwrap();
                    if !meta.is_add_docs {
                        final_widget.common_msg = "It is forbidden to perform saves.".to_string();
                    } else if !meta.is_up_docs {
                        final_widget.common_msg = "It is forbidden to perform updates.".to_string();
                    }
                }
                continue;
            }
            // Get field value for validation.
            let pre_json_value: Option<&serde_json::value::Value> = pre_json.get(field_name);
            // Check field value.
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
            // Field validation.
            match widget_type {
                // Validation of text type fields.
                // *********************************************************************************
                "radioText" | "inputColor" | "inputEmail" | "inputPassword" | "inputPhone"
                | "inputText" | "inputUrl" | "inputIP" | "inputIPv4" | "inputIPv6" | "textArea"
                | "hiddenText" => {
                    // Get field value for validation.
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        if widget_type != "inputPassword" {
                            final_widget.value = clean_data.clone();
                        } else {
                            final_widget.value = String::new();
                        }
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
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
                            // Trying to apply the value default.
                            if !is_update && widget_type != "inputPassword" {
                                if !final_widget.value.is_empty() {
                                    field_value = final_widget.value.trim().to_string();
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = if widget_type != "inputPassword" {
                        mongodb::bson::Bson::String(field_value.clone())
                    } else {
                        mongodb::bson::Bson::Null
                    };
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression.
                    // Checking `minlength`, `maxlength`, `min length`, `max length`.
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
                    // Validation of range (`min` <> `max`).
                    // Hint: The `validate_length()` method did not
                    // provide the desired result.
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
                    // Validation of `unique`.
                    // -----------------------------------------------------------------------------
                    if widget_type != "inputPassword" && final_widget.unique {
                        Self::check_unique(hash, field_name, &bson_field_value, &coll)
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                    }
                    // Validation in regular expression (email, password, etc...).
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        match widget_type {
                            "inputPassword" => {
                                if !is_update && !field_value.is_empty() {
                                    // Generate password hash and add to result document.
                                    let hash: String = Self::create_password_hash(field_value)?;
                                    final_doc.insert(field_name, mongodb::bson::Bson::String(hash));
                                }
                            }
                            _ => {
                                // Insert result from other fields.
                                final_doc.insert(field_name, bson_field_value);
                            }
                        }
                    }
                }
                // Validation of date type fields.
                // *********************************************************************************
                "inputDate" | "inputDateTime" => {
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
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default.
                                if !final_widget.value.is_empty() {
                                    field_value = final_widget.value.trim().to_string();
                                    final_widget.value = String::new();
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    if final_widget.min != "0".to_string() && final_widget.max != "0".to_string() {
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
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, dt_value_bson);
                    }
                }
                // Validation of `select` type fields.
                // *********************************************************************************
                "selectText" | "selectI32" | "selectU32" | "selectI64" | "selectF64"
                | "selectTextDyn" | "selectI32Dyn" | "selectU32Dyn" | "selectI64Dyn"
                | "selectF64Dyn" => {
                    // Get selected items.
                    if !pre_json_value.is_null() {
                        final_doc.insert(
                            field_name,
                            match widget_type {
                                "selectText" | "selectTextDyn" => {
                                    let val = pre_json_value.as_str().unwrap().to_string();
                                    final_widget.value = val.clone();
                                    mongodb::bson::Bson::String(val)
                                }
                                "selectI32" | "selectI32Dyn" => {
                                    let val = pre_json_value.as_i64().unwrap() as i32;
                                    final_widget.value = val.to_string();
                                    mongodb::bson::Bson::Int32(val)
                                }
                                "selectU32" | "selectI64" | "selectU32Dyn" | "selectI64Dyn" => {
                                    let val = pre_json_value.as_i64().unwrap();
                                    final_widget.value = val.to_string();
                                    mongodb::bson::Bson::Int64(val)
                                }
                                "selectF64" | "selectF64Dyn" => {
                                    let val = pre_json_value.as_f64().unwrap();
                                    final_widget.value = val.to_string();
                                    mongodb::bson::Bson::Double(val)
                                }
                                _ => Err(format!(
                                    "Model: `{}` > Field: `{}` > Method: `check()` : \
                                        Unsupported widget type - `{}`.",
                                    model_name, field_name, widget_type
                                ))?,
                            },
                        );
                    } else {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                        } else if !ignore_fields.contains(&field_name) {
                            if !is_update {
                                // Trying to apply the value default.
                                if !final_widget.widget.contains("Dyn")
                                    && !final_widget.value.is_empty()
                                {
                                    final_doc.insert(
                                        field_name,
                                        match widget_type {
                                            "selectText" => {
                                                let val = final_widget.value.trim().to_string();
                                                mongodb::bson::Bson::String(val)
                                            }
                                            "selectI32" => {
                                                let val = final_widget
                                                    .value
                                                    .trim()
                                                    .parse::<i32>()
                                                    .unwrap();
                                                mongodb::bson::Bson::Int32(val)
                                            }
                                            "selectU32" | "selectI64" => {
                                                let val = final_widget
                                                    .value
                                                    .trim()
                                                    .parse::<i64>()
                                                    .unwrap();
                                                mongodb::bson::Bson::Int64(val)
                                            }
                                            "selectF64" => {
                                                let val = final_widget
                                                    .value
                                                    .trim()
                                                    .parse::<f64>()
                                                    .unwrap();
                                                mongodb::bson::Bson::Double(val)
                                            }
                                            _ => Err(format!(
                                                "Model: `{}` > Field: `{}` > Method: `check()` : \
                                                Unsupported widget type - `{}`.",
                                                model_name, field_name, widget_type
                                            ))?,
                                        },
                                    );
                                } else {
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
                                }
                            }
                        }
                        final_widget.value = String::new();
                    }
                }
                "selectTextMult" | "selectI32Mult" | "selectU32Mult" | "selectI64Mult"
                | "selectF64Mult" | "selectTextMultDyn" | "selectI32MultDyn"
                | "selectU32MultDyn" | "selectI64MultDyn" | "selectF64MultDyn" => {
                    // Get selected items.
                    if !pre_json_value.is_null() {
                        final_doc.insert(
                            field_name,
                            match widget_type {
                                "selectTextMult" | "selectTextMultDyn" => {
                                    let val = pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| {
                                            mongodb::bson::Bson::String(
                                                item.as_str().unwrap().into(),
                                            )
                                        })
                                        .collect::<Vec<mongodb::bson::Bson>>();
                                    mongodb::bson::Bson::Array(val)
                                }
                                "selectI32Mult" | "selectI32MultDyn" => {
                                    mongodb::bson::Bson::Array(
                                        pre_json_value
                                            .as_array()
                                            .unwrap()
                                            .iter()
                                            .map(|item| {
                                                mongodb::bson::Bson::Int32(
                                                    item.as_i64().unwrap() as i32
                                                )
                                            })
                                            .collect::<Vec<mongodb::bson::Bson>>(),
                                    )
                                }
                                "selectU32Mult" | "selectI64Mult" | "selectU32MultDyn"
                                | "selectI64MultDyn" => mongodb::bson::Bson::Array(
                                    pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| {
                                            mongodb::bson::Bson::Int64(item.as_i64().unwrap())
                                        })
                                        .collect::<Vec<mongodb::bson::Bson>>(),
                                ),
                                "selectF64Mult" | "selectF64MultDyn" => mongodb::bson::Bson::Array(
                                    pre_json_value
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .map(|item| {
                                            mongodb::bson::Bson::Double(item.as_f64().unwrap())
                                        })
                                        .collect::<Vec<mongodb::bson::Bson>>(),
                                ),
                                _ => Err(format!(
                                    "Model: `{}` > Field: `{}` > Method: `check()` : \
                                        Unsupported widget type - `{}`.",
                                    model_name, field_name, widget_type
                                ))?,
                            },
                        );
                    } else {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                        } else if !is_update {
                            final_doc.insert(field_name, mongodb::bson::Bson::Null);
                        }
                    }
                    final_widget.value = String::new();
                }
                // Validation of file type fields.
                // *********************************************************************************
                "inputFile" => {
                    // Get field value for validation.
                    let mut field_value: FileData = if !pre_json_value.is_null() {
                        let clean_data: FileData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        FileData::default()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if field_value.path.is_empty() && field_value.url.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default.
                                if !final_widget.value.is_empty() {
                                    field_value = serde_json::from_str(final_widget.value.trim())?;
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    final_widget.value = String::new();
                    // Flags to check.
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Incorrectly filled field. \
                            Example: {{\"path\":\"./media/hello_world.odt\",\"url\":\"/media/hello_world.odt\"}}",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let path: String = field_value.path.clone();
                    let f_path = std::path::Path::new(path.as_str());
                    if !f_path.exists() || !f_path.is_file() {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                                `check()` : File is missing - {}",
                            model_name, field_name, path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    field_value.size = metadata.len() as u32;
                    // Get file name.
                    field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Insert result.
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = mongodb::bson::ser::to_bson(&field_value.clone())?;
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                "inputImage" => {
                    // Get field value for validation.
                    let mut field_value: ImageData = if !pre_json_value.is_null() {
                        let clean_data: ImageData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        ImageData::default()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if field_value.path.is_empty() && field_value.url.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            if !is_update {
                                // Trying to apply the value default.
                                if !final_widget.value.is_empty() {
                                    field_value = serde_json::from_str(final_widget.value.trim())?;
                                } else if !is_err_symptom && !ignore_fields.contains(&field_name) {
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    final_widget.value = String::new();
                    // Flags to check.
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Incorrectly filled field. \
                            Example: {{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}}",
                            model_name, field_name
                        ))?
                    }
                    // Create path for validation of file.
                    let path: String = field_value.path.clone();
                    let f_path = std::path::Path::new(path.as_str());
                    if !f_path.exists() || !f_path.is_file() {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                                `check()` : File is missing - {}",
                            model_name, field_name, path
                        ))?
                    }
                    // Get file metadata.
                    let metadata: std::fs::Metadata = f_path.metadata()?;
                    // Get file size in bytes.
                    field_value.size = metadata.len() as u32;
                    // Get file name
                    field_value.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                    // Get image width and height.
                    let dimensions: (u32, u32) = image::image_dimensions(path)?;
                    field_value.width = dimensions.0;
                    field_value.height = dimensions.1;
                    // Insert result.
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = mongodb::bson::ser::to_bson(&field_value.clone())?;
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                // Validation of number type fields.
                // *********************************************************************************
                "radioI32" | "numberI32" | "rangeI32" | "hiddenI32" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error.
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
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    // Get clean data.
                    let field_value: i32 = field_value.unwrap() as i32;
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result.
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
                    // Validation of range (`min` <> `max`).
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
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                "radioU32" | "numberU32" | "rangeU32" | "checkBoxI64" | "radioI64"
                | "numberI64" | "rangeI64" | "hiddenU32" | "hiddenI64" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error.
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
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    // Get clean data.
                    let field_value: i64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = mongodb::bson::Bson::Int64(field_value);
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
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                "radioF64" | "numberF64" | "rangeF64" | "hiddenF64" => {
                    // Get field value for validation.
                    let mut field_value: Option<f64> = pre_json_value.as_f64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible ).
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
                                    final_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                    // Get clean data.
                    let field_value: f64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Used to validation uniqueness and in the final result.
                    let bson_field_value = mongodb::bson::Bson::Double(field_value);
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
                    // -----------------------------------------------------------------------------
                    let field_value: bool = if pre_json_value.is_null() {
                        let mut result = false;
                        // Validation, if the field is required and empty, accumulate the error.
                        // ( The default value is used whenever possible )
                        // -------------------------------------------------------------------------
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error = Self::accumula_err(
                                &final_widget,
                                &"You must definitely choose.".to_owned(),
                            )
                            .unwrap();
                        } else {
                            // Trying to apply the value default.
                            if !final_widget.value.is_empty() {
                                result = final_widget.value.trim().parse::<bool>().unwrap();
                            }
                        }
                        result
                    } else {
                        true
                    };
                    final_widget.value = String::new();
                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.checked = field_value.clone();
                    // Insert result.
                    // -----------------------------------------------------------------------------
                    if !is_err_symptom && !ignore_fields.contains(&field_name) {
                        let bson_field_value = mongodb::bson::Bson::Boolean(field_value);
                        final_doc.insert(field_name, bson_field_value);
                    }
                }
                _ => Err(format!(
                    "Model: `{}` > Field: `{}` > Method: `check()` : \
                     Unsupported widget type - `{}`.",
                    model_name, field_name, widget_type
                ))?,
            }

            // Insert or update fields for timestamps `created_at` and `updated_at`.
            // -------------------------------------------------------------------------------------
            if !is_err_symptom {
                let dt: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
                if !is_update {
                    final_doc.insert("created_at", mongodb::bson::Bson::DateTime(dt));
                    final_doc.insert("updated_at", mongodb::bson::Bson::DateTime(dt));
                } else {
                    final_doc.insert("updated_at", mongodb::bson::Bson::DateTime(dt));
                }
            }

            // Insert a field for linking a document to a user account.
            // -------------------------------------------------------------------------------------
            if !is_err_symptom && !is_update {
                final_doc.insert("chain", mongodb::bson::Bson::Null);
            }
        }

        // Enrich the widget map with values for dynamic widgets.
        Self::vitaminize(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            &client_cache,
            &mut final_map_widgets,
        )?;

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputDataForm::CheckModel((
            !is_err_symptom,
            meta.fields_name.clone(),
            final_map_widgets,
            final_doc,
        )))
    }

    // Save to database as a new document or update an existing document.
    // *********************************************************************************************
    fn save(
        &mut self,
        chain: Option<mongodb::bson::Bson>,
        options_insert: Option<mongodb::options::InsertOneOptions>,
        options_update: Option<mongodb::options::UpdateOptions>,
    ) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get checked data from the `check()` method.
        let verified_data: OutputDataForm = self.check()?;
        let is_no_error: bool = verified_data.bool();
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        // Get widget map.
        let mut final_map_widgets: std::collections::HashMap<String, Widget> = verified_data.wig();
        let is_update: bool = !self.get_hash().unwrap_or_default().is_empty();
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());

        // Save to database.
        // -----------------------------------------------------------------------------------------
        if is_no_error {
            let mut final_doc = verified_data.doc();
            if chain.is_some() {
                final_doc.insert("chain", chain.unwrap());
            }
            if !is_update {
                let result: mongodb::results::InsertOneResult =
                    coll.insert_one(final_doc, options_insert)?;
                self.set_hash(result.inserted_id.as_object_id().unwrap().to_hex());
            } else if !final_doc.is_empty() {
                let hash: Option<String> = self.get_hash();
                if hash.is_none() {
                    Err(format!(
                        "Model: `{}` > Field: `hash` : \
                        An empty `hash` field is not allowed when updating.",
                        meta.model_name
                    ))?
                }
                let object_id: mongodb::bson::oid::ObjectId =
                    mongodb::bson::oid::ObjectId::with_string(hash.unwrap().as_str())?;
                let query: mongodb::bson::document::Document =
                    mongodb::bson::doc! {"_id": object_id};
                let update: mongodb::bson::document::Document = mongodb::bson::doc! {
                    "$set": final_doc,
                };
                coll.update_one(query, update, options_update)?;
            }
        }

        // Add hash-line (for document identification).
        // -----------------------------------------------------------------------------------------
        let hash = self.get_hash().unwrap_or_default();
        if !hash.is_empty() {
            final_map_widgets.get_mut(&"hash".to_owned()).unwrap().value = hash.clone();
        }

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputDataForm::Save((
            is_no_error,
            hash,
            meta.fields_name.clone(),
            final_map_widgets,
        )))
    }

    // Remove document from collection.
    // *********************************************************************************************
    fn delete(
        &self,
        options: Option<mongodb::options::DeleteOptions>,
    ) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: mongodb::sync::Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Get Model hash  for ObjectId.
            let hash: Option<String> = self.get_hash();
            if hash.is_none() {
                Err(format!(
                    "Model: `{}` > Field: `hash` : \
                        An empty `hash` field is not allowed when deleting.",
                    meta.model_name
                ))?
            }
            let object_id: mongodb::bson::oid::ObjectId =
                mongodb::bson::oid::ObjectId::with_string(hash.unwrap().as_str())?;
            // Create query.
            let query: mongodb::bson::document::Document = mongodb::bson::doc! {"_id": object_id};
            // Execute query.
            coll.delete_one(query, options).is_ok()
        } else {
            false
        };
        Ok(OutputDataForm::Delete((result_bool, err_msg)))
    }

    // Operations with passwords.
    // *********************************************************************************************
    // Generate password hash and add to result document.
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

    // Match the password from the user to the password in the database.
    // ---------------------------------------------------------------------------------------------
    fn verify_password(
        &self,
        password: &str,
        options: Option<mongodb::options::FindOneOptions>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        // Access the collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Get hash-line of Model.
        let hash: Option<String> = self.get_hash();
        if hash.is_none() {
            Err(format!(
                "Model: `{}` > Method: `verify_password` : \
                An empty `hash` field is not allowed when updating.",
                meta.model_name
            ))?
        }
        // Convert hash-line to ObjectId.
        let object_id: mongodb::bson::oid::ObjectId =
            mongodb::bson::oid::ObjectId::with_string(hash.unwrap().as_str())?;
        // Create a filter to search for a document.
        let filter: mongodb::bson::document::Document = mongodb::bson::doc! {"_id": object_id};
        // An attempt to find the required document.
        let doc = coll.find_one(filter, options)?;
        // We check that for the given `hash` a document is found in the database.
        if doc.is_none() {
            Err(format!(
                "Model: `{}` > Method: `verify_password` : \
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
                "Model: `{}` > Method: `verify_password` : \
                The password field is missing.",
                meta.model_name
            ))?
        }
        // Get password hash or empty string.
        let password_hash = password_hash.unwrap();
        //
        let password_hash = if password_hash != &mongodb::bson::Bson::Null {
            password_hash.as_str().unwrap()
        } else {
            ""
        };
        // Password verification.
        Ok(argon2::verify_encoded(password_hash, password.as_bytes())?)
    }

    // For replace or recover password.
    // ---------------------------------------------------------------------------------------------
    fn update_password(
        &self,
        old_password: &str,
        new_password: &str,
        options_find_old: Option<mongodb::options::FindOneOptions>,
        options_update: Option<mongodb::options::UpdateOptions>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        // Validation current password.
        if !self.verify_password(old_password, options_find_old)? {
            return Ok(false);
        }
        //
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        // Access the collection.
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Get hash-line of Model.
        let hash = self.get_hash().unwrap();
        // Convert hash-line to ObjectId.
        let object_id: mongodb::bson::oid::ObjectId =
            mongodb::bson::oid::ObjectId::with_string(hash.as_str())?;
        // Create a filter to search for a document.
        let query: mongodb::bson::document::Document = mongodb::bson::doc! {"_id": object_id};
        let new_password_hash = Self::create_password_hash(new_password)?;
        let doc = mongodb::bson::doc! {"password": new_password_hash};
        let update: mongodb::bson::document::Document = mongodb::bson::doc! {
            "$set": doc,
        };
        // Update password.
        Ok(coll
            .update_one(query, update, options_update)?
            .modified_count
            == 1_i64)
    }
}
