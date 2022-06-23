//! Output data for QCommons.

use crate::widgets::Widget;
use mongodb::bson::spec::ElementType;

/// Helper methods for converting output data (use in the commons.rs module).
pub trait Converters {
    /// Get json-line from document ( presence of widgets ).
    fn one_to_json(
        doc: Option<mongodb::bson::document::Document>,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        //
        if doc.is_some() {
            Ok(mongodb::bson::Bson::Document(Self::to_prepared_doc(
                doc.unwrap(),
                ignore_fields,
                map_widget_type,
                model_name,
            )?)
            .into_relaxed_extjson()
            .to_string())
        } else {
            Ok(String::new())
        }
    }

    /// Get widgets map from document ( presence of widgets ).
    fn one_doc_to_wig(
        doc: Option<mongodb::bson::document::Document>,
        ignore_fields: &Vec<String>,
        model_name: &str,
        fields_name: &Vec<String>,
        mut map_widgets: std::collections::HashMap<String, Widget>,
    ) -> Result<Option<std::collections::HashMap<String, Widget>>, Box<dyn std::error::Error>> {
        if doc.is_some() {
            let prepared_doc =
                Self::to_prepared_doc(doc.unwrap(), ignore_fields, map_widget_type, model_name)?;
            for field in fields_name {
                if !ignore_fields.contains(field) {
                    let mut widget = map_widgets.get_mut(field).unwrap();
                    let doc = prepared_doc.get(field).unwrap();
                    if doc.element_type() != ElementType::Null {
                        match doc.element_type() {
                            ElementType::String => {
                                widget.value = doc.as_str().unwrap().to_string();
                            }
                            ElementType::Int32 => {
                                widget.value = doc.as_i32().unwrap().to_string();
                            }
                            ElementType::Int64 => {
                                widget.value = doc.as_i64().unwrap().to_string();
                            }
                            ElementType::Double => {
                                widget.value = doc.as_f64().unwrap().to_string();
                            }
                            ElementType::Boolean => {
                                widget.checked = doc.as_bool().unwrap();
                            }
                            ElementType::Array => {
                                widget.value =
                                    serde_json::to_string(&doc.clone().into_relaxed_extjson())?;
                            }
                            _ => match widget.widget.as_str() {
                                "inputFile" | "inputImage" => {
                                    widget.value =
                                        serde_json::to_string(&doc.clone().into_relaxed_extjson())?;
                                }
                                _ => Err(format!(
                                    "Model: `{}` > Method: `one_doc_to_wig()` \
                                    -> Invalid Widget type.",
                                    model_name
                                ))?,
                            },
                        }
                    }
                }
            }
            Ok(Some(map_widgets))
        } else {
            Ok(None)
        }
    }

    /// Get model instance from document.
    /// Hint: For the `save`, `update`, `delete` operations.
    fn to_model_instance<T>(
        doc: Option<mongodb::bson::document::Document>,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<Option<T>, mongodb::bson::de::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        if doc.is_some() {
            let doc =
                Self::to_prepared_doc(doc.unwrap(), ignore_fields, map_widget_type, model_name)
                    .unwrap();
            let mut prepared_doc = mongodb::bson::document::Document::new();
            for (field_name, widget_type) in map_widget_type {
                if ignore_fields.contains(&field_name) {
                    continue;
                }
                let bson_val = doc.get(field_name).unwrap();
                if widget_type == "inputFile" || widget_type == "inputImage" {
                    prepared_doc.insert(
                        field_name,
                        if bson_val.element_type() != ElementType::Null {
                            let result =
                                serde_json::to_string(&bson_val.clone().into_relaxed_extjson())
                                    .unwrap();
                            mongodb::bson::Bson::String(result)
                        } else {
                            mongodb::bson::Bson::Null
                        },
                    );
                } else {
                    prepared_doc.insert(field_name, bson_val);
                }
            }
            Ok(Some(mongodb::bson::de::from_document::<T>(prepared_doc)?))
        } else {
            Ok(None)
        }
    }

    /// Get prepared document.
    /// Hint: Converting data types to model-friendly formats.
    // ---------------------------------------------------------------------------------------------
    fn to_prepared_doc(
        doc: mongodb::bson::document::Document,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<mongodb::bson::document::Document, Box<dyn std::error::Error>> {
        let mut prepared_doc = mongodb::bson::document::Document::new();
        for (field_name, widget_type) in map_widget_type {
            if ignore_fields.contains(&field_name) {
                continue;
            }
            if field_name == "hash" {
                let bson_val = doc.get("_id").unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        mongodb::bson::Bson::String(bson_val.as_object_id().unwrap().to_hex())
                    } else {
                        Err(format!(
                            "Model: `{}` > Field: `hash` > Method: `find_one()` -> \
                                Missing document identifier `_id`.",
                            model_name
                        ))?
                    },
                );
            } else if widget_type == "inputPassword" {
                let bson_val = doc.get(field_name).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        mongodb::bson::Bson::String(String::new())
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDate" {
                let bson_val = doc.get(field_name).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..10].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDateTime" {
                let bson_val = doc.get(field_name).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val.element_type() != ElementType::Null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..16].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else {
                let bson_val = doc.get(field_name).unwrap();
                prepared_doc.insert(field_name, bson_val);
            }
        }

        Ok(prepared_doc)
    }

    /// Get prepared documents ( missing widgets ).
    fn many_to_docs(
        filter: Option<mongodb::bson::document::Document>,
        find_options: Option<mongodb::options::FindOptions>,
        collection: mongodb::sync::Collection,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        //
        let mut cursor = collection.find(filter, find_options)?;
        let mut docs: Vec<mongodb::bson::document::Document> = Vec::new();
        while let Some(doc) = cursor.next() {
            let prepared_doc =
                Self::to_prepared_doc(doc?, ignore_fields, map_widget_type, model_name);
            docs.push(prepared_doc?);
        }

        Ok(docs)
    }

    /// Get json-line from document list ( missing widgets ).
    fn many_to_json(
        filter: Option<mongodb::bson::document::Document>,
        find_options: Option<mongodb::options::FindOptions>,
        collection: mongodb::sync::Collection,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        //
        let mut cursor = collection.find(filter, find_options)?;
        let mut json_line = String::new();
        while let Some(doc) = cursor.next() {
            let prepared_doc =
                Self::to_prepared_doc(doc?, ignore_fields, map_widget_type, model_name);
            //
            json_line = format!(
                "{},{}",
                json_line,
                mongodb::bson::Bson::Document(prepared_doc?)
                    .into_relaxed_extjson()
                    .to_string(),
            );
        }

        if !json_line.is_empty() {
            Ok(format!("[{}]", &json_line[1..]))
        } else {
            Ok(String::new())
        }
    }
}
