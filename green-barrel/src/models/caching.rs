//! Caching inmodelation about Models for speed up work.

use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Bson, Document},
    Client,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;
use std::{collections::HashMap, convert::TryFrom, error::Error};

use crate::{
    models::{converters::Converters, helpers::ControlArr, Main},
    store::METADATA,
};

type ChoicesStrMap = HashMap<String, Vec<String>>;
type ChoicesI32Map = HashMap<String, Vec<i32>>;
type ChoicesI64Map = HashMap<String, Vec<i64>>;
type ChoicesF64Map = HashMap<String, Vec<f64>>;

/// Caching inmodelation about Models for speed up work.
// #################################################################################################
#[async_trait(?Send)]
pub trait Caching: Main + Converters {
    /// Add metadata to cache.
    // *********************************************************************************************
    async fn caching(client: &Client) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get metadata of Model.
        let mut meta = Self::generate_metadata()?;
        // Enrich the field map with values for dynamic fields.
        Self::injection(
            client,
            meta.app_name.as_str(),
            meta.unique_app_key.as_str(),
            meta.collection_name.as_str(),
            &mut meta.model_json,
            &meta.fields_name,
        )
        .await?;
        let (choices_str_map, choices_i32_map, choices_i64_map, choices_f64_map) =
            Self::get_choice_maps(&meta.model_json, &meta.field_type_map)?;
        meta.choice_str_map = choices_str_map;
        meta.choice_i32_map = choices_i32_map;
        meta.choice_i64_map = choices_i64_map;
        meta.choice_f64_map = choices_f64_map;
        // Get metadata store.
        // Get a key to access the metadata store.
        let key = Self::key()?;
        let mut metadata = METADATA.lock().await;
        // Save the meta to storage.
        metadata.insert(key, meta);
        //
        Ok(())
    }

    /// Get choice maps for fields type `choice`.
    fn get_choice_maps(
        model_json: &Value,
        field_type_map: &HashMap<String, String>,
    ) -> Result<(ChoicesStrMap, ChoicesI32Map, ChoicesI64Map, ChoicesF64Map), Box<dyn Error>> {
        //
        let mut choices_str_map = HashMap::<String, Vec<String>>::new();
        let mut choices_i32_map = HashMap::<String, Vec<i32>>::new();
        let mut choices_i64_map = HashMap::<String, Vec<i64>>::new();
        let mut choices_f64_map = HashMap::<String, Vec<f64>>::new();
        for (field_name, field_type) in field_type_map {
            if let Some(choices) = model_json.get(field_name).unwrap().get("choices") {
                if field_type.contains("Text") {
                    let choices = choices
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap().to_string())
                        .collect::<Vec<String>>();
                    choices_str_map.insert(field_name.into(), choices);
                } else if field_type.contains("I32") {
                    let choices = choices
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| {
                            i32::try_from(item.as_array().unwrap()[0].as_i64().unwrap()).unwrap()
                        })
                        .collect::<Vec<i32>>();
                    choices_i32_map.insert(field_name.into(), choices);
                } else if field_type.contains("U32") || field_type.contains("I64") {
                    let choices = choices
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    choices_i64_map.insert(field_name.into(), choices);
                } else if field_type.contains("F64") {
                    let choices = choices
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .collect::<Vec<f64>>();
                    choices_f64_map.insert(field_name.into(), choices);
                }
            }
        }
        Ok((
            choices_str_map,
            choices_i32_map,
            choices_i64_map,
            choices_f64_map,
        ))
    }

    /// Get a new model instance with custom settings.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let user = User::new().await?;
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
    async fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let metadata = METADATA.lock().await;
        // Get meta of Model.
        if let Some(meta) = metadata.get(&key) {
            let instance = serde_json::from_value(meta.model_json.clone())?;
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
    /// let json_line = User::json().await?;
    /// println!("{json_line}");
    /// ```
    ///
    async fn json() -> Result<String, Box<dyn Error>> {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let metadata = METADATA.lock().await;
        // Get metadata of Model.
        if let Some(meta) = metadata.get(&key) {
            let json_line = serde_json::to_string(&meta.model_json)?;
            return Ok(json_line);
        }
        //
        Err(format!(
            "Model key: `{key}` ; Method: `json()` => \
             Failed to get data from cache.",
        ))?
    }

    /// Update data for dynamic fields.
    /// A more convenient use of these types of fields is implemented in the Green Panel project:
    /// https://github.com/kebasyaty/green-panel
    ///
    /// # Example:
    ///
    /// let dyn_data = json!({
    ///     "field_name": "field_name",
    ///     "value": 5,
    ///     "title": "Title",
    ///     "is_delete": false
    /// });
    /// assert!(User::update_dyn_field(&client, dyn_data).await.is_ok());
    /// ```
    ///
    // *********************************************************************************************
    async fn update_dyn_field(client: &Client, dyn_data: Value) -> Result<(), Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (
            model_name,
            model_json,
            project_name,
            unique_project_key,
            database_name,
            collection_name,
        ) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let metadata = METADATA.lock().await;
            // Get metadata of Model.
            if let Some(meta) = metadata.get(&key) {
                (
                    meta.model_name.clone(),
                    meta.model_json.clone(),
                    meta.app_name.clone(),
                    meta.unique_app_key.clone(),
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `update_dyn_field()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Define conditional constants.
        let const_field_name = {
            if let Some(field_name) = dyn_data["field_name"].as_str() {
                field_name
            } else {
                Err(format!(
                    "Model: {model_name} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `field_name` => \
                        The field is missing."
                ))?
            }
        };
        let const_title = {
            if let Some(title) = dyn_data["title"].as_str() {
                title
            } else {
                Err(format!(
                    "Model: {model_name} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `title` => \
                        The field is missing."
                ))?
            }
        };
        let const_is_delete = {
            if let Some(is_delete) = dyn_data["is_delete"].as_bool() {
                is_delete
            } else {
                Err(format!(
                    "Model: {model_name} > Method: `update_dyn_field()` > \
                        Parameter: `dyn_data` > Field: `is_delete` => \
                        The field is missing."
                ))?
            }
        };
        // Define conditional constants.
        // Get field map and check the field name for belonging to the Model.
        let const_field = {
            if let Some(field) = model_json.get(const_field_name) {
                field
            } else {
                Err(format!(
                    "Model: {model_name} > Method: `update_dyn_field()` => \
                        There is no field named `{const_field_name}` in the model."
                ))?
            }
        };
        let const_field_type = const_field.get("field_type").unwrap().as_str().unwrap();
        // Check the Field type for belonging to dynamic types.
        if !const_field_type.contains("Dyn") {
            Err(format!(
                "Model: {model_name} > Field: `{const_field_name}` ; \
                Method: `update_dyn_field()` => Field `{const_field_type}` is not dynamic."
            ))?
        }
        //
        // Get access to the technical base of the project.
        let coll = {
            let green_tech_keyword = format!("green_tech__{project_name}__{unique_project_key}");
            let db = client.database(&green_tech_keyword);
            db.collection::<Document>("dynamic_fields")
        };
        //
        let filter = doc! {
            "database": database_name.clone(),
            "collection": collection_name.clone()
        };
        // Get the target array from the dynamic data collection.
        let mut obj_fields_doc = {
            let curr_dyn_date_doc = coll.find_one(filter.clone(), None).await?.unwrap();
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
                        "Model: {model_name} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `&str` type."
                    ))?
                }
            } else if const_field_type.contains("I32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                        Err(format!(
                            "Model: {model_name} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{}` is not a `i32` type.",
                            val
                        ))?
                    }
                    let val = i32::try_from(val)?;
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i32().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {model_name} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `i32` type."
                    ))?
                }
            } else if const_field_type.contains("U32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if val < (u32::MIN as i64) || val > (u32::MAX as i64) {
                        Err(format!(
                            "Model: {model_name} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `u32` type."
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .any(|x| x == val)
                } else {
                    Err(format!(
                        "Model: {model_name} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `u32` type."
                    ))?
                }
            } else if const_field_type.contains("I64") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if !(i64::MIN..=i64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {model_name} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `i64` type."
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {model_name} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `i64` type."
                    ))?
                }
            } else if const_field_type.contains("F64") {
                if let Some(val) = dyn_data["value"].as_f64() {
                    if !(f64::MIN..=f64::MAX).contains(&val) {
                        Err(format!(
                            "Model: {model_name} > Method: `update_dyn_field()` > \
                                Parameter: `dyn_data` > Field: `value` => \
                                The value `{val}` is not a `f64` type."
                        ))?
                    }
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .any(|item| item == val)
                } else {
                    Err(format!(
                        "Model: {model_name} > Method: `update_dyn_field()` > \
                            Parameter: `dyn_data` > Field: `value` => \
                            The value is not a `f64` type."
                    ))?
                }
            } else {
                false
            };
            if !const_is_delete && is_value_exist {
                Err(format!(
                    "Model: {model_name} > Field: `{const_field_name}` ; Method: `update_dyn_field()` => \
                    Cannot add new value, similar value already exists."
                ))?
            }
            if const_is_delete && !is_value_exist {
                Err(format!(
                    "Model: {model_name} > Field: `{const_field_name}` ; Method: `update_dyn_field()` => \
                        The value cannot be deleted, it is missing."
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
                            "Model: {model_name} > Method: `update_dyn_field()` => \
                            Invalid data type."
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
                    "Model: {model_name} > Method: `update_dyn_field()` => \
                    Invalid data type."
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
            coll.update_one(filter, update, None).await?;
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
                    "Model: {model_name} > Method: `update_dyn_field()` => \
                    Invalid data type."
                ))?
            };
            //
            let db = client.database(database_name.as_str());
            let coll = db.collection::<Document>(collection_name.as_str());
            let mut cursor = coll.find(None, None).await?;
            // Iterate over all documents in the collection.
            while let Some(mut doc_from_db) = cursor.try_next().await? {
                let mut is_changed = false;
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
                            "Model: {model_name} > Method: `update_dyn_field()` => \
                            Invalid data type."
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
                    coll.update_one(query, doc_from_db, None).await?;
                }
            }
        }
        // Update metadata in cache.
        Self::caching(client).await?;

        Ok(())
    }
}
