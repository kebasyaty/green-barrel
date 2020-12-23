//! # Migration
//!
//! `Monitor` - Creation and updating of a technical database for monitoring the state of models.
//! `ModelState` - Creation and updating of a technical database for monitoring the state of models.

use crate::{
    forms::{FileData, ImageData},
    store::DB_MAP_CLIENT_NAMES,
};
use mongodb::{
    bson, bson::document::Document, options::UpdateModifications, sync::Client, sync::Collection,
    sync::Cursor, sync::Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

// MIGRATION
// #################################################################################################
// Creation and updating of a technical database for monitoring the state of models
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub status: bool,
}

pub struct Monitor<'a> {
    pub keyword: &'a str,
    pub models: Vec<crate::models::Meta>,
}

impl<'a> Monitor<'a> {
    // Get mango tech name
    // *********************************************************************************************
    pub fn mango_tech_name(&self) -> String {
        // Keyword Validation.
        // KEYWORD - It is recommended not to change.
        // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
        // Example: "PROJECT_NAME_7rzg_cfqQB3B7q7T"
        let re = Regex::new(r"^[_a-zA-Z\d]{6,48}$").unwrap();
        if !re.is_match(self.keyword) {
            panic!("Keyword - Valid characters: _ a-z A-Z 0-9 ; Size: 6-48.");
        }
        format!("mango_tech__{}", self.keyword)
    }

    // Refresh models state
    // *********************************************************************************************
    fn refresh(
        &self,
        client_store: &std::sync::MutexGuard<'_, std::collections::HashMap<String, Client>>,
    ) {
        for meta in self.models.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Establish a connection with the technical database of the project
            let mango_tech_keyword: String = self.mango_tech_name();
            let collection_name: &str = "models";
            let database_names: Vec<String> = client.list_database_names(None, None).unwrap();
            // Create a technical database for the project if it doesn't exist
            if !database_names.contains(&mango_tech_keyword) {
                client
                    .database(&mango_tech_keyword)
                    .create_collection(collection_name, None)
                    .unwrap();
            } else {
                // Reset model state information
                let mango_orm_db: Database = client.database(&mango_tech_keyword);
                let mango_orm_collection: Collection = mango_orm_db.collection(collection_name);
                let mut cursor: Cursor = mango_orm_collection.find(None, None).unwrap();

                while let Some(result) = cursor.next() {
                    match result {
                        Ok(document) => {
                            let mut model_state: ModelState =
                                bson::de::from_document(document).unwrap();
                            model_state.status = false;
                            let query: Document = bson::doc! {
                                "database": &model_state.database,
                                "collection": &model_state.collection
                            };
                            let update: UpdateModifications = UpdateModifications::Document(
                                bson::ser::to_document(&model_state).unwrap(),
                            );
                            mango_orm_collection
                                .update_one(query, update, None)
                                .unwrap();
                        }
                        Err(err) => panic!("Migration `refresh()` > {}", err),
                    }
                }
            }
        }
    }

    // Reorganize databases state
    // (full delete of orphaned collections and databases)
    // *********************************************************************************************
    fn napalm(
        &self,
        client_store: &std::sync::MutexGuard<'_, std::collections::HashMap<String, Client>>,
    ) {
        for meta in self.models.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Establish a connection with the technical database of the project
            let mango_tech_keyword: String = self.mango_tech_name();
            let collection_name: &str = "models";
            let mango_tech_db: Database = client.database(&mango_tech_keyword);
            let mango_tech_collection: Collection = mango_tech_db.collection(collection_name);
            // Delete orphaned Collections
            let cursor: Cursor = mango_tech_collection.find(None, None).unwrap();
            let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect();
            for result in results {
                match result {
                    Ok(document) => {
                        let model_state: ModelState = bson::de::from_document(document).unwrap();
                        if !model_state.status {
                            // Delete Collection (left without a model)
                            client
                                .database(&model_state.database)
                                .collection(&model_state.collection)
                                .drop(None)
                                .unwrap();
                            // Delete a document with a record about the state of
                            // the model from the technical base
                            let query: Document = bson::doc! {
                                "database": &model_state.database,
                                "collection": &model_state.collection
                            };
                            mango_tech_collection.delete_one(query, None).unwrap();
                        }
                    }
                    Err(err) => panic!("Migration `napalm()` > {}", err),
                }
            }
        }
    }

    // Migrating Models
    // *********************************************************************************************
    // 1.Checking widgets for correct attribute values and default values.
    // 2.Check model changes and (if required) apply to the database.
    pub fn migrat(&self) {
        // Get cache MongoDB clients
        let client_store: std::sync::MutexGuard<'_, std::collections::HashMap<String, Client>> =
            DB_MAP_CLIENT_NAMES.lock().unwrap();
        // Run refresh models state
        self.refresh(&client_store);

        // Run the migration process for registered models
        for meta in self.models.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            let fields_name: Vec<&str> =
                meta.fields_name.iter().map(|item| item.as_str()).collect();
            let ignore_fields: Vec<&str> = meta
                .ignore_fields
                .iter()
                .map(|item| item.as_str())
                .collect();
            // List field names without `hash` and ignored fields
            let trunc_list_fields_name: Vec<&str> = fields_name
                .iter()
                .filter(|item| **item != "hash" && !ignore_fields.contains(item))
                .map(|item| *item)
                .collect();
            // Name of the technical database of the project
            let mango_tech_keyword: String = self.mango_tech_name();
            let database_names: Vec<String> = client.list_database_names(None, None).unwrap();
            // Map of default values and value types from `value (default)` attribute -
            // <field_name, (widget_type, value)>
            let map_default_values: std::collections::HashMap<String, (String, String)> =
                meta.map_default_values.clone();

            // Check the field changes in the Model and (if required)
            // update documents in the current Collection
            // -------------------------------------------------------------------------------------
            // Get a list of current model field names from the technical database
            // `mango_orm_keyword`
            let filter: Document = mongodb::bson::doc! {
                "database": &meta.database_name,
                "collection": &meta.collection_name
            };
            let model: Option<Document> = client
                .database(&mango_tech_keyword)
                .collection("models")
                .find_one(filter, None)
                .unwrap();
            if model.is_some() {
                // Get a list of fields from the technical database
                let mango_orm_fnames: Vec<String> = {
                    let model: Document = model.unwrap();
                    let fields: Vec<mongodb::bson::Bson> =
                        model.get_array("fields").unwrap().to_vec();
                    fields
                        .into_iter()
                        .map(|item: mongodb::bson::Bson| item.as_str().unwrap().to_string())
                        .collect()
                };
                // Check if the set of fields in the collection of
                // the current Model needs to be updated
                let mut run_documents_modification: bool = false;
                if trunc_list_fields_name.len() != mango_orm_fnames.len() {
                    run_documents_modification = true;
                } else {
                    for item in trunc_list_fields_name.iter() {
                        if mango_orm_fnames.iter().any(|item2| item2 != item) {
                            run_documents_modification = true;
                            break;
                        }
                    }
                }
                // Start (if necessary) updating the set of fields in the current collection
                if run_documents_modification {
                    // Get the database and collection of the current Model
                    let db: Database = client.database(&meta.database_name);
                    let collection: mongodb::sync::Collection =
                        db.collection(&meta.collection_name);
                    // Get cursor to all documents of the current Model
                    let mut cursor: mongodb::sync::Cursor = collection.find(None, None).unwrap();
                    // Iterate through all documents in a current (model) collection
                    while let Some(result) = cursor.next() {
                        let doc_from_db: mongodb::bson::document::Document = result.unwrap();
                        // Create temporary blank document
                        let mut tmp_doc = mongodb::bson::document::Document::new();
                        // Loop over all fields of the model
                        for field in fields_name.iter() {
                            if *field == "hash" || ignore_fields.contains(&field) {
                                continue;
                            }
                            // If the field exists, get its value
                            if doc_from_db.contains_key(field) {
                                let value_from_db: Option<&mongodb::bson::Bson> =
                                    doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    panic!(
                                        "Service: `{}` > Model: `{}` > Field: `{}` > \
                                        Method: `migrat()` : \
                                        Can't get field value from database.",
                                        meta.service_name, meta.model_name, field
                                    );
                                }
                            } else {
                                // If no field exists, get default value
                                let value = map_default_values.get(*field).unwrap();
                                tmp_doc.insert(
                                    field.to_string(),
                                    match value.0.as_str() {
                                        "checkBoxText" | "radioText" | "inputColor"
                                        | "inputEmail" | "inputPassword" | "inputPhone"
                                        | "inputText" | "inputUrl" | "inputIP" | "inputIPv4"
                                        | "inputIPv6" | "textArea" | "selectText" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::String(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputDate" => {
                                            // Example: "1970-02-28"
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !crate::store::REGEX_IS_DATE.is_match(&val) {
                                                    panic!(
                                                        "Service: `{}` > Model: `{}` > \
                                                    Method: `widgets()` : Incorrect date \
                                                    format. Example: 1970-02-28",
                                                        meta.service_name, meta.model_name
                                                    )
                                                }
                                                let val = format!("{}T00:00", val);
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            &val,
                                                            "%Y-%m-%dT%H:%M",
                                                        )
                                                        .unwrap(),
                                                        chrono::Utc,
                                                    );
                                                mongodb::bson::Bson::DateTime(dt)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputDateTime" => {
                                            // Example: "1970-02-28T00:00"
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !crate::store::REGEX_IS_DATETIME.is_match(&val) {
                                                    panic!(
                                                        "Service: `{}` > Model: `{}` > \
                                                    Method: `widgets()` : \
                                                    Incorrect date and time format. \
                                                    Example: 1970-02-28T00:00",
                                                        meta.service_name, meta.model_name
                                                    )
                                                }
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            &val,
                                                            "%Y-%m-%dT%H:%M",
                                                        )
                                                        .unwrap(),
                                                        chrono::Utc,
                                                    );
                                                mongodb::bson::Bson::DateTime(dt)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "checkBoxI32" | "inputRadioI32" | "numberI32"
                                        | "rangeI32" | "selectI32" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Int32(
                                                    val.parse::<i32>().unwrap(),
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "checkBoxU32" | "radioU32" | "numberU32" | "rangeU32"
                                        | "selectU32" | "checkBoxI64" | "radioI64"
                                        | "numberI64" | "rangeI64" | "selectI64" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Int64(
                                                    val.parse::<i64>().unwrap(),
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "checkBoxF64" | "radioF64" | "numberF64" | "rangeF64"
                                        | "selectF64" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Double(
                                                    val.parse::<f64>().unwrap(),
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "checkBoxBool" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Boolean(
                                                    val.parse::<bool>().unwrap(),
                                                )
                                            } else {
                                                mongodb::bson::Bson::Boolean(false)
                                            }
                                        }
                                        "inputFile" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let mut file_data = 
                                                    serde_json::from_str::<FileData>(val.as_str()).unwrap();
                                                // Define flags to check
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    panic!(
                                                        "Model: `{}` > Field: `{}` > Method: \
                                                        `migrat()` : Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field
                                                    );
                                                }
                                                // Create path for validation of file
                                                let path: String = file_data.path.clone();
                                                let f_path = std::path::Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    panic!(
                                                        "Model: `{}` > Field: `{}` > Method: \
                                                        `migrat()` : File is missing - {}",
                                                        meta.model_name, field, path
                                                    )
                                                }
                                                // Get file metadata
                                                let metadata: std::fs::Metadata = f_path.metadata().unwrap();
                                                // Get file size in bytes
                                                file_data.size = metadata.len() as u32;
                                                // Get file name
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Create doc
                                                let result = mongodb::bson::ser::to_document(&file_data).unwrap();
                                                mongodb::bson::Bson::Document(result)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputImage" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let mut file_data = 
                                                    serde_json::from_str::<ImageData>(val.as_str()).unwrap();
                                                // Define flags to check
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    panic!(
                                                        "Model: `{}` > Field: `{}` > Method: \
                                                        `migrat()` : Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field
                                                    );
                                                }
                                                // Create path for validation of file
                                                let path: String = file_data.path.clone();
                                                let f_path = std::path::Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    panic!(
                                                        "Model: `{}` > Field: `{}` > Method: \
                                                        `migrat()` : File is missing - {}",
                                                        meta.model_name, field, path
                                                    )
                                                }
                                                // Get file metadata
                                                let metadata: std::fs::Metadata = f_path.metadata().unwrap();
                                                // Get file size in bytes
                                                file_data.size = metadata.len() as u32;
                                                // Get file name
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Get image width and height
                                                let dimensions: (u32, u32) = image::image_dimensions(path).unwrap();
                                                file_data.width = dimensions.0;
                                                file_data.height = dimensions.1;
                                                // Create doc
                                                let result = mongodb::bson::ser::to_document(&file_data).unwrap();
                                                mongodb::bson::Bson::Document(result)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        _ => panic!(
                                            "Service: `{}` > Model: `{}` > Method: \
                                            `migrat()` : Invalid Widget type.",
                                            meta.service_name, meta.model_name
                                        ),
                                    },
                                );
                            }
                        }
                        // Insert fields for timestamps `created_at` and `updated_at`
                        for field in vec!["created_at", "updated_at"] {
                            if doc_from_db.contains_key(field) {
                                let value_from_db: Option<&mongodb::bson::Bson> =
                                    doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    panic!(
                                        "Service: `{}` > Model: `{}` > \
                                        Method: `migrat()` : \
                                        Cannot get field value from database for \
                                        field `{}`.",
                                        meta.service_name, meta.model_name, field
                                    );
                                }
                            } else {
                                panic!(
                                    "Service: `{}` > Model: `{}` > Method: `migrat()` : \
                                    Key `{}` was not found in the document from \
                                    the database.",
                                    meta.service_name, meta.model_name, field
                                );
                            }
                        }
                        // Save updated document
                        let query =
                            mongodb::bson::doc! {"_id": doc_from_db.get_object_id("_id").unwrap()};
                        let mut update: Document = mongodb::bson::document::Document::new();
                        update.insert("$set".to_string(), mongodb::bson::Bson::Document(tmp_doc));
                        collection.update_one(query, update, None).unwrap();
                    }
                }
            }

            // Create a new database (if doesn't exist) and add new collection
            // -------------------------------------------------------------------------------------
            // Get the database for the current collection of Model
            let db: Database = client.database(&meta.database_name);
            // If there is no collection for the current Model, create it
            if !database_names.contains(&meta.database_name)
                || !db
                    .list_collection_names(None)
                    .unwrap()
                    .contains(&meta.collection_name)
            {
                db.create_collection(&meta.collection_name, None).unwrap();
            }

            // Update the state of models for `models::Monitor`
            // -------------------------------------------------------------------------------------
            // Get the technical database `mango_orm_keyword` for the current model
            let db: Database = client.database(&mango_tech_keyword);
            // Check if there is a technical database of the project, if not, causes panic
            if !database_names.contains(&mango_tech_keyword)
                || !db
                    .list_collection_names(None)
                    .unwrap()
                    .contains(&"models".to_owned())
            {
                panic!("For migration not used `models::Monitor.refresh()`.");
            } else {
                let collection: Collection = db.collection("models");
                let filter: Document = mongodb::bson::doc! {"database": &meta.database_name, "collection": &meta.collection_name};
                let doc: Document = mongodb::bson::doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name,
                    "fields": trunc_list_fields_name.iter().map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                    "status": true
                };
                // Check if there is model state in the database
                if collection.count_documents(filter.clone(), None).unwrap() == 0_i64 {
                    // Add model state information
                    collection.insert_one(doc, None).unwrap();
                } else {
                    // Update model state information
                    let update: UpdateModifications = UpdateModifications::Document(doc);
                    collection.update_one(filter, update, None).unwrap();
                }
            }
        }

        // Run reorganize databases state
        self.napalm(&client_store);
    }
}
