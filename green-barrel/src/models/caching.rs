//! Caching inmodelation about Models for speed up work.

use mongodb::{
    bson::{doc, Bson},
    sync::Client,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::{convert::TryFrom, error::Error};

use crate::{
    models::{
        converters::Converters,
        helpers::{ControlArr, Meta},
        Main,
    },
    store::{ModelCache, MODEL_STORE, MONGODB_CLIENT_STORE},
};

/// Caching inmodelation about Models for speed up work.
// #################################################################################################
pub trait Caching: Main + Converters {
    /// Add metadata to cache.
    // *********************************************************************************************
    fn caching() -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get write access in cache.
        let mut model_store = MODEL_STORE.write()?;
        // Create `ModelCache` default and add map of fields and metadata of model.
        let (meta, mut model_json) = Self::generate_metadata()?;
        // Get MongoDB client for current model.
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let client = client_store.get(&meta.db_client_name).unwrap();
        // Enrich the field map with values for dynamic fields.
        Self::injection(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            client,
            &mut model_json,
            &meta.fields_name,
        )?;
        // Init new ModelCache.
        let new_model_cache = ModelCache { meta, model_json };
        // Save structure `ModelCache` to store.
        model_store.insert(key, new_model_cache);
        //
        Ok(())
    }

    /// Get metadata of Model.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let metadata = ModelName::meta()?;
    ///
    /// println!("{:?}", metadata);
    /// ```
    ///
    fn meta() -> Result<Meta, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::caching()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let metadata = Self::generate_metadata()?;
            Err(format!(
                "Model: `{}` ; Method: `meta()` => \
                    Failed to get data from cache.",
                metadata.0.model_name
            ))?
        }
        //
        let meta = model_cache.unwrap().meta.clone();
        Ok(meta)
    }

    /// Get a new model instance with custom settings.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User::new()?;
    /// user.username.set("user");
    /// user.email.set("user_1_@noreply.net");
    /// user.password.set("12345678");
    /// user.confirm_password.set("12345678");
    /// user.is_staff.set(true);
    /// user.is_active.set(true);
    ///
    /// println!("{:?}", user);
    /// ```
    ///
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::caching()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` ; Method: `new()` => \
                    Failed to get data from cache.",
                meta.model_name
            ))?
        }
        //
        let instance = serde_json::from_value(model_cache.unwrap().model_json.clone())?;
        Ok(instance)
    }

    /// Get field attributes in Json modelat for page templates.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let json_line = UserProfile::json()?;
    /// println!("{}", json_line);
    /// ```
    ///
    fn json() -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects to cache.
            Self::caching()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` ; Method: `json()` => \
                    Failed to get data from cache.",
                meta.model_name
            ))?
        }
        //
        let json_line = serde_json::to_string(&model_cache.unwrap().model_json)?;
        Ok(json_line)
    }

    /// Get cached Model data.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let (model_cache, client_cache) = UserProfile::get_cache_data_for_query()?;
    /// println!("{:?}", model_cache);
    /// ```
    ///
    fn get_cache_data_for_query() -> Result<(ModelCache, Client), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::caching()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` > Method: `get_cache_data_for_query()` => \
                    Failed to get data from cache.",
                meta.model_name
            ))?
        }
        //
        let model_cache = model_cache.unwrap();
        // Get model metadata from cache.
        let meta = &model_cache.meta;
        // Get MongoDB client for current model.
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let client = client_store.get(&meta.db_client_name).unwrap();
        //
        Ok((model_cache.clone(), client.clone()))
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
    /// assert!(ModelName::update_dyn_field(dyn_data).is_ok());
    /// ```
    ///
    // *********************************************************************************************
    fn update_dyn_field(dyn_data: Value) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        //
        // Define conditional constants.
        let const_field_name = {
            if let Some(field_name) = dyn_data["field_name"].as_str() {
                field_name
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `field_name` => \
                        The field is missing.",
                    Self::meta()?.model_name
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
                    Self::meta()?.model_name
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
                    Self::meta()?.model_name
                ))?
            }
        };
        //
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        //
        // Define conditional constants.
        // Get field map and check the field name for belonging to the Model.
        let const_field = {
            if let Some(field) = model_cache.model_json.get(const_field_name) {
                field
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_field()` => \
                        There is no field named `{}` in the model.",
                    meta.model_name, const_field_name
                ))?
            }
        };
        let const_field_type = const_field.get("field_type").unwrap().as_str().unwrap();
        // Check the Field type for belonging to dynamic types.
        if !const_field_type.contains("Dyn") {
            Err(format!(
                "Model: {} > Field: `{}` ; Method: `update_dyn_field()` => \
                    Field `{}` is not dynamic.",
                meta.model_name, const_field_name, const_field_type
            ))?
        }
        //
        // Get access to the technical base of the project.
        let coll = {
            let green_tech_keyword = format!(
                "green_tech__{}__{}",
                meta.project_name, meta.unique_project_key
            );
            let db = client_cache.database(&green_tech_keyword);
            db.collection("dynamic_fields")
        };
        //
        let filter = doc! {
            "database": meta.database_name.clone(),
            "collection": meta.collection_name.clone()
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
                        meta.model_name
                    ))?
                }
            } else if const_field_type.contains("I32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `i32` type.",
                            meta.model_name, val
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
                        meta.model_name
                    ))?
                }
            } else if const_field_type.contains("U32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (u32::MIN as i64) || val > (u32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `u32` type.",
                            meta.model_name, val
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
                        meta.model_name
                    ))?
                }
            } else if const_field_type.contains("I64") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if !(i64::MIN..=i64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `i64` type.",
                            meta.model_name, val
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
                        meta.model_name
                    ))?
                }
            } else if const_field_type.contains("F64") {
                if let Some(val) = dyn_data["value"].as_f64() {
                    if !(f64::MIN..=f64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `f64` type.",
                            meta.model_name, val
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
                        meta.model_name
                    ))?
                }
            } else {
                false
            };
            if !const_is_delete && is_value_exist {
                Err(format!(
                    "Model: {} > Field: `{}` ; Method: `update_dyn_field()` => \
                    Cannot add new value, similar value already exists.",
                    meta.model_name, const_field_name
                ))?
            }
            if const_is_delete && !is_value_exist {
                Err(format!(
                    "Model: {} > Field: `{}` ; Method: `update_dyn_field()` => \
                        The value cannot be deleted, it is missing.",
                    meta.model_name, const_field_name
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
                            meta.model_name,
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
                    meta.model_name,
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
                    meta.model_name,
                ))?
            };
            //
            let db = client_cache.database(meta.database_name.as_str());
            let coll = db.collection(meta.collection_name.as_str());
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
                            meta.model_name,
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
        Self::caching()?;
        //
        Ok(())
    }
}
