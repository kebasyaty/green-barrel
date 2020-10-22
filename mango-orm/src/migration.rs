//! # Migration
//!
//! `Monitor` - Creation and updating of a technical database for monitoring the state of models.

use crate::{
    models::Meta,
    store::{REGEX_IS_DATE, REGEX_IS_DATETIME},
    widgets::{FieldType, Widget},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use futures::stream::StreamExt;
use mongodb::bson::{doc, Bson};
use mongodb::{
    bson, bson::document::Document, options::UpdateModifications, Client, Collection, Cursor,
    Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// MIGRATION
// #################################################################################################
/// Creation and updating of a technical database for monitoring the state of models
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub status: bool,
}

// Database state monitoring
pub struct Monitor<'a> {
    pub keyword: &'a str,
    pub client: &'a Client,
    // Register models
    pub models: Vec<Meta<'a>>,
}

impl<'a> Monitor<'a> {
    // Refresh models state
    // *********************************************************************************************
    pub async fn refresh(&self) {
        // Keyword Validation.
        // KEYWORD - It is recommended not to change within the boundaries of one project.
        // ( Valid characters: _ a-z A-Z 0-9 ; Size: 8-16 )
        let re = Regex::new(r"^[_a-zA-Z\d]{8,16}$").unwrap();
        if !re.is_match(self.keyword) {
            panic!("Keyword - Valid characters: _ a-z A-Z 0-9 ; Size: 8-16.");
        }
        // Establish a connection with the technical database of the project
        let mango_orm_keyword: String = format!("mango_orm_{}", self.keyword);
        let collection_name: &str = "models";
        let database_names: Vec<String> =
            self.client.list_database_names(None, None).await.unwrap();
        // Create a technical database for the project if it doesn't exist
        if !database_names.contains(&mango_orm_keyword) {
            self.client
                .database(&mango_orm_keyword)
                .create_collection(collection_name, None)
                .await
                .unwrap();
        } else {
            // Reset model state information
            let mango_orm_db: Database = self.client.database(&mango_orm_keyword);
            let mango_orm_collection: Collection = mango_orm_db.collection(collection_name);
            let mut cursor: Cursor = mango_orm_collection.find(None, None).await.unwrap();

            while let Some(result) = cursor.next().await {
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
                            .await
                            .unwrap();
                    }
                    Err(err) => panic!("Migration `refresh()` -> {}", err),
                }
            }
        }
    }
    // Reorganize databases state
    // ( full delete of orphaned collections and databases )
    // *********************************************************************************************
    pub async fn napalm(&self) {
        // Establish a connection with the technical database of the project
        let mango_orm_keyword: String = format!("mango_orm_{}", self.keyword);
        let collection_name: &str = "models";
        let mango_orm_db: Database = self.client.database(&mango_orm_keyword);
        let mango_orm_collection: Collection = mango_orm_db.collection(collection_name);
        // Delete orphaned Collections
        let cursor: Cursor = mango_orm_collection.find(None, None).await.unwrap();
        let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;
        for result in results {
            match result {
                Ok(document) => {
                    let model_state: ModelState = bson::de::from_document(document).unwrap();
                    if !model_state.status {
                        // Delete Collection (left without a model)
                        self.client
                            .database(&model_state.database)
                            .collection(&model_state.collection)
                            .drop(None)
                            .await
                            .unwrap();
                        // Delete a document with a record about the state of
                        // the model from the technical base
                        let query: Document = bson::doc! {
                            "database": &model_state.database,
                            "collection": &model_state.collection
                        };
                        mango_orm_collection.delete_one(query, None).await.unwrap();
                    }
                }
                Err(err) => panic!("Migration `napalm()` -> {}", err),
            }
        }
    }

    // Migrating Models
    // *********************************************************************************************
    // 1.Checking widgets for correct attribute values and default values.
    // 2.Check model changes and (if required) apply to the database.
    pub async fn migrat(&self) {
        // Refresh models state
        self.refresh().await;
        //
        for meta in self.models {
            let model_name: &str = meta.model_name;
            let field_names: &'static [&str] = meta.field_names;
            let ignore_fields: Vec<&str> = meta.ignore_fields;
            // Validation of required fields in `Meta`
            if meta.service.is_empty() || meta.database.is_empty() {
                panic!(
                    "Service: `{}` -> Model: `{}` -> Method: `meta()` : \
                    The `service` and` database` fields must not be empty.",
                    meta.service, model_name
                )
            }
            // Checking for a required field `hash`
            if !field_names.contains(&"hash") {
                panic!(
                    "Service: `{}` -> Model: `{}` -> Field: `hash` : \
                    Add a `hash` field to the Model (`String` type).",
                    meta.service, model_name
                )
            }
            // Reserved field `created`
            if field_names.contains(&"created") {
                panic!(
                    "Service: `{}` -> Model: `{}` -> Field: `created` : \
                    This field is reserved. Solution - Replace with a different name",
                    meta.service, model_name
                )
            }
            // Reserved field `updated`
            if field_names.contains(&"updated") {
                panic!(
                    "Service: `{}` -> Model: `{}` -> Field: `updated` : \
                    This field is reserved. Solution - Replace with a different name",
                    meta.service, model_name
                )
            }
            // Check if ignored fields match model fields
            for field in ignore_fields.iter() {
                if !field_names.contains(field) {
                    panic!(
                        "Service: `{}` -> Model: `{}` : \
                        The model structure is missing an ignored `{}` field.",
                        meta.service, model_name, field
                    )
                }
            }
            // List field names without `hash` and other auxiliary fields
            let field_names_without_auxiliary: Vec<&str> = field_names
                .iter()
                .map(|field| field.clone())
                .filter(|field| field != &"hash" && !ignore_fields.contains(field))
                .collect();
            // Checking for the presence of fields
            if field_names_without_auxiliary.is_empty() {
                panic!(
                    "Service: `{}` -> Model: `{}` -> Method: `migrat()` : \
                        The model structure has no fields.",
                    meta.service, model_name
                );
            }
            // Create a map with field types
            let map_field_types: HashMap<&str, &str> = meta.field_types;
            // Technical database for `models::Monitor`
            let mango_orm_keyword = format!("mango_orm_{}", self.keyword);
            // Checking the status of Widgets
            let map_widgets: HashMap<&str, Widget> = Self::widgets_full_map().unwrap();
            // List of existing databases
            let database_names: Vec<String> =
                self.client.list_database_names(None, None).await.unwrap();
            // Map of default values and value types from `value` attribute -
            // (String, String) -> index `0` - `enum type` , index `1` - `value`
            let mut default_values: HashMap<&str, (&str, String)> = HashMap::new();

            // Checking Widgets
            // -------------------------------------------------------------------------------------
            // Looping over fields and attributes
            for (field, widget) in map_widgets {
                // Checking for the correct field name
                if !field_names.contains(&field) {
                    panic!(
                        "Service: `{}` -> Model: `{}` -> widgets() : \
                        `{}` - Incorrect field name.",
                        meta.service, model_name, field
                    )
                }
                // Add in map default value
                default_values.insert(
                    field,
                    (widget.value.get_enum_type(), widget.value.get_raw_data()),
                );
                // Checking attribute states
                match widget.value {
                    // Hash
                    // -----------------------------------------------------------------------------
                    FieldType::Hash => {
                        let enum_field_type = "Hash".to_string();
                        let data_field_type = "String".to_string();
                        if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }

                    // InputCheckBoxText
                    // InputCheckBoxI32
                    // InputCheckBoxU32
                    // InputCheckBoxI64
                    // InputCheckBoxF64
                    // -----------------------------------------------------------------------------
                    FieldType::InputCheckBoxText(_)
                    | FieldType::InputCheckBoxI32(_)
                    | FieldType::InputCheckBoxU32(_)
                    | FieldType::InputCheckBoxI64(_)
                    | FieldType::InputCheckBoxF64(_) => {
                        let mut enum_field_type = String::new();
                        let mut data_field_type = String::new();
                        match widget.value {
                            FieldType::InputCheckBoxText(_) => {
                                enum_field_type = "InputCheckBoxText".to_string();
                                data_field_type = "String".to_string();
                            }
                            FieldType::InputCheckBoxI32(_) => {
                                enum_field_type = "InputCheckBoxI32".to_string();
                                data_field_type = "i32".to_string();
                            }
                            FieldType::InputCheckBoxU32(_) => {
                                enum_field_type = "InputCheckBoxU32".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::InputCheckBoxI64(_) => {
                                enum_field_type = "InputCheckBoxI64".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::InputCheckBoxF64(_) => {
                                enum_field_type = "InputCheckBoxF64".to_string();
                                data_field_type = "f64".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.maxlength != 0 {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `maxlength` = only 0 (zero).",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                fields must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if widget.other_attrs.contains("checked") {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `other_attrs` - must not contain the word `checked`.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if data_field_type != map_field_types[field] {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `{}`.",
                                meta.service, model_name, field, map_field_types[field]
                            )
                        }
                    }

                    // InputColor
                    // InputEmail
                    // InputPassword
                    // InputText
                    // InputUrl
                    // InputIP
                    // InputIPv4
                    // InputIPv6
                    // TextArea
                    // -----------------------------------------------------------------------------
                    FieldType::InputColor(_)
                    | FieldType::InputEmail(_)
                    | FieldType::InputPassword(_)
                    | FieldType::InputText(_)
                    | FieldType::InputUrl(_)
                    | FieldType::InputIP(_)
                    | FieldType::InputIPv4(_)
                    | FieldType::InputIPv6(_)
                    | FieldType::TextArea(_) => {
                        let mut enum_field_type = String::new();
                        match widget.value {
                            FieldType::InputColor(_) => {
                                enum_field_type = "InputColor".to_string();
                            }
                            FieldType::InputEmail(_) => {
                                enum_field_type = "InputEmail".to_string();
                            }
                            FieldType::InputPassword(_) => {
                                enum_field_type = "InputPassword".to_string();
                            }
                            FieldType::InputText(_) => {
                                enum_field_type = "InputText".to_string();
                            }
                            FieldType::InputUrl(_) => {
                                enum_field_type = "InputUrl".to_string();
                            }
                            FieldType::TextArea(_) => {
                                enum_field_type = "TextArea".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != "U32"
                            || widget.min.get_enum_type() != "U32"
                            || widget.max.get_enum_type() != "U32"
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The attributes `min` and `max` \
                                must be of type `DataType::U32`.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }

                    // InputDate
                    // InputDateTime
                    // -----------------------------------------------------------------------------
                    FieldType::InputDate(_) | FieldType::InputDateTime(_) => {
                        let mut enum_field_type = String::new();
                        match widget.value {
                            FieldType::InputDate(_) => {
                                enum_field_type = "InputDate".to_string();
                            }
                            FieldType::InputDateTime(_) => {
                                enum_field_type = "InputDateTime".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` ->\
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != "U32" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The attribute `step` must be of type \
                                `DataType::U32`.",
                                meta.service, model_name, field
                            )
                        } else if widget.min.get_enum_type() != "Text"
                            || widget.max.get_enum_type() != "Text"
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The attributes `min` and `max` \
                                must be of type `DataType::Text`.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        } else if widget.min.get_raw_data().len() != widget.max.get_raw_data().len()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                    Method: `widgets()` : The `min` and` max` \
                                    attributes must be in \
                                    the appropriate format `1970-02-28` or \
                                    ` 1970-02-28T00:00` or an empty strings.",
                                meta.service, model_name, field
                            )
                        } else if !widget.min.get_raw_data().is_empty()
                            && !widget.max.get_raw_data().is_empty()
                        {
                            let mut date_min: String = widget.min.get_raw_data();
                            let mut date_max: String = widget.max.get_raw_data();
                            let mut date_value: String = widget.value.get_raw_data();
                            match widget.value {
                                FieldType::InputDate(_) => {
                                    // Example: "1970-02-28"
                                    if !REGEX_IS_DATE.is_match(&date_min) {
                                        panic!(
                                            "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                                Method: `widgets()` -> Attribute: `min` : \
                                                Incorrect date format. Example: 1970-02-28",
                                            meta.service, model_name, field
                                        )
                                    }
                                    if !REGEX_IS_DATE.is_match(&date_max) {
                                        panic!(
                                            "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                                Method: `widgets()` -> Attribute: `max` : \
                                                Incorrect date format. Example: 1970-02-28",
                                            meta.service, model_name, field
                                        )
                                    }
                                    if !date_value.is_empty() {
                                        if !REGEX_IS_DATE.is_match(&date_value) {
                                            panic!(
                                                "Service: `{}` -> Model: `{}` -> \
                                                    Field: `{}` -> Method: `widgets()` -> \
                                                    Attribute: `value` : Incorrect date \
                                                    format. Example: 1970-02-28",
                                                meta.service, model_name, field
                                            )
                                        }
                                        date_value = format!("{}T00:00", date_value);
                                    }
                                    date_min = format!("{}T00:00", date_min);
                                    date_max = format!("{}T00:00", date_max);
                                }
                                FieldType::InputDateTime(_) => {
                                    // Example: "1970-02-28T00:00"
                                    if !REGEX_IS_DATETIME.is_match(&date_min) {
                                        panic!(
                                            "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                                Method: `widgets()` -> Attribute: `min` : \
                                                Incorrect date format. \
                                                Example: 1970-02-28T00:00",
                                            meta.service, model_name, field
                                        )
                                    }
                                    if !REGEX_IS_DATETIME.is_match(&date_max) {
                                        panic!(
                                            "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                                Method: `widgets()` -> Attribute: `max` : \
                                                Incorrect date format. \
                                                Example: 1970-02-28T00:00",
                                            meta.service, model_name, field
                                        )
                                    }
                                    if !date_value.is_empty() {
                                        if !REGEX_IS_DATETIME.is_match(&date_value) {
                                            panic!(
                                                "Service: `{}` -> Model: `{}` -> \
                                                    Field: `{}` -> Method: `widgets()` -> \
                                                    Attribute: `value` : Incorrect date \
                                                    format. Example: 1970-02-28T00:00",
                                                meta.service, model_name, field
                                            )
                                        }
                                    }
                                }
                                _ => panic!("Invalid field type"),
                            }
                            // Get DateTime
                            let dt_min: DateTime<Utc> = DateTime::<Utc>::from_utc(
                                NaiveDateTime::parse_from_str(&date_min, "%Y-%m-%dT%H:%M").unwrap(),
                                Utc,
                            );
                            let dt_max: DateTime<Utc> = DateTime::<Utc>::from_utc(
                                NaiveDateTime::parse_from_str(&date_max, "%Y-%m-%dT%H:%M").unwrap(),
                                Utc,
                            );
                            // If the `max` attribute is not greater than `min`, call a panic
                            if dt_min >= dt_max {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                    Method: `widgets()` -> Attribute: `min` : \
                                    Must be less than `max`.",
                                    meta.service, model_name, field
                                )
                            } else if !date_value.is_empty() {
                                // Check that the default is in the dates range
                                // from `min` to `max`.
                                let dt_value: DateTime<Utc> = DateTime::<Utc>::from_utc(
                                    NaiveDateTime::parse_from_str(&date_value, "%Y-%m-%dT%H:%M")
                                        .unwrap(),
                                    Utc,
                                );
                                if dt_value < dt_min || dt_value > dt_max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                            Method: `widgets()` -> Attribute: `value` : \
                                            Out of range between `min` and` max`.",
                                        meta.service, model_name, field
                                    )
                                }
                            }
                        }
                    }

                    // InputFile
                    // InputImage
                    // -----------------------------------------------------------------------------
                    FieldType::InputFile | FieldType::InputImage => {
                        let mut enum_field_type = String::new();
                        match widget.value {
                            FieldType::InputFile => {
                                enum_field_type = "InputFile".to_string();
                            }
                            FieldType::InputImage => {
                                enum_field_type = "InputImage".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }

                    // InputNumberI32
                    // InputNumberU32
                    // InputNumberI64
                    // InputNumberF64
                    // -----------------------------------------------------------------------------
                    FieldType::InputNumberI32(_)
                    | FieldType::InputNumberU32(_)
                    | FieldType::InputNumberI64(_)
                    | FieldType::InputNumberF64(_) => {
                        let mut enum_field_type = String::new();
                        let mut data_field_type = String::new();
                        let mut step_min_max_enum_type = String::new();
                        match widget.value {
                            FieldType::InputNumberI32(_) => {
                                enum_field_type = "InputNumberI32".to_string();
                                data_field_type = "i32".to_string();
                                step_min_max_enum_type = "I32".to_string();
                            }
                            FieldType::InputNumberU32(_) => {
                                enum_field_type = "InputNumberU32".to_string();
                                data_field_type = "i64".to_string();
                                step_min_max_enum_type = "U32".to_string();
                            }
                            FieldType::InputNumberI64(_) => {
                                enum_field_type = "InputNumberI64".to_string();
                                data_field_type = "i64".to_string();
                                step_min_max_enum_type = "I64".to_string();
                            }
                            FieldType::InputNumberF64(_) => {
                                enum_field_type = "InputNumberF64".to_string();
                                data_field_type = "f64".to_string();
                                step_min_max_enum_type = "F64".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if data_field_type != map_field_types[field] {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `{}`.",
                                meta.service, model_name, field, map_field_types[field]
                            )
                        } else if widget.step.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `step` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        } else if widget.min.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `min` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        } else if widget.max.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `max` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        }
                    }

                    // InputRadioText
                    // InputRadioI32
                    // InputRadioU32
                    // InputRadioI64
                    // InputRadioF64
                    // -----------------------------------------------------------------------------
                    FieldType::InputRadioText(_)
                    | FieldType::InputRadioI32(_)
                    | FieldType::InputRadioU32(_)
                    | FieldType::InputRadioI64(_)
                    | FieldType::InputRadioF64(_) => {
                        let mut enum_field_type = String::new();
                        let mut data_field_type = String::new();
                        match widget.value {
                            FieldType::InputRadioText(_) => {
                                enum_field_type = "InputRadioText".to_string();
                                data_field_type = "String".to_string();
                            }
                            FieldType::InputRadioI32(_) => {
                                enum_field_type = "InputRadioI32".to_string();
                                data_field_type = "i32".to_string();
                            }
                            FieldType::InputRadioU32(_) => {
                                enum_field_type = "InputRadioU32".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::InputRadioI64(_) => {
                                enum_field_type = "InputRadioI64".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::InputRadioF64(_) => {
                                enum_field_type = "InputRadioF64".to_string();
                                data_field_type = "f64".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.maxlength != 0 {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `maxlength` = only 0 (zero).",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if widget.other_attrs.contains("checked") {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `other_attrs` - must not contain the word `checked`.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` - must not be an empty vec![]",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if data_field_type != map_field_types[field] {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `{}`.",
                                meta.service, model_name, field, map_field_types[field]
                            )
                        }
                    }

                    // InputRangeI32
                    // InputRangeU32
                    // InputRangeI64
                    // InputRangeF64
                    // -----------------------------------------------------------------------------
                    FieldType::InputRangeI32(_)
                    | FieldType::InputRangeU32(_)
                    | FieldType::InputRangeI64(_)
                    | FieldType::InputRangeF64(_) => {
                        let mut enum_field_type = String::new();
                        let mut data_field_type = String::new();
                        let mut step_min_max_enum_type = String::new();
                        match widget.value {
                            FieldType::InputRangeI32(_) => {
                                enum_field_type = "InputRangeI32".to_string();
                                data_field_type = "i32".to_string();
                                step_min_max_enum_type = "I32".to_string();
                            }
                            FieldType::InputRangeU32(_) => {
                                enum_field_type = "InputRangeU32".to_string();
                                data_field_type = "i64".to_string();
                                step_min_max_enum_type = "U32".to_string();
                            }
                            FieldType::InputRangeI64(_) => {
                                enum_field_type = "InputRangeI64".to_string();
                                data_field_type = "i64".to_string();
                                step_min_max_enum_type = "I64".to_string();
                            }
                            FieldType::InputRangeF64(_) => {
                                enum_field_type = "InputRangeI64".to_string();
                                data_field_type = "f64".to_string();
                                step_min_max_enum_type = "F64".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` = only blank vec![].",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if data_field_type != map_field_types[field] {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `{}`.",
                                meta.service, model_name, field, map_field_types[field]
                            )
                        } else if widget.step.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `step` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        } else if widget.min.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `min` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        } else if widget.max.get_data_type() != data_field_type {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `max` = `{}`.",
                                meta.service,
                                model_name,
                                field,
                                enum_field_type,
                                step_min_max_enum_type
                            )
                        }
                    }

                    // SelectText
                    // SelectI32
                    // SelectU32
                    // SelectI64
                    // SelectF64
                    // -----------------------------------------------------------------------------
                    FieldType::SelectText(_)
                    | FieldType::SelectI32(_)
                    | FieldType::SelectU32(_)
                    | FieldType::SelectI64(_)
                    | FieldType::SelectF64(_) => {
                        let mut enum_field_type = String::new();
                        let mut data_field_type = String::new();
                        match widget.value {
                            FieldType::SelectText(_) => {
                                enum_field_type = "SelectText".to_string();
                                data_field_type = "String".to_string();
                            }
                            FieldType::SelectI32(_) => {
                                enum_field_type = "SelectI32".to_string();
                                data_field_type = "i32".to_string();
                            }
                            FieldType::SelectU32(_) => {
                                enum_field_type = "SelectU32".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::SelectI64(_) => {
                                enum_field_type = "SelectI64".to_string();
                                data_field_type = "i64".to_string();
                            }
                            FieldType::SelectF64(_) => {
                                enum_field_type = "SelectF64".to_string();
                                data_field_type = "f64".to_string();
                            }
                            _ => panic!("Invalid field type"),
                        }
                        if widget.relation_model != String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `relation_model` = only blank string.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType::`{}` : \
                                `select` - Should not be empty.",
                                meta.service, model_name, field, enum_field_type
                            )
                        } else if data_field_type != map_field_types[field] {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `{}`.",
                                meta.service, model_name, field, map_field_types[field]
                            )
                        }
                    }

                    // ForeignKey
                    // -----------------------------------------------------------------------------
                    FieldType::ForeignKey => {
                        if widget.relation_model == String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = FieldType `ForeignKey` : \
                                `relation_model` = \
                                <CategoryName>::meta().collection.to_string().",
                                meta.service, model_name, field
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = \
                                FieldType `ForeignKey` : `select` = only blank vec![].",
                                meta.service, model_name, field
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }

                    // ManyToMany
                    // -----------------------------------------------------------------------------
                    FieldType::ManyToMany => {
                        if widget.relation_model == String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = \
                                FieldType `ManyToMany` : `relation_model` = \
                                <CategoryName>::meta().collection.to_string().",
                                meta.service, model_name, field
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = \
                                FieldType `ManyToMany` : `select` = only blank vec![].",
                                meta.service, model_name, field
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }

                    // OneToOne
                    // -----------------------------------------------------------------------------
                    FieldType::OneToOne => {
                        if widget.relation_model == String::new() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = \
                                FieldType `OneToOne` : `relation_model` = \
                                <CategoryName>::meta().collection.to_string().",
                                meta.service, model_name, field
                            )
                        } else if widget.step.get_enum_type() != widget.min.get_enum_type()
                            || widget.step.get_enum_type() != widget.max.get_enum_type()
                        {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` : The `step`, `min` and `max` \
                                attributes must have the same types.",
                                meta.service, model_name, field
                            )
                        } else if !widget.select.is_empty() {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                Method: `widgets()` -> For `value` = \
                                FieldType `OneToOne` : `select` = only blank vec![].",
                                meta.service, model_name, field
                            )
                        } else if map_field_types[field] != "String" {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Field type is not equal to `String`.",
                                meta.service, model_name, field
                            )
                        }
                    }
                    _ => panic!(
                        "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                `field_type` - Non-existent field type.",
                        meta.service, model_name, field
                    ),
                }

                // Checking the values of the `step`, `min` and `max` attributes
                // ---------------------------------------------------------------------------------
                if widget.step.get_enum_type() != "Text"
                    && widget.min.get_enum_type() != "Text"
                    && widget.max.get_enum_type() != "Text"
                {
                    match widget.step.get_enum_type() {
                        "I32" => {
                            let step: i32 = widget.step.get_raw_data().parse().unwrap();
                            let min: i32 = widget.min.get_raw_data().parse().unwrap();
                            let max: i32 = widget.max.get_raw_data().parse().unwrap();
                            if step > 0_i32 || min > 0_i32 || max > 0_i32 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The `min` attribute must not be greater \
                                        than `max`.",
                                        meta.service, model_name, field
                                    )
                                } else if step > 0_i32 && (max - min) % step != 0_i32 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The value of the `step` attribute does not \
                                        match the condition (max - min) % step == 0.",
                                        meta.service, model_name, field
                                    )
                                }
                            }
                        }
                        "U32" | "I64" => {
                            let step: i64 = widget.step.get_raw_data().parse().unwrap();
                            let min: i64 = widget.min.get_raw_data().parse().unwrap();
                            let max: i64 = widget.max.get_raw_data().parse().unwrap();
                            if step > 0_i64 || min > 0_i64 || max > 0_i64 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The `min` attribute must not be greater \
                                        than `max`.",
                                        meta.service, model_name, field
                                    )
                                } else if step > 0_i64 && (max - min) % step != 0_i64 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The value of the `step` attribute does not \
                                        match the condition (max - min) % step == 0.",
                                        meta.service, model_name, field
                                    )
                                }
                            }
                        }
                        "F64" => {
                            let step: f64 = widget.step.get_raw_data().parse().unwrap();
                            let min: f64 = widget.min.get_raw_data().parse().unwrap();
                            let max: f64 = widget.max.get_raw_data().parse().unwrap();
                            if step > 0_f64 || min > 0_f64 || max > 0_f64 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The `min` attribute must not be greater \
                                        than `max`.",
                                        meta.service, model_name, field
                                    )
                                } else if step > 0_f64 && (max - min) % step != 0_f64 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                        widgets : The value of the `step` attribute does not \
                                        match the condition (max - min) % step == 0.",
                                        meta.service, model_name, field
                                    )
                                }
                            }
                        }
                        _ => panic!(
                            "Service: `{}` -> Model: `{}` -> Field: `{}` : \
                                Non-existent field type.",
                            meta.service, model_name, field
                        ),
                    }
                }
            }

            // Check the field changes in the Model and (if required)
            // update documents in the current Collection
            // -------------------------------------------------------------------------------------
            // Get a list of current model field names from the technical database
            // `mango_orm_keyword`
            let filter: Document = doc! {
                "database": &meta.database,
                "collection": &meta.collection
            };
            let model: Option<Document> = self
                .client
                .database(&mango_orm_keyword)
                .collection("models")
                .find_one(filter, None)
                .await
                .unwrap();
            if model.is_some() {
                // Get a list of fields from the technical database
                let mango_orm_fnames: Vec<String> = {
                    let model: Document = model.unwrap();
                    let fields: Vec<Bson> = model.get_array("fields").unwrap().to_vec();
                    fields
                        .into_iter()
                        .map(|item: Bson| item.as_str().unwrap().to_string())
                        .collect()
                };
                // Check if the set of fields in the collection of
                // the current Model needs to be updated
                let mut run_documents_modification: bool = false;
                if field_names_without_auxiliary.len() != mango_orm_fnames.len() {
                    run_documents_modification = true;
                } else {
                    for item in field_names_without_auxiliary {
                        if mango_orm_fnames.iter().any(|item2| item2 != &item) {
                            run_documents_modification = true;
                            break;
                        }
                    }
                }
                // Start (if necessary) updating the set of fields in the current collection
                if run_documents_modification {
                    // Get the database and collection of the current Model
                    let db: Database = self.client.database(&meta.database);
                    let collection: Collection = db.collection(&meta.collection);
                    // Get cursor to all documents of the current Model
                    let mut cursor: Cursor = collection.find(None, None).await.unwrap();
                    // Iterate through all documents in a current (model) collection
                    while let Some(result) = cursor.next().await {
                        let doc_from_db: Document = result.unwrap();
                        // Create temporary blank document
                        let mut tmp_doc = doc! {};
                        // Loop over all fields of the model
                        for field in field_names {
                            if field == &"hash" || ignore_fields.contains(field) {
                                continue;
                            }
                            // If the field exists, get its value
                            if doc_from_db.contains_key(field) {
                                let value_from_db: Option<&Bson> = doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> \
                                            Method: `migrat()` : \
                                            Can't get field value from database.",
                                        meta.service, model_name, field
                                    );
                                }
                            } else {
                                // If no field exists, get default value
                                let value = &default_values[field];
                                tmp_doc.insert(
                                    field.to_string(),
                                    match value.0 {
                                        "InputCheckBoxText" | "InputRadioText" | "InputColor"
                                        | "InputEmail" | "InputPassword" | "InputTel"
                                        | "InputText" | "InputUrl" | "InputIP" | "InputIPv4"
                                        | "InputIPv6" | "TextArea" | "SelectText" => {
                                            Bson::String(value.1.clone())
                                        }
                                        "InputDate" => {
                                            // Example: "1970-02-28"
                                            let mut val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !REGEX_IS_DATE.is_match(&val) {
                                                    panic!(
                                                        "Service: `{}` -> Model: `{}` -> \
                                                        Method: `widgets()` : Incorrect date \
                                                        format. Example: 1970-02-28",
                                                        meta.service, model_name
                                                    )
                                                }
                                                let val = format!("{}T00:00", val);
                                                let dt: DateTime<Utc> = DateTime::<Utc>::from_utc(
                                                    NaiveDateTime::parse_from_str(
                                                        &val,
                                                        "%Y-%m-%dT%H:%M",
                                                    )
                                                    .unwrap(),
                                                    Utc,
                                                );
                                                Bson::DateTime(dt)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "InputDateTime" => {
                                            // Example: "1970-02-28T00:00"
                                            let mut val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !REGEX_IS_DATETIME.is_match(&val) {
                                                    panic!(
                                                        "Service: `{}` -> Model: `{}` -> \
                                                        Method: `widgets()` : \
                                                        Incorrect date and time format. \
                                                        Example: 1970-02-28T00:00",
                                                        meta.service, model_name
                                                    )
                                                }
                                                let dt: DateTime<Utc> = DateTime::<Utc>::from_utc(
                                                    NaiveDateTime::parse_from_str(
                                                        &val,
                                                        "%Y-%m-%dT%H:%M",
                                                    )
                                                    .unwrap(),
                                                    Utc,
                                                );
                                                Bson::DateTime(dt)
                                            } else {
                                                Bson::Null
                                            }
                                        }
                                        "InputCheckBoxI32" | "InputRadioI32" | "InputNumberI32"
                                        | "InputRangeI32" | "SelectI32" => {
                                            Bson::Int32(value.1.parse::<i32>().unwrap())
                                        }
                                        "InputCheckBoxU32" | "InputRadioU32" | "InputNumberU32"
                                        | "InputRangeU32" | "SelectU32" | "InputCheckBoxI64"
                                        | "InputRadioI64" | "InputNumberI64" | "InputRangeI64"
                                        | "SelectI64" => {
                                            Bson::Int64(value.1.parse::<i64>().unwrap())
                                        }
                                        "InputCheckBoxF64" | "InputRadioF64" | "InputNumberF64"
                                        | "InputRangeF64" | "SelectF64" => {
                                            Bson::Double(value.1.parse::<f64>().unwrap())
                                        }
                                        "InputCheckBoxBool" => {
                                            Bson::Boolean(value.1.parse::<bool>().unwrap())
                                        }
                                        _ => panic!(
                                            "Service: `{}` -> Model: `{}` -> Method: \
                                                `migrat()` : Invalid enum type.",
                                            meta.service, model_name
                                        ),
                                    },
                                );
                            }
                        }
                        // Insert fields for timestamps `created` and `updated`
                        for field in vec!["created", "updated"] {
                            if doc_from_db.contains_key(field) {
                                let value_from_db: Option<&Bson> = doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> \
                                            Method: `migrat()` : \
                                            Cannot get field value from database for \
                                            field `{}`.",
                                        meta.service, model_name, field
                                    );
                                }
                            } else {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Method: `migrat()` : \
                                        Key `{}` was not found in the document from \
                                        the database.",
                                    meta.service, model_name, field
                                );
                            }
                        }
                        // Save updated document
                        let query = doc! {"_id": doc_from_db.get_object_id("_id").unwrap()};
                        let update = UpdateModifications::Document(tmp_doc);
                        collection.update_one(query, update, None).await.unwrap();
                    }
                }
            }

            // Create a new database (if doesn't exist) and add new collection
            // -------------------------------------------------------------------------------------
            // Get the database for the current collection of Model
            let db: Database = self.client.database(&meta.database);
            // If there is no collection for the current Model, create it
            if !database_names.contains(&meta.database)
                || !db
                    .list_collection_names(None)
                    .await
                    .unwrap()
                    .contains(&meta.collection)
            {
                db.create_collection(&meta.collection, None).await.unwrap();
            }

            // Update the state of models for `models::Monitor`
            // -------------------------------------------------------------------------------------
            // Get the technical database `mango_orm_keyword` for the current model
            let db: Database = self.client.database(&mango_orm_keyword);
            // Check if there is a technical database of the project, if not, causes panic
            if !database_names.contains(&mango_orm_keyword)
                || !db
                    .list_collection_names(None)
                    .await
                    .unwrap()
                    .contains(&"models".to_owned())
            {
                panic!("For migration not used `models::Monitor.refresh()`.");
            } else {
                let collection = db.collection("models");
                let filter = doc! {"database": &meta.database, "collection": &meta.collection};
                let doc = doc! {
                    "database": &meta.database,
                    "collection": &meta.collection,
                    "fields": field_names.iter().map(|field| field.to_string())
                        .filter(|field| field != "hash"
                            && !ignore_fields.contains(&field.as_str()))
                        .collect::<Vec<String>>(),
                    "status": true
                };
                // Check if there is model state in the database
                if collection
                    .count_documents(filter.clone(), None)
                    .await
                    .unwrap()
                    == 0_i64
                {
                    // Add model state information
                    collection.insert_one(doc, None).await.unwrap();
                } else {
                    // Update model state information
                    let update = UpdateModifications::Document(doc);
                    collection.update_one(filter, update, None).await.unwrap();
                }
            }
        }
        // Reorganize databases state
        // (full delete of orphaned collections and databases)
        self.napalm().await;
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
