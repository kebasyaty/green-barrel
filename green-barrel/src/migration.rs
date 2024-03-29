//! Migrations are Green Barrel’s way of
//! propagating changes you make to
//! your models (adding a field, deleting a collection, etc.) into
//! your database schema.

use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{
        de::from_document,
        doc,
        document::Document,
        ser::{to_bson, to_document},
        Bson,
    },
    Client, Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};

use crate::{
    models::helpers::{FileData, ImageData},
    store::METADATA,
};

// MIGRATION
// #################################################################################################
/// For creation and updating of a technical database.
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub field_type_map: HashMap<String, String>,
    pub status: bool,
}

/// For monitoring the state of models.
pub struct Monitor<'a> {
    pub app_name: &'a str,
    pub unique_app_key: &'a str,
    pub model_key_list: Vec<String>,
}

impl<'a> Monitor<'a> {
    /// Get the name of the technical database for a project.
    // *********************************************************************************************
    pub fn green_tech_name(&self) -> Result<String, Box<dyn Error>> {
        // app_name Validation.
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 20
        let re = Regex::new(r"^[a-zA-Z][_a-zA-Z\d]{1,20}$")?;
        if !re.is_match(self.app_name) {
            Err("app_name => \
                    Valid characters: _ a-z A-Z 0-9 and \
                    Max size: 20 ; \
                    First character: a-z A-Z")?
        }
        // UNIQUE_PROJECT_KEY Validation.
        // UNIQUE_PROJECT_KEY - It is recommended not to change.
        // Valid characters: a-z A-Z 0-9
        // Size: 16
        // Example: "7rzgacfqQB3B7q7T"
        let re = Regex::new(r"^[a-zA-Z\d]{16}$")?;
        if !re.is_match(self.unique_app_key) {
            Err("UNIQUE_PROJECT_KEY => \
                    Valid characters: a-z A-Z 0-9 and \
                    Size: 16.")?
        }
        //
        Ok(format!(
            "green_tech__{}__{}",
            self.app_name, self.unique_app_key
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
    async fn refresh(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        // Get the name of the technical database for a project.
        let db_green_tech: String = self.green_tech_name()?;
        // Collection for monitoring the state of Models.
        let collection_models_name: &str = "monitor_models";
        // Used to store selection items, for
        // Fields type like selectTextDyn, selectTextMultDyn, etc.
        let collection_dyn_fields_type: &str = "dynamic_fields";
        //Get a list of databases.
        let database_names = client.list_database_names(None, None).await?;
        // Create a technical database for the project if it doesn't exist.
        if !database_names.contains(&db_green_tech) {
            // Create a collection for models.
            client
                .database(&db_green_tech)
                .create_collection(collection_models_name, None)
                .await?;
            // Create a collection for fields types of `select`.
            // (selectTextDyn, selectTextMultDyn, etc.)
            client
                .database(&db_green_tech)
                .create_collection(collection_dyn_fields_type, None)
                .await?;
        } else {
            // Reset models state information.
            let green_tech_db = client.database(&db_green_tech);
            let collection_models = green_tech_db.collection::<Document>(collection_models_name);
            let mut cursor = collection_models.find(None, None).await?;
            while let Some(doc) = cursor.try_next().await? {
                let mut model_state = from_document::<ModelState>(doc)?;
                model_state.status = false;
                let query = doc! {
                    "database": &model_state.database,
                    "collection": &model_state.collection
                };
                let update = doc! {"$set": to_document(&model_state)?};
                collection_models.update_one(query, update, None).await?;
            }
        }
        //
        Ok(())
    }

    /// Reorganize databases state
    /// (full delete of orphaned collections and databases)
    // *********************************************************************************************
    async fn napalm(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        // Get the name of the technical database for a project.
        let db_green_tech: String = self.green_tech_name()?;
        let collection_models_name: &str = "monitor_models";
        let collection_dyn_fields_type: &str = "dynamic_fields";
        let green_tech_db: Database = client.database(&db_green_tech);
        let collection_models = green_tech_db.collection::<Document>(collection_models_name);
        let collection_dyn_fields =
            green_tech_db.collection::<Document>(collection_dyn_fields_type);
        // Delete orphaned Collections.
        let mut cursor = collection_models.find(None, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            let model_state: ModelState = from_document(doc)?;
            if !model_state.status {
                // Delete Collection (left without a model).
                client
                    .database(&model_state.database)
                    .collection::<Document>(&model_state.collection)
                    .drop(None)
                    .await?;
                // Delete a document with a record about the state of
                // the model from the technical base.
                let query: Document = doc! {
                    "database": &model_state.database,
                    "collection": &model_state.collection
                };
                collection_models.delete_one(query.clone(), None).await?;
                collection_dyn_fields.delete_one(query, None).await?;
            }
        }
        //
        Ok(())
    }

    /// Migrating Models
    // *********************************************************************************************
    /// Check the changes in the models and (if necessary) apply to the database.
    pub async fn migrat(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        // Run refresh models state.
        self.refresh(client).await?;
        for model_key in self.model_key_list.iter() {
            // Get metadata of Model
            let meta = {
                // Get metadata store.
                let metadata = METADATA.lock().await;
                // Get metadata of Model.
                if let Some(meta) = metadata.get(model_key) {
                    meta.clone()
                } else {
                    Err(format!(
                        "Model key: `{model_key}` ; Method: `migrat()` => \
                    Failed to get data from cache.",
                    ))?
                }
            };
            if !meta.is_add_doc {
                continue;
            }
            // Service_name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{1,30}$")
                .unwrap()
                .is_match(meta.service_name.as_str())
            {
                Err(format!(
                    "Model: `{}` > SERVICE_NAME => \
                        Valid characters: _ a-z A-Z 0-9 \
                        ; Max size: 30 \
                        ; First character: _ a-z A-Z",
                    meta.model_name
                ))?;
            }
            // Database name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{14,61}$")
                .unwrap()
                .is_match(meta.database_name.as_str())
            {
                Err(format!(
                    "Model: `{}` > DATABASE_NAME => \
                        Valid characters: _ a-z A-Z 0-9 \
                        ; Max size: 20 \
                        ; First character: _ a-z A-Z",
                    meta.model_name
                ))?;
            }
            //
            let fields_name = &meta.fields_name;
            let ignore_fields = &meta.ignore_fields;
            // List field names without `hash` and ignored fields.
            let trunc_fields_name_list = fields_name
                .iter()
                .filter(|item| *item != "hash" && !ignore_fields.contains(*item))
                .collect::<Vec<&String>>();
            // Get the name of the technical database for a project.
            let db_green_tech: String = self.green_tech_name()?;
            let database_names: Vec<String> = client.list_database_names(None, None).await?;
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
                .find_one(filter, None)
                .await?;
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
                    let collection = db.collection::<Document>(&meta.collection_name);
                    // Get cursor to all documents of the current Model.
                    let mut cursor = collection.find(None, None).await?;
                    // Iterate through all documents in a current (model) collection.
                    while let Some(doc_from_db) = cursor.try_next().await? {
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
                                         "ColorField" | "EmailField" | "PasswordField" | "PhoneField"| "TextField" 
                                        | "URLField" | "IPField"  |"ChoiceTextField" | "SlugField" => {
                                            if !default_value.is_null() {
                                                Bson::String(default_value.as_str().unwrap().to_string())
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "DateField" => {
                                           if !default_value.is_null() {
                                                let val = format!("{}T00:00",default_value.as_str().unwrap());
                                                if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str( &val, "%Y-%m-%dT%H:%M")
                                                {
                                                    let dt = chrono::DateTime::<Utc>::from_naive_utc_and_offset(ndt,Utc);
                                                    Bson::DateTime(dt.into())
                                                } else {
                                                    Err(format!("Service: `{}` > Model: `{}` ; \
                                                        Method: `migrat()` => \
                                                        Incorrect date format. \
                                                        Example: 1970-02-28",
                                                        meta.service_name, meta.model_name
                                                    ))?
                                                }
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "DateTimeField" | "HiddenDateTimeField" => {
                                            if !default_value.is_null() {
                                                let val = default_value.as_str().unwrap();
                                                if let Ok(ndt) = chrono::NaiveDateTime::parse_from_str( val, "%Y-%m-%dT%H:%M")
                                                {
                                                    let dt = chrono::DateTime::<Utc>::from_naive_utc_and_offset(ndt,Utc);
                                                    Bson::DateTime(dt.into())
                                                } else {
                                                    Err(format!("Service: `{}` > Model: `{}` ; \
                                                        Method: `migrat()` => \
                                                        Incorrect date and time format. \
                                                        Example: 1970-02-28T00:00",
                                                        meta.service_name, meta.model_name
                                                    ))?
                                                }
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "I32Field" | "ChoiceI32Field" => {
                                            if !default_value.is_null() {
                                                Bson::Int32(
                                                    i32::try_from(default_value.as_i64().unwrap())?
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "U32Field" | "ChoiceU32Field" | "I64Field" | "ChoiceI64Field" => {
                                            if !default_value.is_null() {
                                                Bson::Int64(
                                                    default_value.as_i64().unwrap()
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "F64Field" | "ChoiceF64Field" => {
                                            if !default_value.is_null() {
                                                Bson::Double(
                                                   default_value.as_f64().unwrap()
                                                )
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "BoolField" => {
                                            if !default_value.is_null() {
                                                Bson::Boolean(
                                                    default_value.as_bool().unwrap()
                                                )
                                            } else {
                                                Bson::Boolean(false)
                                            }
                                        }
                                        "FileField" => {
                                            if !default_value.is_null() {
                                                let mut file_data = serde_json::from_value::<FileData>(default_value.clone())?;
                                                // Define flags to check.
                                                if file_data.path.is_empty() || file_data.url.is_empty() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                        `migrat()` => Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field_name)
                                                    )?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = Path::new(path.as_str());
                                                if !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; \
                                                    Method: `migrat()` => File is missing - {}",
                                                        meta.model_name, field_name, path)
                                                    )?
                                                }
                                                // Get file metadata.
                                                let metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as f64;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Create doc.
                                                let result = to_document(&file_data)?;
                                                Bson::Document(result)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "ImageField" => {
                                            if !default_value.is_null() {
                                                let mut file_data = serde_json::from_value::<ImageData>(default_value.clone())?;
                                                // Define flags to check.
                                                if file_data.path.is_empty() || file_data.url.is_empty() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                        `migrat()` => Check the `path` and `url` \
                                                        attributes in the `default` field parameter.",
                                                        meta.model_name, field_name
                                                    ))?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = Path::new(path.as_str());
                                                if !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` ; Method: \
                                                            `migrat()` => Image is missing - {}",
                                                        meta.model_name, field_name, path
                                                    ))?
                                                }
                                                // Get file metadata.
                                                let metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as f64;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Get image width and height.
                                                let dimensions = image::image_dimensions(path)?;
                                                file_data.width = dimensions.0 as f64;
                                                file_data.height = dimensions.1 as f64;
                                                // Create doc.
                                                let result = to_document(&file_data)?;
                                                Bson::Document(result)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "ChoiceTextMultField" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<String>>(default_value.clone())?
                                                    .iter().map(|item| Bson::String(item.clone()))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "ChoiceI32MultField" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<i32>>(default_value.clone())?
                                                    .iter().map(|item| Bson::Int32(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                         "ChoiceU32MultField" | "ChoiceI64MultField"  => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<i64>>(default_value.clone())?
                                                    .iter().map(|item| mongodb::bson::Bson::Int64(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "ChoiceF64MultField" => {
                                            if !default_value.is_null() {
                                                let val = serde_json::from_value::<Vec<f64>>(default_value.clone())?
                                                    .iter().map(|item| Bson::Double(*item))
                                                    .collect::<Vec<Bson>>();
                                                Bson::Array(val)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "ChoiceTextDynField" | "ChoiceTextMultDynField" | "ChoiceI32DynField"
                                        | "ChoiceI32MultDynField" | "ChoiceU32DynField" | "ChoiceU32MultDynField"
                                        | "ChoiceI64DynField" | "ChoiceI64MultDynField" | "ChoiceF64DynField"
                                        | "ChoiceF64MultDynField" => {
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
                        collection
                            .update_one(query, doc! {"$set": tmp_doc}, None)
                            .await?;
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
                    .list_collection_names(None)
                    .await?
                    .contains(&meta.collection_name)
            {
                db.create_collection(&meta.collection_name, None).await?;
            }

            // Get the technical database `db_green_tech` for the current model.
            // -------------------------------------------------------------------------------------
            let db: Database = client.database(&db_green_tech);

            // Update the state of models for `models::Monitor`.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_green_tech)
                || !db
                    .list_collection_names(None)
                    .await?
                    .contains(&"monitor_models".to_owned())
            {
                Err("In the `refresh()` method, \
                        no technical database has been created for the project.")?
            } else {
                let collection = db.collection::<Document>("monitor_models");
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
                if collection.count_documents(filter.clone(), None).await? == 0 {
                    // Add model state information.
                    collection.insert_one(doc, None).await?;
                } else {
                    // Full update model state information.
                    collection
                        .update_one(filter, doc! {"$set": doc}, None)
                        .await?;
                }
            }

            // Document management to support model fields with dynamic fields type.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_green_tech)
                || !db
                    .list_collection_names(None)
                    .await?
                    .contains(&"dynamic_fields".to_owned())
            {
                Err("In the `refresh()` method, \
                        no technical database has been created for the project.")?
            }
            //
            let collection = db.collection::<Document>("dynamic_fields");
            let filter = doc! {
                "database": &meta.database_name,
                "collection": &meta.collection_name
            };
            // Check if there is a document in the database for
            // storing the values of dynamic fields type of model.
            if collection.count_documents(filter.clone(), None).await? == 0 {
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
                collection.insert_one(new_doc, None).await?;
            } else {
                // Get an existing document.
                let mut exist_doc = collection.find_one(filter.clone(), None).await?.unwrap();
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
                collection
                    .update_one(filter, doc! {"$set":exist_doc}, None)
                    .await?;
            }
        }
        // Run reorganize databases state.
        self.napalm(client).await?;
        //
        Ok(())
    }
}
