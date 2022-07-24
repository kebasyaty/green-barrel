//! Caching inmodelation about Models for speed up work.

use mongodb::{
    bson::{doc, document::Document, Bson},
    sync::Client,
};
use std::{collections::HashMap, error::Error};

use crate::{
    models::{converters::Converters, Main, Meta},
    store::{ModelCache, MODEL_STORE, MONGODB_CLIENT_STORE},
    widgets::{generate_html::GenerateHtml, Enctype, HttpMethod, Widget},
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

    /// Accepts json-line to update data, for dynamic widgets.
    /// Hint: Used in conjunction with the admin panel.
    ///
    /// # Example:
    ///
    /// ```
    /// let json_line =  r#"{"field_name":[["value","Title"], ...]}"#;
    /// // or
    /// let json_line = r#"{
    ///        "field_name":[["value","Title"], ...],
    ///        "field_name_2":[["value","Title 2"], ...],
    ///        "field_name_3":[["value","Title 3"], ...]
    ///     }"#;
    ///
    /// assert!(Dynamic::db_update_dyn_widgets(json_line).is_ok());
    /// ```
    ///
    // *********************************************************************************************
    fn db_update_dyn_widgets(json_line: &str) -> Result<(), Box<dyn Error>> {
        // Refresh the state in the technical database.
        // -----------------------------------------------------------------------------------------
        // Validation json-line.
        let re = regex::RegexBuilder::new(r#"^\{[\s]*(?:"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+","[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+"\])(?:,\["[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+","[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+"\])*)*\]))(?:,[\s]*"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+","[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+"\])(?:,\["[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+","[-_.,`@#$%^&+=*!~)(:><?;№|\\/\s\w]+"\])*)*\]))*[\s]*\}$"#)
            .case_insensitive(true)
            .build()
            .unwrap();
        if !re.is_match(json_line) {
            Err(format!(
                r#"Model: {} > Method: `db_update_dyn_widgets` => \
                   The `json_line` parameter was not validation. \
                   Example: {{"field_name":[["value","Title"]]}}"#,
                Self::meta()?.model_name
            ))?
        }

        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = model_cache.meta;
        let mango_tech_keyword = format!(
            "mango_tech__{}__{}",
            meta.project_name.clone(),
            meta.unique_project_key.clone()
        );
        let db = client_cache.database(&mango_tech_keyword);
        let coll = db.collection("dynamic_widgets");
        let query = doc! {
            "database": meta.database_name.clone(),
            "collection": meta.collection_name.clone()
        };
        let new_dyn_data: serde_json::Value = serde_json::from_str(json_line)?;
        let new_dyn_data = serde_json::from_value::<Document>(new_dyn_data)?;
        let mut curr_dyn_date = coll.find_one(query.clone(), None)?.unwrap();
        let dyn_date = curr_dyn_date.get_document_mut("fields").unwrap();

        for (field_name, bson_val) in new_dyn_data {
            dyn_date.insert(field_name.as_str(), bson_val);
        }

        let update = doc! {
            "$set": { "fields": dyn_date.clone() }
        };
        coll.update_one(query, update, None)?;

        // Clean up orphaned (if any) data.
        // -----------------------------------------------------------------------------------------
        let db = client_cache.database(meta.database_name.as_str());
        let coll = db.collection(meta.collection_name.as_str());
        let mut cursor = coll.find(None, None)?;
        // Iterate over all documents in the collection.
        while let Some(db_doc) = cursor.next() {
            let mut is_changed = false;
            let mut curr_doc = db_doc.clone()?;
            // Iterate over all fields in the document.
            for (field_name, widget_type) in meta.widget_type_map.clone() {
                // Choosing the only dynamic widgets.
                if widget_type.contains("Dyn") {
                    if curr_doc.is_null(field_name.as_str()) {
                        continue;
                    }
                    // Get a list of values to match.
                    let dyn_vec: Vec<String> = dyn_date
                        .get_array(field_name.as_str())?
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap().to_string())
                        .collect();
                    // Selecting widgets with multi-selection support.
                    if widget_type.contains("Mult") {
                        let mut new_arr_bson = Vec::<Bson>::new();
                        if widget_type.contains("Text") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_str().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("I32") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_i32().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("U32") || widget_type.contains("I64") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_i64().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("F64") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_f64().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        }
                        if is_changed {
                            if !new_arr_bson.is_empty() {
                                curr_doc.insert(field_name, Bson::Array(new_arr_bson));
                            } else {
                                curr_doc.insert(field_name, Bson::Null);
                            }
                        }
                    } else {
                        let mut val = String::new();
                        // Select widgets with support for one selection.
                        if widget_type.contains("Text") {
                            val = curr_doc.get_str(field_name.as_str())?.to_string();
                        } else if widget_type.contains("I32") {
                            val = curr_doc.get_i32(field_name.as_str())?.to_string();
                        } else if widget_type.contains("U32") || widget_type.contains("I64") {
                            val = curr_doc.get_i64(field_name.as_str())?.to_string();
                        } else if widget_type.contains("F64") {
                            val = curr_doc.get_f64(field_name.as_str())?.to_string();
                        }
                        if !dyn_vec.contains(&val) {
                            curr_doc.insert(field_name, Bson::Null);
                            is_changed = true;
                        }
                    }
                }
            }
            if is_changed {
                // Update values for dynamic widgets.
                // ---------------------------------------------------------------------------------
                let query = doc! {"_id": curr_doc.get_object_id("_id")?};
                coll.update_one(query, curr_doc, None)?;
            }
        }

        // Update metadata and widgects map to cache.
        // -----------------------------------------------------------------------------------------
        Self::to_cache()?;
        //
        Ok(())
    }
}
