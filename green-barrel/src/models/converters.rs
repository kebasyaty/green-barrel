//! Helper methods for converting output data (use in the commons.rs module).

use mongodb::{
    bson::{de::from_document, document::Document, spec::ElementType, Bson},
    options::FindOptions,
    sync::Collection,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{collections::HashMap, error::Error};

/// Helper methods for converting output data (use in the commons.rs module).
pub trait Converters {
    /// Get model instance from document ( for the `save`, `update`, `delete` operations ).
    // ---------------------------------------------------------------------------------------------
    fn to_model_instance(
        doc: Option<Document>,
        ignore_fields: &Vec<String>,
        widget_type_map: &HashMap<String, String>,
        model_name: &str,
    ) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: DeserializeOwned + Sized,
    {
        if doc.is_none() {
            return Ok(None);
        }
        let prepared_doc =
            Self::to_prepared_doc(doc.unwrap(), ignore_fields, widget_type_map, model_name)
                .unwrap();
        let mut accumula_doc = Document::new();
        for (field_name, widget_type) in widget_type_map {
            if ignore_fields.contains(&field_name) {
                continue;
            }
            let bson_val = prepared_doc.get(field_name).unwrap();
            if widget_type == "InputFile" || widget_type == "InputImage" {
                accumula_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        let result =
                            serde_json::to_string(&bson_val.clone().into_relaxed_extjson())
                                .unwrap();
                        Bson::String(result)
                    } else {
                        Bson::Null
                    },
                );
            } else {
                accumula_doc.insert(field_name, bson_val);
            }
        }
        Ok(Some(from_document::<Self>(accumula_doc)?))
    }

    /// Get prepared document ( converting data types to model-friendly formats ).
    // ---------------------------------------------------------------------------------------------
    fn to_prepared_doc(
        doc: Document,
        ignore_fields: &Vec<String>,
        widget_type_map: &HashMap<String, String>,
        model_name: &str,
    ) -> Result<Document, Box<dyn Error>> {
        //
        let mut accumula_doc = Document::new();
        for (field_name, widget_type) in widget_type_map {
            if ignore_fields.contains(&field_name) {
                continue;
            }
            if field_name == "hash" {
                let bson_val = doc.get("_id").unwrap();
                accumula_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        Bson::String(bson_val.as_object_id().unwrap().to_hex())
                    } else {
                        Err(format!(
                            "Model: `{}` > Field: `hash` ; Method: `find_one()` => \
                            Missing document identifier `_id`.",
                            model_name
                        ))?
                    },
                );
            } else if widget_type == "InputPassword" {
                let bson_val = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        Bson::String(String::new())
                    } else {
                        Bson::Null
                    },
                );
            } else if widget_type == "InputDate" {
                let bson_val = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        Bson::String(bson_val.as_datetime().unwrap().to_rfc3339()[..10].into())
                    } else {
                        Bson::Null
                    },
                );
            } else if widget_type == "InputDateTime" {
                let bson_val = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        Bson::String(bson_val.as_datetime().unwrap().to_rfc3339()[..19].into())
                    } else {
                        Bson::Null
                    },
                );
            } else {
                let bson_val = doc.get(field_name).unwrap();
                accumula_doc.insert(field_name, bson_val);
            }
        }

        Ok(accumula_doc)
    }

    /// one_to_json_line
    // ---------------------------------------------------------------------------------------------
    fn one_to_json_val<'a>(
        db_doc: Document,
        ignore_fields: &Vec<String>,
        widget_type_map: &HashMap<String, String>,
        model_name: &str,
        fields_name: &Vec<String>,
        mut model_json: &Value,
    ) -> Result<&'a Value, Box<dyn Error>> {
        //
        let prepared_doc =
            Self::to_prepared_doc(db_doc, ignore_fields, widget_type_map, model_name)?;
        //
        for field_name in fields_name {
            if !ignore_fields.contains(field_name) {
                let field_doc = prepared_doc.get(field_name).unwrap();
                model_json
            }
        }
        //
        Ok(model_json)
    }

    /// Get prepared documents ( missing widgets ).
    // ---------------------------------------------------------------------------------------------
    fn many_to_doc_list(
        filter: Option<Document>,
        find_options: Option<FindOptions>,
        collection: Collection,
    ) -> Result<Vec<Document>, Box<dyn Error>> {
        //
        let mut doc_list: Vec<Document> = Vec::new();
        let mut cursor = collection.find(filter, find_options)?;
        while let Some(Ok(db_doc)) = cursor.next() {
            doc_list.push(db_doc);
        }

        Ok(doc_list)
    }

    /// Get json-line from document list ( missing widgets ).
    // ---------------------------------------------------------------------------------------------
    fn many_to_json_line(
        filter: Option<Document>,
        find_options: Option<FindOptions>,
        collection: Collection,
        ignore_fields: &Vec<String>,
        widget_type_map: &HashMap<String, String>,
        model_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        //
        let mut json_line = String::new();
        let mut cursor = collection.find(filter, find_options)?;
        while let Some(Ok(db_doc)) = cursor.next() {
            let prepared_doc =
                Self::to_prepared_doc(db_doc, ignore_fields, widget_type_map, model_name)?;
            //
            json_line = format!(
                "{},{}",
                json_line,
                Bson::Document(prepared_doc)
                    .into_relaxed_extjson()
                    .to_string(),
            );
        }

        if json_line.is_empty() {
            return Ok(json_line);
        }
        Ok(format!("[{}]", &json_line[1..]))
    }
}
