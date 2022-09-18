//! Migrations are green-barrel’s way of propagating changes you make to your models (adding a field, deleting a model, etc.) into your database schema.

use mongodb::{
    bson::{
        de::from_document,
        doc,
        document::Document,
        ser::{to_bson, to_document},
        Bson,
    },
    options::UpdateModifications,
    sync::Client,
    sync::Collection,
    sync::Cursor,
    sync::Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::Metadata, path::Path, sync::RwLockReadGuard};

use crate::{
    models::helpers::{FileData, ImageData, Meta},
    store::MONGODB_CLIENT_STORE,
};

// MIGRATION
// #################################################################################################
/// For creation and updating of a technical database.
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub field_type_map: HashMap<String, String>, // Field type map
    pub status: bool,
}

/// For monitoring the state of models.
pub struct Monitor<'a> {
    pub project_name: &'a str,
    pub unique_project_key: &'a str,
    pub metadata_list: Vec<Meta>,
}

impl<'a> Monitor<'a> {
    /// Get the name of the technical database for a project.
    // *********************************************************************************************
    pub fn green_tech_name(&self) -> Result<String, Box<dyn Error>> {
        // PROJECT_NAME Validation.
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 21
        let re = Regex::new(r"^[a-zA-Z][_a-zA-Z\d]{1,21}$")?;
        if !re.is_match(self.project_name) {
            Err("PROJECT_NAME => \
                    Valid characters: _ a-z A-Z 0-9 and \
                    Max size: 21 ; \
                    First character: a-z A-Z")?
        }
        // UNIQUE_PROJECT_KEY Validation.
        // UNIQUE_PROJECT_KEY - It is recommended not to change.
        // Valid characters: a-z A-Z 0-9
        // Size: 8-16
        // Example: "7rzgacfqQB3B7q7T"
        let re = Regex::new(r"^[a-zA-Z\d]{8,16}$")?;
        if !re.is_match(self.unique_project_key) {
            Err("UNIQUE_PROJECT_KEY => \
                    Valid characters: a-z A-Z 0-9 and \
                    Size: 8-16.")?
        }
        //
        Ok(format!(
            "green_tech__{}__{}",
            self.project_name, self.unique_project_key
        ))
    }

    /// Refresh models state.
    // *********************************************************************************************
    ///
    /// ```
    /// if {
    ///     If there is no technical database, it will be created.
    /// } else {
    ///     Resets the Model's status to `false`.
    /// }
    /// ```
    ///
    fn refresh(&self) -> Result<(), Box<dyn Error>> {
        // Get cache MongoDB clients.
        let client_store: RwLockReadGuard<HashMap<String, Client>> = MONGODB_CLIENT_STORE.read()?;
        //
        for meta in self.metadata_list.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Get the name of the technical database for a project.
            let db_green_tech: String = self.green_tech_name()?;
            // Collection for monitoring the state of Models.
            let collection_models_name: &str = "monitor_models";
            // Used to store selection items, for
            // Fields type like selectTextDyn, selectTextMultDyn, etc.
            let collection_dyn_fields_type: &str = "dynamic_fields";
            //Get a list of databases.
            let database_names: Vec<String> = client.list_database_names(None, None)?;
            // Create a technical database for the project if it doesn't exist.
            if !database_names.contains(&db_green_tech) {
                // Create a collection for models.
                client
                    .database(&db_green_tech)
                    .create_collection(collection_models_name, None)?;
                // Create a collection for fields types of `select`.
                // (selectTextDyn, selectTextMultDyn, etc.)
                client
                    .database(&db_green_tech)
                    .create_collection(collection_dyn_fields_type, None)?;
            } else {
                // Reset models state information.
                let green_tech_db: Database = client.database(&db_green_tech);
                let collection_models: Collection =
                    green_tech_db.collection(collection_models_name);
                let cursor: Cursor = collection_models.find(None, None)?;

                for result in cursor {
                    let document = result?;
                    let mut model_state: ModelState = from_document(document)?;
                    model_state.status = false;
                    let query: Document = doc! {
                        "database": &model_state.database,
                        "collection": &model_state.collection
                    };
                    let update = to_document(&model_state)?;
                    collection_models.update_one(query, update, None)?;
                }
            }
        }
        //
        Ok(())
    }

    /// Reorganize databases state
    /// (full delete of orphaned collections and databases)
    // *********************************************************************************************
    fn napalm(&self) -> Result<(), Box<dyn Error>> {
        // Get cache MongoDB clients.
        let client_store: RwLockReadGuard<HashMap<String, Client>> = MONGODB_CLIENT_STORE.read()?;
        //
        for meta in self.metadata_list.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Get the name of the technical database for a project.
            let db_green_tech: String = self.green_tech_name()?;
            let collection_models_name: &str = "monitor_models";
            let collection_dyn_fields_type: &str = "dynamic_fields";
            let green_tech_db: Database = client.database(&db_green_tech);
            let collection_models: Collection = green_tech_db.collection(collection_models_name);
            let collection_dyn_fields: Collection =
                green_tech_db.collection(collection_dyn_fields_type);
            // Delete orphaned Collections.
            let cursor: Cursor = collection_models.find(None, None)?;
            let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect();
            for result in results {
                let document = result?;
                let model_state: ModelState = from_document(document)?;
                if !model_state.status {
                    // Delete Collection (left without a model).
                    client
                        .database(&model_state.database)
                        .collection(&model_state.collection)
                        .drop(None)?;
                    // Delete a document with a record about the state of
                    // the model from the technical base.
                    let query: Document = doc! {
                        "database": &model_state.database,
                        "collection": &model_state.collection
                    };
                    collection_models.delete_one(query.clone(), None)?;
                    collection_dyn_fields.delete_one(query, None)?;
                }
            }
        }
        //
        Ok(())
    }

    /// Migrating Models -
    // *********************************************************************************************
    /// Check the changes in the models and (if necessary) apply to the database.
    pub fn migrat(&self) -> Result<(), Box<dyn Error>> {
        // Run refresh models state.
        self.refresh()?;
        // Get cache MongoDB clients.
        let client_store: RwLockReadGuard<HashMap<String, Client>> = MONGODB_CLIENT_STORE.read()?;

        // Get model metadata
        for meta in self.metadata_list.iter() {
            // Service_name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{1,31}$")
                .unwrap()
                .is_match(meta.service_name.as_str())
            {
                Err(format!(
                    "Model: `{}` > SERVICE_NAME => \
                        Valid characters: _ a-z A-Z 0-9 \
                        ; Max size: 31 \
                        ; First character: _ a-z A-Z",
                    meta.model_name
                ))?;
            }
            // Database name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{14,62}$")
                .unwrap()
                .is_match(meta.database_name.as_str())
            {
                Err(format!(
                    "Model: `{}` > DATABASE_NAME => \
                        Valid characters: _ a-z A-Z 0-9 \
                        ; Max size: 21 \
                        ; First character: _ a-z A-Z",
                    meta.model_name
                ))?;
            }
            //
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            let fields_name = &meta.fields_name;
            let ignore_fields = &meta.ignore_fields;
            // List field names without `hash` and ignored fields.
            let trunc_fields_name_list = fields_name
                .iter()
                .filter(|item| *item != "hash" && !ignore_fields.contains(*item))
                .collect::<Vec<&String>>();
            // Get the name of the technical database for a project.
            let db_green_tech: String = self.green_tech_name()?;
            let database_names: Vec<String> = client.list_database_names(None, None)?;
            // Map of default values and value types from `value (default)` attribute -
            // <field_name, (field_type, value)>
            let default_value_map: HashMap<String, serde_json::Value> =
                meta.default_value_map.clone();
            // Get map of fields types.
            let field_type_map = &meta.field_type_map;
            // Get truncated map of fields types.
            let trunc_field_type_map = field_type_map.clone();
            trunc_field_type_map
                .clone()
                .retain(|item, _| item != "hash" && !ignore_fields.contains(item));
            // Get a map of fields type from the technical database,
            // from the `monitor_models` collection for current Model.
            let monitor_field_type_map: HashMap<String, String>;

            // Check the field changes in the Model and (if required)
            // update documents in the current Collection.
            // -------------------------------------------------------------------------------------
            // Get a list of current model field names from the technical database
            // `green_barrel_keyword`.
            let filter: Document = doc! {
                "database": &meta.database_name,
                "collection": &meta.collection_name
            };
            let model: Option<Document> = client
                .database(&db_green_tech)
                .collection("monitor_models")
                .find_one(filter, None)?;
            if let Some(model) = model {
                // Get a list of fields from the technical database,
                // from the `monitor_models` collection for current Model.
                let monitor_models_fields_name: Vec<String> = {
                    let fields: Vec<Bson> = model.get_array("fields")?.to_vec();
                    fields
                        .into_iter()
                        .map(|item| item.as_str().unwrap().to_string())
                        .collect()
                };
                // Get a map of fields type from the technical database,
                // from the `monitor_models` collection for current Model.
                monitor_field_type_map = {
                    model
                        .get_document("field_type_map")
                        .unwrap()
                        .iter()
                        .map(|item| (item.0.clone(), item.1.as_str().unwrap().to_string()))
                        .collect()
                };
                // Check if the set of fields in the collection of
                // the current Model needs to be updated.
                let mut changed_fields: Vec<&str> = Vec::new();
                for field in trunc_fields_name_list.iter() {
                    if !monitor_models_fields_name.contains(&field.to_string())
                        || (trunc_field_type_map.get(*field).unwrap()
                            != monitor_field_type_map.get(*field).unwrap_or(&String::new()))
                    {
                        changed_fields.push(field);
                    }
                }
                // Start (if necessary) updating the set of fields in the current collection.
                if !changed_fields.is_empty() {
                    // Get the database and collection of the current Model.
                    let db: Database = client.database(&meta.database_name);
                    let collection: mongodb::sync::Collection =
                        db.collection(&meta.collection_name);
                    // Get cursor to all documents of the current Model.
                    let mut cursor: Cursor = collection.find(None, None)?;
                    // Iterate through all documents in a current (model) collection.
                    while let Some(Ok(doc_from_db)) = cursor.next() {
                        // Create temporary blank document.
                        let mut tmp_doc = Document::new();
                        // Loop over all fields of the model.
                        for (field_name, field_type) in field_type_map.iter() {
                            if field_name == "hash" || ignore_fields.contains(field_name) {
                                continue;
                            }
                            // Insert the reserved fields.
                            if field_name == "created_at" || field_name == "updated_at" {
                                if doc_from_db.contains_key(field_name) {
                                    let value_from_db: Option<&Bson> = doc_from_db.get(field_name);
                                    if let Some(value_from_db) = value_from_db {
                                        tmp_doc.insert(field_name.to_string(), value_from_db);
                                    } else {
                                        Err(format!(
                                            "Service: `{}` > Model: `{}` ; \
                                                Method: `migrat()` => \
                                                Cannot get field value from database for \
                                                field `{}`.",
                                            meta.service_name, meta.model_name, field_name
                                        ))?
                                    }
                                } else {
                                    Err(format!(
                                        "Service: `{}` > Model: `{}` ; Method: `migrat()` => \
                                            Key `{}` was not found in the document from \
                                            the database.",
                                        meta.service_name, meta.model_name, field_name
                                    ))?
                                }
                                //
                                continue;
                            }
                            // If the field exists, get its value.
                            if !changed_fields.contains(&field_name.as_str()) {
                                let value_from_db: Option<&Bson> = doc_from_db.get(field_name);
                                if let Some(value_from_db) = value_from_db {
                                    tmp_doc.insert(field_name.to_string(), value_from_db);
                                } else {
                                    Err(format!(
                                        "Service: `{}` > Model: `{}` > Field: `{}` ; \
                                            Method: `migrat()` => \
                                            Can't get field value from database.",
                                        meta.service_name, meta.model_name, field_name
                                    ))?;
                                }
                            } else {
                                // If no field exists, get default value.
                                let default_value = default_value_map.get(field_name).unwrap();
                                tmp_doc.insert(
                                    field_name.clone(),
                                    match field_type.as_str() {
                                         "RadioText" | "InputColor"
                                        | "InputEmail" | "InputPassword" | "InputPhone"
                                        | "InputText" | "InputUrl" | "InputIP" | "InputIPv4"
                                        | "InputIPv6" | "TextArea" | "SelectText" | "AutoSlug" => {
                                            if !default_value.is_null() {
                                                Bson::String(default_value.as_str().unwrap().to_string())
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "InputDate" => {
                                            // Example: "1970-02-28".
                                            if !default_value.is_null() {
                                                let val = default_value.as_str().unwrap();
                                                if !crate::store::REGEX_IS_DATE.is_match(val) {
                                                    Err(format!("Service: `{}` > Model: `{}` ; \
                                                            Method: `migrat()` => Incorrect date \
                                                            format. Example: 1970-02-28",
                                                        meta.service_name, meta.model_name))?
                                                }
                                                let val = format!("{}T00:00", val);
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            val.as_str(),
                                                            "%Y-%m-%dT%H:%M",
                                                        )?, chrono::Utc,
                                                    );
                                                Bson::DateTime(dt)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "InputDateTime" | "HiddenDateTime" => {
                                            // Example: "1970-02-28T00:00".
                                            if !default_value.is_null() {
                                                let val = default_value.as_str().unwrap();
                                                if !crate::store::REGEX_IS_DATETIME.is_match(val) {
                                                    Err(format!("Service: `{}` > Model: `{}` ; \
                                                            Method: `migrat()` => \
                                                            Incorrect date and time format. \
                                                            Example: 1970-02-28T00:00",
                                                        meta.service_name, meta.model_name
                                                    ))?
                                                }
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            val,
                                                            "%Y-%m-%dT%H:%M",
                                                        )?, chrono::Utc,
                                                    );
                                                Bson::DateTime(dt)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "RadioI32" | "NumberI32" | "RangeI32" | "SelectI32" => {
                                            if !default_value.is_null() {
                                                Bson::Int32(
                                                    i32::try_from(default_value.as_i64().unwrap())?
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "RadioU32" | "NumberU32" | "RangeU32"
                                        | "SelectU32" | "RadioI64" | "NumberI64"
                                        | "RangeI64" | "SelectI64" => {
                                            if !default_value.is_null() {
                                                Bson::Int64(
                                                    default_value.as_i64().unwrap()
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "RadioF64" | "NumberF64" | "RangeF64" 
                                        | "SelectF64" => {
                                            if !default_value.is_null() {
                                                Bson::Double(
                                                   default_value.as_f64().unwrap()
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "CheckBox" => {
                                            if !default_value.is_null() {
                                                Bson::Boolean(
                                                    default_value.as_bool().unwrap()
                                                )
                                            } else {
                                                Bson::Boolean(false)
                                            }
                                        }
                                        "InputFile" => {
                                            if !default_value.is_null() {
                                                let mut file_data = serde_json::from_value::<FileData>(default_value.clone())?;
                                                // Define flags to check.
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                        `migrat()` => Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field_name)
                                                    )?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; \
                                                    Method: `migrat()` => File is missing - {}",
                                                        meta.model_name, field_name, path)
                                                    )?
                                                }
                                                // Get file metadata.
                                                let metadata: Metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as u32;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Create doc.
                                                let result = to_document(&file_data)?;
                                                Bson::Document(result)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "InputImage" => {
                                            if !default_value.is_null() {
                                                let mut file_data = serde_json::from_value::<ImageData>(default_value.clone())?;
                                                // Define flags to check.
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                        `migrat()` => Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field_name
                                                    ))?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                            `migrat()` => File is missing - {}",
                                                        meta.model_name, field_name, path
                                                    ))?
                                                }
                                                // Get file metadata.
                                                let metadata: Metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as u32;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Get image width and height.
                                                let dimensions: (u32, u32) = image::image_dimensions(path)?;
                                                file_data.width = dimensions.0;
                                                file_data.height = dimensions.1;
                                                // Create doc.
                                                let result = to_document(&file_data)?;
                                                Bson::Document(result)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "SelectTextMult" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<String>>(default_value.clone())?
                                                    .iter().map(|item| Bson::String(item.clone()))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "SelectI32Mult" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<i32>>(default_value.clone())?
                                                    .iter().map(|item| Bson::Int32(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                         "SelectU32Mult" | "SelectI64Mult"  => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<i64>>(default_value.clone())?
                                                    .iter().map(|item| mongodb::bson::Bson::Int64(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "SelectF64Mult" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<f64>>(default_value.clone())?
                                                    .iter().map(|item| Bson::Double(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "SelectTextDyn" | "SelectTextMultDyn" | "SelectI32Dyn"
                                        | "SelectI32MultDyn" | "SelectU32Dyn" | "SelectU32MultDyn"
                                        | "SelectI64Dyn" | "SelectI64MultDyn" | "SelectF64Dyn"
                                        | "SelectF64MultDyn" => {
                                            Bson::Null
                                        }
                                        _ => {
                                            Err(format!("Service: `{}` > Model: `{}` ; \
                                                    Method: `migrat()` => Invalid Field type.",
                                                meta.service_name, meta.model_name
                                            ))?
                                        }
                                    },
                                );
                            }
                        }
                        // Save updated document.
                        let query = doc! {"_id": doc_from_db.get_object_id("_id")?};
                        collection.update_one(query, tmp_doc, None)?;
                    }
                }
            } else {
                monitor_field_type_map = HashMap::new();
            }

            // Create a new database (if doesn't exist) and add new collection.
            // -------------------------------------------------------------------------------------
            // Get the database for the current collection of Model.
            let db: Database = client.database(&meta.database_name);
            // If there is no collection for the current Model, create it.
            if !database_names.contains(&meta.database_name)
                || !db
                    .list_collection_names(None)?
                    .contains(&meta.collection_name)
            {
                db.create_collection(&meta.collection_name, None)?;
            }

            // Get the technical database `db_green_tech` for the current model.
            // -------------------------------------------------------------------------------------
            let db: Database = client.database(&db_green_tech);

            // Update the state of models for `models::Monitor`.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_green_tech)
                || !db
                    .list_collection_names(None)?
                    .contains(&"monitor_models".to_owned())
            {
                Err("In the `refresh()` method, \
                        no technical database has been created for the project.")?
            } else {
                let collection: Collection = db.collection("monitor_models");
                let filter = doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name
                };
                let doc: Document = mongodb::bson::doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name,
                    "fields": trunc_fields_name_list.iter().map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                    "field_type_map": to_bson(&trunc_field_type_map.clone())?,
                    "status": true
                };
                // Check if there is model state in the database.
                if collection.count_documents(filter.clone(), None)? == 0_i64 {
                    // Add model state information.
                    collection.insert_one(doc, None)?;
                } else {
                    // Full update model state information.
                    let update = UpdateModifications::Document(doc);
                    collection.update_one(filter, update, None)?;
                }
            }

            // Document management to support model fields with dynamic fields type.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_green_tech)
                || !db
                    .list_collection_names(None)?
                    .contains(&"dynamic_fields".to_owned())
            {
                Err("In the `refresh()` method, \
                        no technical database has been created for the project.")?
            }
            //
            let collection: Collection = db.collection("dynamic_fields");
            let filter = doc! {
                "database": &meta.database_name,
                "collection": &meta.collection_name
            };
            // Check if there is a document in the database for
            // storing the values of dynamic fields type of model.
            if collection.count_documents(filter.clone(), None)? == 0_i64 {
                // Init new document.
                let mut new_doc = doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name,
                    "fields": {}
                };
                // Add empty arrays to the new document.
                let mut fields_doc = Document::new();
                for (field_name, field_type) in field_type_map.clone() {
                    if field_type.contains("Dyn") {
                        fields_doc.insert(field_name, Bson::Array(Vec::new()));
                    }
                }
                // Insert new document.
                new_doc.insert("fields".to_string(), fields_doc);
                collection.insert_one(new_doc, None)?;
            } else {
                // Get an existing document.
                let mut exist_doc = collection.find_one(filter.clone(), None)?.unwrap();
                // Get a document with `dynamic_fields` fields.
                let fields_doc = exist_doc.get_document_mut("fields")?;
                // Get a list of fields from the technical database,
                // from the `dynamic_fields` collection for current Model.
                let dyn_fields_from_db: Vec<String> =
                    fields_doc.keys().map(|item| item.into()).collect();
                // Create an empty list for fields with dynamic field types.
                let mut dyn_fields_from_model: Vec<String> = Vec::new();
                // Add new (if any) fields in `fields_doc`.
                for (field_name, field_type) in trunc_field_type_map.clone() {
                    if field_type.contains("Dyn") {
                        dyn_fields_from_model.push(field_name.clone());
                        // If the new field or fields type do not match,
                        // initialize with an empty array.
                        if !dyn_fields_from_db.contains(&field_name)
                            || (field_type
                                != *monitor_field_type_map
                                    .get(field_name.as_str())
                                    .unwrap_or(&String::new()))
                        {
                            fields_doc.insert(field_name, Bson::Array(Vec::new()));
                        }
                    }
                }
                // Remove orphaned fields.
                for field_name in dyn_fields_from_db {
                    if !dyn_fields_from_model.contains(&field_name) {
                        fields_doc.remove(&field_name).unwrap();
                    }
                }
                // Full update existing document.
                collection.update_one(filter, exist_doc, None)?;
            }
        }

        // Unlock.
        drop(client_store);
        // Run reorganize databases state.
        self.napalm()?;
        //
        Ok(())
    }
}
