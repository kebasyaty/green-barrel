//! Helper methods for converting output data (use in the commons.rs module).

use mongodb::{
    bson::{document::Document, spec::ElementType, Bson},
    options::FindOptions,
    sync::Collection,
};

use serde_json::Value;
use std::{collections::HashMap, error::Error};

/// Helper methods for converting output data (use in the commons.rs module).
pub trait Converters {
    /// Get prepared document ( converting data types to model-friendly formats ).
    // ---------------------------------------------------------------------------------------------
    fn to_prepared_doc(
        doc: Document,
        ignore_fields: &[String],
        field_type_map: &HashMap<String, String>,
        model_name: &str,
    ) -> Result<Document, Box<dyn Error>> {
        //
        let mut accumula_doc = Document::new();
        for (field_name, field_type) in field_type_map {
            if ignore_fields.contains(field_name) {
                continue;
            }
            if field_name == "hash" {
                let val_bson = doc.get("_id").unwrap();
                accumula_doc.insert(
                    field_name,
                    if val_bson.element_type() != ElementType::Null {
                        Bson::String(val_bson.as_object_id().unwrap().to_hex())
                    } else {
                        Err(format!(
                            "Model: `{}` > Field: `hash` ; Method: `find_one()` => \
                                Missing document identifier `_id`.",
                            model_name
                        ))?
                    },
                );
            } else if field_type == "InputPassword" {
                let val_bson = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if val_bson.element_type() != ElementType::Null {
                        Bson::String(String::new())
                    } else {
                        Bson::Null
                    },
                );
            } else if field_type == "InputDate" {
                let val_bson = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if val_bson.element_type() != ElementType::Null {
                        Bson::String(val_bson.as_datetime().unwrap().to_rfc3339()[..10].into())
                    } else {
                        Bson::Null
                    },
                );
            } else if field_type.contains("DateTime") {
                let val_bson = doc.get(field_name).unwrap();
                accumula_doc.insert(
                    field_name,
                    if val_bson.element_type() != ElementType::Null {
                        Bson::String(val_bson.as_datetime().unwrap().to_rfc3339()[..19].into())
                    } else {
                        Bson::Null
                    },
                );
            } else {
                let val_bson = doc.get(field_name).unwrap();
                accumula_doc.insert(field_name, val_bson);
            }
        }

        Ok(accumula_doc)
    }

    /// In the model instance, in the format serde_json::Value,
    /// Update the field type values from the corresponding document from the database.
    // ---------------------------------------------------------------------------------------------
    fn one_to_json_val(
        db_doc: Document,
        ignore_fields: &[String],
        field_type_map: &HashMap<String, String>,
        model_name: &str,
        fields_name: &Vec<String>,
        model_json: &mut Value,
    ) -> Result<(), Box<dyn Error>> {
        //
        let doc_json = Bson::Document(Self::to_prepared_doc(
            db_doc,
            ignore_fields,
            field_type_map,
            model_name,
        )?)
        .into_relaxed_extjson();
        //
        for field_name in fields_name {
            if !ignore_fields.contains(field_name) {
                let val_json = doc_json.get(field_name).unwrap().clone();
                if let Some(val) = model_json.get_mut(field_name).unwrap().get_mut("value") {
                    *val = val_json;
                } else if let Some(val) = model_json.get_mut(field_name).unwrap().get_mut("checked")
                {
                    *val = val_json;
                }
            }
        }
        //
        Ok(())
    }

    /// Get prepared documents ( missing fields type ).
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

    /// Get json-line from document list ( missing fields type ).
    // ---------------------------------------------------------------------------------------------
    fn many_to_json(
        filter: Option<Document>,
        find_options: Option<FindOptions>,
        collection: Collection,
        ignore_fields: &[String],
        field_type_map: &HashMap<String, String>,
        model_name: &str,
    ) -> Result<String, Box<dyn Error>> {
        //
        let mut json_line = String::new();
        let mut cursor = collection.find(filter, find_options)?;
        while let Some(Ok(db_doc)) = cursor.next() {
            let prepared_doc =
                Self::to_prepared_doc(db_doc, ignore_fields, field_type_map, model_name)?;
            //
            json_line = format!(
                "{},{:?}",
                json_line,
                Bson::Document(prepared_doc).into_relaxed_extjson()
            );
        }

        if json_line.is_empty() {
            return Ok(json_line);
        }
        Ok(format!("[{}]", &json_line[1..]))
    }
}
