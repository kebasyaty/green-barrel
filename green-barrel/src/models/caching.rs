//! Caching inmodelation about Models for speed up work.

use mongodb::{
    bson::{doc, Bson, Document},
    sync::Client,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, convert::TryFrom, error::Error};

use crate::models::{
    converters::Converters,
    helpers::{ControlArr, Meta},
    Main,
};

type OptionsStrMap = HashMap<String, Vec<String>>;
type OptionsI32Map = HashMap<String, Vec<i32>>;
type OptionsI64Map = HashMap<String, Vec<i64>>;
type OptionsF64Map = HashMap<String, Vec<f64>>;

/// Caching inmodelation about Models for speed up work.
// #################################################################################################
pub trait Caching: Main + Converters {
    /// Add metadata to cache.
    // *********************************************************************************************
    fn caching(
        meta_store: &Arc<Mutex<HashMap<String, Meta>>>,
        client: &Client,
    ) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key: String = Self::key()?;
        // Get metadata of Model.
        let mut metadata = Self::generate_metadata()?;
        // Enrich the field map with values for dynamic fields.
        Self::injection(
            metadata.project_name.as_str(),
            metadata.unique_project_key.as_str(),
            metadata.collection_name.as_str(),
            &mut metadata.model_json,
            &metadata.fields_name,
            client,
        )?;
        let (options_str_map, options_i32_map, options_i64_map, options_f64_map) =
            Self::get_option_maps(&metadata.model_json, &metadata.field_type_map)?;
        metadata.option_str_map = options_str_map;
        metadata.option_i32_map = options_i32_map;
        metadata.option_i64_map = options_i64_map;
        metadata.option_f64_map = options_f64_map;
        // Get Metadata Store.
        let mut store = meta_store.lock().unwrap();
        // Save the metadata to storage.
        store.insert(key, metadata);
        //
        Ok(())
    }

    /// Get option maps for fields type `select`.
    fn get_option_maps(
        model_json: &Value,
        field_type_map: &HashMap<String, String>,
    ) -> Result<(OptionsStrMap, OptionsI32Map, OptionsI64Map, OptionsF64Map), Box<dyn Error>> {
        //
        let mut options_str_map = HashMap::<String, Vec<String>>::new();
        let mut options_i32_map = HashMap::<String, Vec<i32>>::new();
        let mut options_i64_map = HashMap::<String, Vec<i64>>::new();
        let mut options_f64_map = HashMap::<String, Vec<f64>>::new();
        for (field_name, field_type) in field_type_map {
            if let Some(options) = model_json.get(field_name).unwrap().get("options") {
                if field_type.contains("Text") {
                    let options = options
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap().to_string())
                        .collect::<Vec<String>>();
                    options_str_map.insert(field_name.into(), options);
                } else if field_type.contains("I32") {
                    let options = options
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| {
                            i32::try_from(item.as_array().unwrap()[0].as_i64().unwrap()).unwrap()
                        })
                        .collect::<Vec<i32>>();
                    options_i32_map.insert(field_name.into(), options);
                } else if field_type.contains("U32") || field_type.contains("I64") {
                    let options = options
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    options_i64_map.insert(field_name.into(), options);
                } else if field_type.contains("F64") {
                    let options = options
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .collect::<Vec<f64>>();
                    options_f64_map.insert(field_name.into(), options);
                }
            }
        }
        Ok((
            options_str_map,
            options_i32_map,
            options_i64_map,
            options_f64_map,
        ))
    }

    /// Get a new model instance with custom settings.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User::new(&meta_store)?;
    /// user.username.set("user");
    /// user.email.set("user_1_@noreply.net");
    /// user.password.set("12345678");
    /// user.confirm_password.set("12345678");
    /// user.is_staff.set(true);
    /// user.is_active.set(true);
    ///
    /// println!("{:#?}", user);
    /// ```
    ///
    fn new(meta_store: &Arc<Mutex<HashMap<String, Meta>>>) -> Result<Self, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key: String = Self::key()?;
        // Get Metadata Store.
        let store = meta_store.lock().unwrap();
        // Get metadata of Model.
        if let Some(metadata) = store.get(&key) {
            let instance = serde_json::from_value(metadata.model_json.clone())?;
            return Ok(instance);
        }
        //
        Err(format!(
            "Model key: `{key}` ; Method: `new()` => \
             Failed to get data from cache.",
        ))?
    }

    /// Get field attributes in Json modelat for page templates.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let json_line = User::json(&meta_store)?;
    /// println!("{json_line}");
    /// ```
    ///
    fn json(meta_store: &Arc<Mutex<HashMap<String, Meta>>>) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key: String = Self::key()?;
        // Get Metadata Store.
        let store = meta_store.lock().unwrap();
        // Get metadata of Model.
        if let Some(metadata) = store.get(&key) {
            let json_line = serde_json::to_string(&metadata.model_json)?;
            return Ok(json_line);
        }
        //
        Err(format!(
            "Model key: `{key}` ; Method: `json()` => \
             Failed to get data from cache.",
        ))?
    }

    /// Update data for dynamic fields.
    /// Hint: For more convenience, use the admin panel - https://github.com/kebasyaty/actix-greenpanel
    ///
    /// # Example:
    ///
    /// let dyn_data = json!({
    ///     "field_name": "field_name",
    ///     "value": 5, // restrict with field attributes
    ///     "title": "Title",
    ///     "is_delete": false
    /// });
    /// assert!(User::update_dyn_field(dyn_data, &meta_store, &client).is_ok());
    /// ```
    ///
    // *********************************************************************************************
    fn update_dyn_field(
        dyn_data: Value,
        meta_store: &Arc<Mutex<HashMap<String, Meta>>>,
        client: &Client,
    ) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key: String = Self::key()?;
        // Get Metadata Store.
        let store = meta_store.lock().unwrap();
        // Get metadata of Model.
        let metadata = store.get(&key);
        let metadata = if metadata.is_some() {
            metadata.unwrap()
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `json()` => \
                Failed to get data from cache.",
            ))?
        };
        // Define conditional constants.
        let const_field_name = {
            if let Some(field_name) = dyn_data["field_name"].as_str() {
                field_name
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `field_name` => \
                        The field is missing.",
                    metadata.model_name
                ))?
            }
        };
        let const_title = {
            if let Some(title) = dyn_data["title"].as_str() {
                title
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `title` => \
                        The field is missing.",
                    metadata.model_name
                ))?
            }
        };
        let const_is_delete = {
            if let Some(is_delete) = dyn_data["is_delete"].as_bool() {
                is_delete
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `is_delete` => \
                        The field is missing.",
                    metadata.model_name
                ))?
            }
        };
        // Define conditional constants.
        // Get field map and check the field name for belonging to the Model.
        let const_field = {
            if let Some(field) = metadata.model_json.get(const_field_name) {
                field
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` => \
                        There is no field named `{const_field_name}` in the model.",
                    metadata.model_name,
                ))?
            }
        };
        let const_field_type = const_field.get("field_type").unwrap().as_str().unwrap();
        // Check the Field type for belonging to dynamic types.
        if !const_field_type.contains("Dyn") {
            Err(format!(
                "Model: {} > Field: `{const_field_name}` ; Method: `update_dyn_field()` => \
                    Field `{const_field_type}` is not dynamic.",
                metadata.model_name
            ))?
        }
        //
        // Get access to the technical base of the project.
        let coll = {
            let green_tech_keyword = format!(
                "green_tech__{}__{}",
                metadata.project_name, metadata.unique_project_key
            );
            let db = client.database(&green_tech_keyword);
            db.collection::<Document>("dynamic_fields")
        };
        //
        let filter = doc! {
            "database": metadata.database_name.clone(),
            "collection": metadata.collection_name.clone()
        };
        // Get the target array from the dynamic data collection.
        let mut obj_fields_doc = {
            let curr_dyn_date_doc = coll.find_one(filter.clone(), None)?.unwrap();
            curr_dyn_date_doc.get_document("fields").unwrap().clone()
        };
        let mut target_arr_bson = obj_fields_doc.get_array(const_field_name).unwrap().clone();

        // Update dynamic data.
        // -----------------------------------------------------------------------------------------
        // 1.Check for a similar value.
        // 2.Check that the value type is compatible with the Field type.
        {
            let is_value_exist = if const_field_type.contains("Text") {
                if let Some(val) = dyn_data["value"].as_str() {
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `&str` type.",
                        metadata.model_name
                    ))?
                }
            } else if const_field_type.contains("I32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `i32` type.",
                            metadata.model_name, val
                        ))?
                    }
                    let val = i32::try_from(val)?;
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i32().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `i32` type.",
                        metadata.model_name
                    ))?
                }
            } else if const_field_type.contains("U32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (u32::MIN as i64) || val > (u32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `u32` type.",
                            metadata.model_name
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .any(|x| x == val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `u32` type.",
                        metadata.model_name
                    ))?
                }
            } else if const_field_type.contains("I64") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if !(i64::MIN..=i64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `i64` type.",
                            metadata.model_name
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `i64` type.",
                        metadata.model_name
                    ))?
                }
            } else if const_field_type.contains("F64") {
                if let Some(val) = dyn_data["value"].as_f64() {
                    if !(f64::MIN..=f64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `f64` type.",
                            metadata.model_name
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `f64` type.",
                        metadata.model_name
                    ))?
                }
            } else {
                false
            };
            if !const_is_delete && is_value_exist {
                Err(format!(
                    "Model: {} > Field: `{const_field_name}` ; Method: `update_dyn_field()` => \
                    Cannot add new value, similar value already exists.",
                    metadata.model_name
                ))?
            }
            if const_is_delete && !is_value_exist {
                Err(format!(
                    "Model: {} > Field: `{const_field_name}` ; Method: `update_dyn_field()` => \
                        The value cannot be deleted, it is missing.",
                    metadata.model_name
                ))?
            }
        }
        //
        // Remove or add dynamic value.
        // -----------------------------------------------------------------------------------------
        //
        if const_is_delete {
            // Remove dynamic value.
            //
            let const_target_arr_len = target_arr_bson.len();
            //
            for idx in 0..const_target_arr_len {
                let tmp_arr = target_arr_bson[idx].as_array().unwrap();
                if tmp_arr[1].as_str().unwrap() == const_title {
                    if const_field_type.contains("Text") {
                        if tmp_arr[0].as_str().unwrap() == dyn_data["value"].as_str().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_field_type.contains("I32") {
                        if tmp_arr[0].as_i32().unwrap()
                            == i32::try_from(dyn_data["value"].as_i64().unwrap())?
                        {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_field_type.contains("U32") || const_field_type.contains("I64") {
                        if tmp_arr[0].as_i64().unwrap() == dyn_data["value"].as_i64().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_field_type.contains("F64") {
                        if tmp_arr[0].as_f64().unwrap() == dyn_data["value"].as_f64().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` => \
                                Invalid data type.",
                            metadata.model_name,
                        ))?
                    }
                }
            }
        } else {
            // Add dynamic value.
            //
            if const_field_type.contains("Text") {
                let val_bson = Bson::String(dyn_data["value"].as_str().unwrap().to_string());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_field_type.contains("I32") {
                let val_bson = Bson::Int32(i32::try_from(dyn_data["value"].as_i64().unwrap())?);
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_field_type.contains("U32") || const_field_type.contains("I64") {
                let val_bson = Bson::Int64(dyn_data["value"].as_i64().unwrap());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_field_type.contains("F64") {
                let val_bson = Bson::Double(dyn_data["value"].as_f64().unwrap());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` => \
                        Invalid data type.",
                    metadata.model_name,
                ))?
            }
        }
        //
        // Update dynamic data.
        {
            obj_fields_doc.insert(const_field_name, target_arr_bson.clone());
            let update = doc! {
                "$set": { "fields": obj_fields_doc}
            };
            coll.update_one(filter, update, None)?;
        }

        // Clean up orphaned (if any) data.
        // *****************************************************************************************
        if const_is_delete {
            //
            let const_control_arr = if const_field_type.contains("Text") {
                ControlArr::Text(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap())
                        .collect::<Vec<&str>>(),
                )
            } else if const_field_type.contains("I32") {
                ControlArr::I32(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i32().unwrap())
                        .collect::<Vec<i32>>(),
                )
            } else if const_field_type.contains("U32") || const_field_type.contains("I64") {
                ControlArr::I64(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>(),
                )
            } else if const_field_type.contains("F64") {
                ControlArr::F64(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .collect::<Vec<f64>>(),
                )
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` => \
                        Invalid data type.",
                    metadata.model_name,
                ))?
            };
            //
            let db = client.database(metadata.database_name.as_str());
            let coll = db.collection::<Document>(metadata.collection_name.as_str());
            let cursor = coll.find(None, None)?;
            // Iterate over all documents in the collection.
            for doc_from_db in cursor {
                let mut is_changed = false;
                let mut doc_from_db = doc_from_db?;
                //
                // Skip documents if field value Null.
                if doc_from_db.is_null(const_field_name) {
                    continue;
                }
                // fields with support multiple selection.
                if const_field_type.contains("Mult") {
                    let mut truncated_arr_bson = Vec::<Bson>::new();
                    if const_field_type.contains("Text") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .cloned()
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_str()
                                    .contains(&item.as_str().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_field_type.contains("I32") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .cloned()
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_i32()
                                    .contains(&item.as_i32().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_field_type.contains("U32") || const_field_type.contains("I64") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .cloned()
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_i64()
                                    .contains(&item.as_i64().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_field_type.contains("F64") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .cloned()
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_f64()
                                    .contains(&item.as_f64().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    }
                    //
                    if is_changed {
                        let result_bson = if !truncated_arr_bson.is_empty() {
                            Bson::Array(truncated_arr_bson)
                        } else {
                            Bson::Null
                        };
                        doc_from_db.insert(const_field_name, result_bson);
                    }
                } else {
                    // Select fields with support for one selection.
                    is_changed = if const_field_type.contains("Text") {
                        let val = doc_from_db.get_str(const_field_name)?;
                        !const_control_arr.control_arr_str().contains(&val)
                    } else if const_field_type.contains("I32") {
                        let val = doc_from_db.get_i32(const_field_name)?;
                        !const_control_arr.control_arr_i32().contains(&val)
                    } else if const_field_type.contains("U32") || const_field_type.contains("I64") {
                        let val = doc_from_db.get_i64(const_field_name)?;
                        !const_control_arr.control_arr_i64().contains(&val)
                    } else if const_field_type.contains("F64") {
                        let val = doc_from_db.get_f64(const_field_name)?;
                        !const_control_arr.control_arr_f64().contains(&val)
                    } else {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` => \
                                Invalid data type.",
                            metadata.model_name,
                        ))?
                    };
                    //
                    if is_changed {
                        doc_from_db.insert(const_field_name, Bson::Null);
                    }
                }
                //
                if is_changed {
                    // Update the document in the database.
                    let query = doc! {"_id": doc_from_db.get_object_id("_id")?};
                    coll.update_one(query, doc_from_db, None)?;
                }
            }
        }
        // Update metadata and fields map to cache.
        Self::caching(meta_store, client)?;
        //
        Ok(())
    }
}
