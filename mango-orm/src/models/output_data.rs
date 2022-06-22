//! Output data for QCommons.

/// Helper methods for converting output data (use in the commons.rs module).
pub trait Converters {
    /// Get prepared document.
    /// Hint: For page template.
    fn one_to_doc(
        doc: Option<mongodb::bson::document::Document>,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<Option<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        //
        if doc.is_some() {
            Ok(Some(Self::to_prepared_doc(
                doc.unwrap(),
                ignore_fields,
                map_widget_type,
                model_name,
            )?))
        } else {
            Ok(None)
        }
    }

    /// Get json-line.
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
            Ok(String::from("{}"))
        }
    }

    /// Get model instance.
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
            let bson_null = &mongodb::bson::Bson::Null;
            for (field_name, widget_type) in map_widget_type {
                if ignore_fields.contains(&field_name) {
                    continue;
                }
                let bson_val = doc.get(field_name.as_str()).unwrap();
                if widget_type == "inputFile" || widget_type == "inputImage" {
                    prepared_doc.insert(
                        field_name,
                        if bson_val != bson_null {
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

    /// Get prepared doc.
    /// Hint: Converting data types to model-friendly formats.
    // ---------------------------------------------------------------------------------------------
    fn to_prepared_doc(
        doc: mongodb::bson::document::Document,
        ignore_fields: &Vec<String>,
        map_widget_type: &std::collections::HashMap<String, String>,
        model_name: &str,
    ) -> Result<mongodb::bson::document::Document, Box<dyn std::error::Error>> {
        let bson_null = &mongodb::bson::Bson::Null;
        let mut prepared_doc = mongodb::bson::document::Document::new();
        for (field_name, widget_type) in map_widget_type {
            if ignore_fields.contains(&field_name) {
                continue;
            }
            if field_name == "hash" {
                let bson_val = doc.get("_id").unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
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
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(String::new())
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDate" {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..10].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDateTime" {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..16].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(field_name, bson_val);
            }
        }

        Ok(prepared_doc)
    }

    /// Get prepared documents.
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

    /// Get json-line.
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

        Ok(format!(
            "[{}]",
            if !json_line.is_empty() {
                &json_line[1..]
            } else {
                ""
            }
        ))
    }
}
