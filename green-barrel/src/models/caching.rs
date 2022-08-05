//! Caching inmodelation about Models for speed up work.

use mongodb::{
    bson::{doc, Bson},
    sync::Client,
};
use serde_json::Value;
use std::{collections::HashMap, convert::TryFrom, error::Error};

use crate::{
    helpers::{ControlArr, Enctype, HttpMethod},
    models::{converters::Converters, Main, Meta},
    store::{ModelCache, MODEL_STORE, MONGODB_CLIENT_STORE},
    widgets::{generate_html::GenerateHtml, Widget},
};

/// Caching inmodelation about Models for speed up work.
// #################################################################################################
pub trait Caching: Main + GenerateHtml + Converters {
    /// Add metadata and widgects map to cache.
    // *********************************************************************************************
    fn to_cache() -> Result<(), Box<dyn Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get write access in cache.
        let mut model_store = MODEL_STORE.write()?;
        // Create `ModelCache` default and add map of widgets and metadata of model.
        let meta: Meta = Self::meta()?;
        // Get MongoDB client for current model.
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let client_cache: &Client = client_store.get(&meta.db_client_name).unwrap();
        // Get a widget map.
        let mut widget_map: HashMap<String, Widget> = Self::widgets()?;
        // Enrich the widget map with values for dynamic widgets.
        Self::vitaminize(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            &client_cache,
            &mut widget_map,
        )?;
        // Init new ModelCache.
        let new_model_cache = ModelCache {
            meta,
            widget_map,
            ..Default::default()
        };
        // Save structure `ModelCache` to store.
        model_store.insert(key, new_model_cache);
        //
        Ok(())
    }

    /// Get an widgets map for page template.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let widgets_map = UserProfile::wig()?;
    /// println!("{:?}", widgets_map);
    /// ```
    ///
    fn to_wig() -> Result<HashMap<String, Widget>, Box<dyn Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::to_cache()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` ; Method: `to_wig()` => \
                Failed to get data from cache.",
                meta.model_name
            ))?
        }
        // Get data and return the result.
        Ok(model_cache.unwrap().widget_map.clone())
    }

    /// Get field attributes in Json modelat for page templates.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let json_line = UserProfile::to_json()?;
    /// println!("{}", json_line);
    /// ```
    ///
    fn to_json() -> Result<String, Box<dyn Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::to_cache()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` ; Method: `model_json()` => \
                Failed to get data from cache.",
                meta.model_name
            ))?
        }
        // Generate data and return the result.
        let model_cache = model_cache.unwrap();
        if model_cache.form_json.is_empty() {
            drop(model_store);
            let mut model_store = MODEL_STORE.write()?;
            let model_cache = model_store.get(key.as_str()).unwrap();
            let widget_map = model_cache.widget_map.clone();
            let json = Self::widget_map_to_json(widget_map)?;
            let mut new_model_cache = model_cache.clone();
            new_model_cache.form_json = json.clone();
            model_store.insert(key, new_model_cache);
            return Ok(json);
        }
        //
        Ok(model_cache.form_json.clone())
    }

    /// Json-line for admin panel.
    /// ( converts a widget map to a list, in the order of the Model fields )
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let json_line = UserProfile::to_json_for_admin()?;
    /// println!("{}", json_line);
    /// ```
    ///
    fn model_to_json_for_admin() -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (model_cache, _client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        let widget_map = model_cache.widget_map.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        // Get a list of widgets in the order of the model fields.
        for field_name in meta.fields_name.iter() {
            let mut widget = widget_map.get(field_name).unwrap().clone();
            if field_name == "created_at" || field_name == "updated_at" {
                widget.is_hide = false;
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get Html model of Model for page templates.
    // *********************************************************************************************
    ///
    /// # Example:
    ///
    /// ```
    /// let html = UserProfile::to_html(None, None, None)?;
    /// // OR
    /// let html = UserProfile::to_html(Some("/login"), Some(HttpMethod::POST), Some(Enctype::Multipart))?;
    /// println!("{}", html);
    /// ```
    ///
    fn to_html(
        url_action: Option<&str>,
        http_method: Option<HttpMethod>,
        enctype: Option<Enctype>,
    ) -> Result<String, Box<dyn Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::to_cache()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` > Method: `to_html` => Failed to get data from cache.",
                meta.model_name
            ))?
        }
        // Generate data and return the result.
        let model_cache = model_cache.unwrap();
        if model_cache.form_html.is_empty() {
            drop(model_store);
            let mut model_store = MODEL_STORE.write()?;
            let model_cache = model_store.get(key.as_str()).unwrap();
            let html = Self::generate_html(
                url_action,
                http_method,
                enctype,
                model_cache.meta.service_name.as_str(),
                model_cache.meta.model_name.as_str(),
                &model_cache.meta.fields_name,
                &model_cache.widget_map,
            )?;
            let mut new_model_cache = model_cache.clone();
            new_model_cache.form_html = html.clone();
            model_store.insert(key, new_model_cache);
            return Ok(html);
        }
        //
        Ok(model_cache.form_html.clone())
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
    fn get_cache_data_for_query() -> Result<(ModelCache, Client), Box<dyn Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key()?;
        // Get read access from cache.
        let mut model_store = MODEL_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !model_store.contains_key(key.as_str()) {
            // Unlock.
            drop(model_store);
            // Add metadata and widgects map to cache.
            Self::to_cache()?;
            // Reaccess.
            model_store = MODEL_STORE.read()?;
        }
        // Get model_cache.
        let model_cache = model_store.get(key.as_str());
        if model_cache.is_none() {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` > Method: `get_cache_data_for_query` => \
                Failed to get data from cache.",
                meta.model_name
            ))?
        }
        //
        let model_cache = model_cache.unwrap();
        // Get model metadata from cache.
        let meta: &Meta = &model_cache.meta;
        // Get MongoDB client for current model.
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let client: &Client = client_store.get(&meta.db_client_name).unwrap();
        //
        Ok((model_cache.clone(), client.clone()))
    }

    /// Update data for dynamic widgets.
    /// Hint: For more convenience, use the admin panel - https://github.com/kebasyaty/mango-panel
    ///
    /// # Example:
    ///
    /// ```
    /// // Field attributes for "value":
    /// // minlength - for string type ; Default = 0
    /// // maxlength - for string type ; Default = 256
    /// // min - for numeric type
    /// // max - for numeric type
    ///
    /// let dyn_data = json!({
    ///     "field_name": "field_name",
    ///     "value": 5, // restrict with field attributes
    ///     "title": "Title", // maximum title length = 150 characters
    ///     "is_delete": false
    /// });
    /// assert!(ModelName::update_dyn_wig(dyn_data).is_ok());
    /// ```
    ///
    // *********************************************************************************************
    fn update_dyn_wig(dyn_data: Value) -> Result<(), Box<dyn Error>> {
        //
        // Define conditional constants.
        let const_field_name = {
            if let Some(field_name) = dyn_data["field_name"].as_str() {
                field_name
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_wig` > \
                    Parameter: `dyn_data` > Field: `field_name` => \
                    The field is missing.",
                    Self::meta()?.model_name
                ))?
            }
        };
        let const_title = {
            if let Some(title) = dyn_data["title"].as_str() {
                if title.len() > 150 {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `title` => \
                        The maximum title length is 150 characters.",
                        Self::meta()?.model_name
                    ))?
                }
                title
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_wig` > \
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
                    "Model: {} > Method: `update_dyn_wig` > \
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
        // Get widget map and check the field name for belonging to the Model.
        let const_widget = {
            if let Some(widget_type) = model_cache.widget_map.get(const_field_name) {
                widget_type
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_wig` => \
                    There is no field named `{}` in the model.",
                    meta.model_name, const_field_name
                ))?
            }
        };
        let const_widget_type = const_widget.widget.as_str();
        // Check the Widget type for belonging to dynamic types.
        if !const_widget_type.contains("Dyn") {
            Err(format!(
                "Model: {} > Field: `{}` ; Method: `update_dyn_wig` => \
                Widget `{}` is not dynamic.",
                meta.model_name, const_field_name, const_widget_type
            ))?
        }
        //
        // Get access to the technical base of the project.
        let coll = {
            let mango_tech_keyword = format!(
                "mango_tech__{}__{}",
                meta.project_name.clone(),
                meta.unique_project_key.clone()
            );
            let db = client_cache.database(&mango_tech_keyword);
            db.collection("dynamic_widgets")
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
        // 2.Check that the value type is compatible with the widget type.
        {
            let is_value_exist = if const_widget_type.contains("Text") {
                if let Some(val) = dyn_data["value"].as_str() {
                    let val_len = val.len();
                    if val_len < const_widget.minlength || val_len > const_widget.maxlength {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        Characters = {} ; Min length = {} ; Max length = {}",
                            meta.model_name,
                            val_len,
                            const_widget.minlength,
                            const_widget.maxlength
                        ))?
                    }
                    let arr_vec = target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap())
                        .collect::<Vec<&str>>();
                    arr_vec.contains(&val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
                    Parameter: `dyn_data` > Field: `value` => \
                    The value is not a `&str` type.",
                        meta.model_name
                    ))?
                }
            } else if const_widget_type.contains("I32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if (!const_widget.min.is_empty() && val < const_widget.min.parse::<i64>()?)
                        || (!const_widget.max.is_empty()
                            && val > const_widget.max.parse::<i64>()?)
                    {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        Number = {} ; Min = {} ; Max = {}",
                            meta.model_name, val, const_widget.min, const_widget.max
                        ))?
                    }
                    if val < (i32::MIN as i64) || val > (i32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        The value `{}` is not a `i32` type.",
                            meta.model_name, val
                        ))?
                    }
                    let val = i32::try_from(val)?;
                    let arr_vec = target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i32().unwrap())
                        .collect::<Vec<i32>>();
                    arr_vec.contains(&val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
                    Parameter: `dyn_data` > Field: `value` => \
                    The value is not a `i32` type.",
                        meta.model_name
                    ))?
                }
            } else if const_widget_type.contains("U32") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if (!const_widget.min.is_empty() && val < const_widget.min.parse::<i64>()?)
                        || (!const_widget.max.is_empty()
                            && val > const_widget.max.parse::<i64>()?)
                    {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        Number = {} ; Min = {} ; Max = {}",
                            meta.model_name, val, const_widget.min, const_widget.max
                        ))?
                    }
                    if val < (u32::MIN as i64) || val > (u32::MAX as i64) {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        The value `{}` is not a `u32` type.",
                            meta.model_name, val
                        ))?
                    }
                    let arr_vec = target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    arr_vec.contains(&val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
                    Parameter: `dyn_data` > Field: `value` => \
                    The value is not a `u32` type.",
                        meta.model_name
                    ))?
                }
            } else if const_widget_type.contains("I64") {
                if let Some(val) = dyn_data["value"].as_i64() {
                    if (!const_widget.min.is_empty() && val < const_widget.min.parse::<i64>()?)
                        || (!const_widget.max.is_empty()
                            && val > const_widget.max.parse::<i64>()?)
                    {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        Number = {} ; Min = {} ; Max = {}",
                            meta.model_name, val, const_widget.min, const_widget.max
                        ))?
                    }
                    if val < i64::MIN || val > i64::MAX {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        The value `{}` is not a `i64` type.",
                            meta.model_name, val
                        ))?
                    }
                    let arr_vec = target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>();
                    arr_vec.contains(&val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
                    Parameter: `dyn_data` > Field: `value` => \
                    The value is not a `i64` type.",
                        meta.model_name
                    ))?
                }
            } else if const_widget_type.contains("F64") {
                if let Some(val) = dyn_data["value"].as_f64() {
                    if (!const_widget.min.is_empty() && val < const_widget.min.parse::<f64>()?)
                        || (!const_widget.max.is_empty()
                            && val > const_widget.max.parse::<f64>()?)
                    {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        Number = {} ; Min = {} ; Max = {}",
                            meta.model_name, val, const_widget.min, const_widget.max
                        ))?
                    }
                    if val < f64::MIN || val > f64::MAX {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` > \
                        Parameter: `dyn_data` > Field: `value` => \
                        The value `{}` is not a `f64` type.",
                            meta.model_name, val
                        ))?
                    }
                    let arr_vec = target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .collect::<Vec<f64>>();
                    arr_vec.contains(&val)
                } else {
                    Err(format!(
                        "Model: {} > Method: `update_dyn_wig` > \
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
                    "Model: {} > Field: `{}` ; Method: `update_dyn_wig` => \
                Cannot add new value, similar value already exists.",
                    meta.model_name, const_field_name
                ))?
            }
            if const_is_delete && !is_value_exist {
                Err(format!(
                    "Model: {} > Field: `{}` ; Method: `update_dyn_wig` => \
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
                    if const_widget_type.contains("Text") {
                        if tmp_arr[0].as_str().unwrap() == dyn_data["value"].as_str().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_widget_type.contains("I32") {
                        if tmp_arr[0].as_i32().unwrap()
                            == i32::try_from(dyn_data["value"].as_i64().unwrap())?
                        {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_widget_type.contains("U32") || const_widget_type.contains("I64")
                    {
                        if tmp_arr[0].as_i64().unwrap() == dyn_data["value"].as_i64().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else if const_widget_type.contains("F64") {
                        if tmp_arr[0].as_f64().unwrap() == dyn_data["value"].as_f64().unwrap() {
                            target_arr_bson.remove(idx);
                            break;
                        }
                    } else {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` => \
                            Invalid data type.",
                            meta.model_name,
                        ))?
                    }
                }
            }
        } else {
            // Add dynamic value.
            //
            if const_widget_type.contains("Text") {
                let val_bson = Bson::String(dyn_data["value"].as_str().unwrap().to_string());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_widget_type.contains("I32") {
                let val_bson = Bson::Int32(i32::try_from(dyn_data["value"].as_i64().unwrap())?);
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_widget_type.contains("U32") || const_widget_type.contains("I64") {
                let val_bson = Bson::Int64(dyn_data["value"].as_i64().unwrap());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else if const_widget_type.contains("F64") {
                let val_bson = Bson::Double(dyn_data["value"].as_f64().unwrap());
                let title_bson = Bson::String(const_title.to_string());
                let arr_bson = Bson::Array(vec![val_bson, title_bson]);
                target_arr_bson.push(arr_bson);
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_wig` => \
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
            let const_control_arr = if const_widget_type.contains("Text") {
                ControlArr::Text(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap())
                        .collect::<Vec<&str>>(),
                )
            } else if const_widget_type.contains("I32") {
                ControlArr::I32(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i32().unwrap())
                        .collect::<Vec<i32>>(),
                )
            } else if const_widget_type.contains("U32") || const_widget_type.contains("I64") {
                ControlArr::I64(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_i64().unwrap())
                        .collect::<Vec<i64>>(),
                )
            } else if const_widget_type.contains("F64") {
                ControlArr::F64(
                    target_arr_bson
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_f64().unwrap())
                        .collect::<Vec<f64>>(),
                )
            } else {
                Err(format!(
                    "Model: {} > Method: `update_dyn_wig` => \
                    Invalid data type.",
                    meta.model_name,
                ))?
            };
            //
            let db = client_cache.database(meta.database_name.as_str());
            let coll = db.collection(meta.collection_name.as_str());
            let mut cursor = coll.find(None, None)?;
            // Iterate over all documents in the collection.
            while let Some(doc_from_db) = cursor.next() {
                let mut is_changed = false;
                let mut doc_from_db = doc_from_db?;
                //
                // Skip documents if field value Null.
                if doc_from_db.is_null(const_field_name) {
                    continue;
                }
                // Widgets with support multiple selection.
                if const_widget_type.contains("Mult") {
                    let mut truncated_arr_bson = Vec::<Bson>::new();
                    if const_widget_type.contains("Text") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .map(|item| item.clone())
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_str()
                                    .contains(&item.as_str().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_widget_type.contains("I32") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .map(|item| item.clone())
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_i32()
                                    .contains(&item.as_i32().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_widget_type.contains("U32") || const_widget_type.contains("I64")
                    {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .map(|item| item.clone())
                            .filter(|item| {
                                const_control_arr
                                    .control_arr_i64()
                                    .contains(&item.as_i64().unwrap())
                            })
                            .collect();
                        if truncated_arr_bson.len() != tmp_arr_bson.len() {
                            is_changed = true;
                        }
                    } else if const_widget_type.contains("F64") {
                        let tmp_arr_bson = doc_from_db.get_array(const_field_name)?;
                        truncated_arr_bson = tmp_arr_bson
                            .iter()
                            .map(|item| item.clone())
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
                    // Select widgets with support for one selection.
                    is_changed = if const_widget_type.contains("Text") {
                        let val = doc_from_db.get_str(const_field_name)?;
                        !const_control_arr.control_arr_str().contains(&val)
                    } else if const_widget_type.contains("I32") {
                        let val = doc_from_db.get_i32(const_field_name)?;
                        !const_control_arr.control_arr_i32().contains(&val)
                    } else if const_widget_type.contains("U32") || const_widget_type.contains("I64")
                    {
                        let val = doc_from_db.get_i64(const_field_name)?;
                        !const_control_arr.control_arr_i64().contains(&val)
                    } else if const_widget_type.contains("F64") {
                        let val = doc_from_db.get_f64(const_field_name)?;
                        !const_control_arr.control_arr_f64().contains(&val)
                    } else {
                        Err(format!(
                            "Model: {} > Method: `update_dyn_wig` => \
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

        // Update metadata and widgects map to cache.
        Self::to_cache()?;
        //
        Ok(())
    }
}
