//! # Caching.
//! Caching information about Models for speed up work.
//!
//! Trait:
//! `Caching` - Methods caching information about Models for speed up work.
//! Methods:
//! `form_wig` - Get an widgets map for page template.
//! `form_json` - Get Form attributes in Json format for page templates.
//! `form_html` - Get Html Form of Model for page templates.
//! `get_cache_data_for_query` - Get cached Model data.
//! `db_update_dyn_widgets` - Accepts json-line to update data, for dynamic widgets.
//!

use crate::{
    forms::Widget,
    models::{Meta, ToModel},
    store::{FormCache, DB_MAP_CLIENT_NAMES, FORM_CACHE},
};

// Caching information about Models for speed up work.
// #################################################################################################
pub trait CachingModel: ToModel {
    // Get an widgets map for page template.
    // *********************************************************************************************
    fn form_wig() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        Ok(form_cache.unwrap().map_widgets.clone())
    }

    // Get Form attributes in Json format for page templates.
    // *********************************************************************************************
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there.
        if form_cache.attrs_json.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let json_line = serde_json::to_string(&map_widgets)?;
            let mut form_cache: FormCache = form_cache.clone();
            // Update data.
            form_cache.attrs_json = json_line.clone();
            // Save data to cache.
            form_store.insert(key, form_cache.clone());
            // Return result.
            return Ok(json_line);
        }
        Ok(form_cache.attrs_json.clone())
    }

    // Get Html Form of Model for page templates.
    // *********************************************************************************************
    fn form_html() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get access to the cache.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock().unwrap();
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Add an empty `FormCache` structure to the cache if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Accessing cached MongoDB clients.
            let client_store: std::sync::MutexGuard<
                '_,
                std::collections::HashMap<String, mongodb::sync::Client>,
            > = DB_MAP_CLIENT_NAMES.lock()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Add attributes in json format to cache if they are not there.
        if form_cache.controls_html.is_empty() {
            let map_widgets: std::collections::HashMap<String, Widget> =
                form_cache.map_widgets.clone();
            let controls: String = Self::to_html(&form_cache.meta.fields_name, map_widgets);
            let mut form_cache: FormCache = form_cache.clone();
            form_cache.controls_html = controls.clone();
            form_store.insert(key, form_cache.clone());
            return Ok(controls);
        }
        Ok(form_cache.controls_html.clone())
    }

    // Get cached Model data.
    // *********************************************************************************************
    fn get_cache_data_for_query(
    ) -> Result<(FormCache, mongodb::sync::Client), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Access to the cached model data.
        let mut form_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, FormCache>,
        > = FORM_CACHE.lock()?;
        let mut form_cache: Option<&FormCache> = form_store.get(&key[..]);
        // Accessing cached MongoDB clients.
        let client_store: std::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, mongodb::sync::Client>,
        > = DB_MAP_CLIENT_NAMES.lock()?;
        // Add the `FormCache` structure to the cache for the current model if it is not there.
        if form_cache.is_none() {
            // Create `FormCache` default and add map of widgets and metadata of model.
            let meta: Meta = Self::meta()?;
            // Get MongoDB client for current model.
            let client_cache: &mongodb::sync::Client =
                client_store.get(&meta.db_client_name).unwrap();
            // Get a widget map.
            let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
            // Enrich the widget map with values for dynamic widgets.
            Self::vitaminize(
                meta.keyword.as_str(),
                meta.collection_name.as_str(),
                &client_cache,
                &mut map_widgets,
            )?;
            // Init new FormCache.
            let new_form_cache = FormCache {
                meta,
                map_widgets,
                ..Default::default()
            };
            // Save structure `FormCache` to store.
            form_store.insert(key.clone(), new_form_cache);
            // Update the state of a variable.
            form_cache = form_store.get(&key[..]);
        }
        let form_cache: &FormCache = form_cache.unwrap();
        // Get model metadata from cache.
        let meta: &Meta = &form_cache.meta;
        // Get MongoDB client for current model.
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        // Return result
        Ok((form_cache.clone(), client_cache.clone()))
    }

    // Accepts json-line to update data, for dynamic widgets.
    // Hint: Used in conjunction with the admin panel.
    // Example (json_line): {"field_name":[["value","Title"]]}
    // or
    // r#"{
    //    "field_name":[["value","Title"]],
    //    "field_name_2":[["value","Title 2"]],
    //    "field_name_3":[["value","Title 3"]]
    // }"#
    // *********************************************************************************************
    fn db_update_dyn_widgets(json_line: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Refresh the state in the technical database.
        // -----------------------------------------------------------------------------------------
        // Validation json-line.
        let re = regex::RegexBuilder::new(r#"^\{[\s]*(?:"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*\]))(?:,[\s]*"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*\]))*[\s]*\}$"#)
            .case_insensitive(true)
            .build()
            .unwrap();
        if !re.is_match(json_line) {
            Err(format!(
                r#"Model: {} > Method: `db_update_dyn_widgets()` : \
                The `json_line` parameter was not validation. \
                Example: {{"field_name":[["value","Title"]]}}"#,
                Self::meta()?.model_name
            ))?
        }
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        let mango_tech_keyword = format!("mango_tech__{}", meta.keyword.clone());
        let db = client_cache.database(&mango_tech_keyword);
        let coll = db.collection("dynamic_widgets");
        let query = mongodb::bson::doc! {
            "database": meta.database_name.clone(),
            "collection": meta.collection_name.clone()
        };
        let json_data: serde_json::Value = serde_json::from_str(json_line)?;
        let bson_data = serde_json::from_value::<mongodb::bson::Bson>(json_data)?;
        let new_doc = mongodb::bson::doc! {
            "fields": bson_data.clone()
        };
        let update: mongodb::bson::document::Document = mongodb::bson::doc! {
            "$set": new_doc,
        };
        coll.update_one(query, update, None)?;

        // Clean up orphaned (if any) data.
        // -----------------------------------------------------------------------------------------
        let db = client_cache.database(meta.database_name.as_str());
        let coll = db.collection(meta.collection_name.as_str());
        let mut cursor = coll.find(None, None)?;
        let dyn_doc = bson_data.as_document().unwrap();
        // Iterate over all documents in the collection.
        while let Some(db_doc) = cursor.next() {
            let mut is_changed = false;
            let mut curr_doc = db_doc.clone()?;
            // Iterate over all fields in the document.
            for (field_name, widget_type) in meta.map_widget_type.clone() {
                // Choosing the only dynamic widgets.
                if widget_type.contains("Dyn") {
                    if curr_doc.is_null(field_name.as_str()) {
                        continue;
                    }
                    // Get a list of values to match.
                    let dyn_vec: Vec<String> = dyn_doc
                        .get_array(field_name.as_str())?
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap().to_string())
                        .collect();
                    // Selecting widgets with multi-selection support.
                    if widget_type.contains("Mult") {
                        let mut new_arr_bson = Vec::<mongodb::bson::Bson>::new();
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
                                curr_doc
                                    .insert(field_name, mongodb::bson::Bson::Array(new_arr_bson));
                            } else {
                                curr_doc.insert(field_name, mongodb::bson::Bson::Null);
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
                            curr_doc.insert(field_name, mongodb::bson::Bson::Null);
                            is_changed = true;
                        }
                    }
                }
            }
            if is_changed {
                let query = mongodb::bson::doc! {"_id": curr_doc.get_object_id("_id")?};
                coll.update_one(query, curr_doc, None)?;
            }
        }

        Ok(())
    }
}
