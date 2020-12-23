//! # Output data types for database queries
//!
//! `OutputDataOne` - To return results after processing queries for one document.
//! `OutputDataMany` - To return results after processing queries for many documents.
//!

// To return results after processing queries for one document.
// *************************************************************************************************
#[derive(Debug, Clone)]
pub enum OutputDataOne {
    Doc(
        (
            mongodb::bson::document::Document,
            Vec<String>,
            std::collections::HashMap<String, String>,
        ),
    ),
}

impl Default for OutputDataOne {
    fn default() -> Self {
        OutputDataOne::Doc((
            mongodb::bson::document::Document::new(),
            Vec::new(),
            std::collections::HashMap::new(),
        ))
    }
}

impl OutputDataOne {
    // Get document.
    // (For page templates)
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::Doc(data) => data.0.clone(),
        }
    }

    // Get json-line.
    // (For Ajax)
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> String {
        match self {
            Self::Doc(data) => mongodb::bson::Bson::Document(data.0.clone())
                .into_relaxed_extjson()
                .to_string(),
        }
    }

    // Get model instance.
    // (For the `save` and `update` operations)
    // ---------------------------------------------------------------------------------------------
    pub fn model<T>(&self) -> Result<T, mongodb::bson::de::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Self::Doc(data) => {
                if !data.0.is_empty() {
                    let doc = data.0.clone();
                    let ignore_fields = data.1.clone();
                    let map_widget_type = data.2.clone();
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
                                    let file_info = bson_val.as_document().unwrap();
                                    let path = file_info.get_str("path").unwrap().to_string();
                                    let url = file_info.get_str("url").unwrap().to_string();
                                    let result =
                                        format!("{{\"path\":\"{}\",\"url\":\"{}\"}}", path, url);
                                    mongodb::bson::Bson::String(result)
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else {
                            prepared_doc.insert(field_name, bson_val);
                        }
                    }
                    mongodb::bson::de::from_document::<T>(prepared_doc)
                } else {
                    let prepared_doc = mongodb::bson::document::Document::new();
                    mongodb::bson::de::from_document::<T>(prepared_doc)
                }
            }
        }
    }

    // Get boolean
    // (For check document availability)
    // ---------------------------------------------------------------------------------------------
    pub fn bool(&self) -> bool {
        match self {
            Self::Doc(data) => !data.0.is_empty(),
        }
    }
}

// To return results after processing queries for many documents.
// *************************************************************************************************
#[derive(Debug, Clone)]
pub enum OutputDataMany {
    Data(
        (
            Option<mongodb::bson::document::Document>,
            Option<mongodb::options::FindOptions>,
            mongodb::sync::Collection,
            Vec<String>,
            std::collections::HashMap<String, String>,
            String,
        ),
    ),
}

impl OutputDataMany {
    // Get documents.
    // (For page templates)
    // ---------------------------------------------------------------------------------------------
    pub fn docs(
        &self,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        match self {
            Self::Data(data) => {
                let mut cursor = data.2.find(data.0.clone(), data.1.clone())?;
                let ignore_fields = data.3.clone();
                let bson_null = &mongodb::bson::Bson::Null;
                let mut docs: Vec<mongodb::bson::document::Document> = Vec::new();
                while let Some(doc) = cursor.next() {
                    let doc = doc?;
                    let map_widget_type = data.4.clone();
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
                                    mongodb::bson::Bson::String(
                                        bson_val.as_object_id().unwrap().to_hex(),
                                    )
                                } else {
                                    Err(format!(
                                        "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                Missing document identifier `_id`.",
                                        data.5.clone()
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
                    docs.push(prepared_doc);
                }

                Ok(docs)
            }
        }
    }

    // Get json-line.
    // (For Ajax)
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Data(data) => {
                let mut cursor = data.2.find(data.0.clone(), data.1.clone())?;
                let ignore_fields = data.3.clone();
                let bson_null = &mongodb::bson::Bson::Null;
                let mut json_line = String::new();
                while let Some(doc) = cursor.next() {
                    let doc = doc?;
                    let map_widget_type = data.4.clone();
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
                                    mongodb::bson::Bson::String(
                                        bson_val.as_object_id().unwrap().to_hex(),
                                    )
                                } else {
                                    Err(format!(
                                        "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                Missing document identifier `_id`.",
                                        data.5.clone()
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

                    json_line = format!(
                        "{},{}",
                        json_line,
                        mongodb::bson::Bson::Document(prepared_doc)
                            .into_relaxed_extjson()
                            .to_string(),
                    );
                }

                Ok(format!("[{}]", &json_line[1..]))
            }
        }
    }

    // Get boolean
    // (For check documents availability)
    // ---------------------------------------------------------------------------------------------
    pub fn bool(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.count()? > 0)
    }

    // Get the number of documents
    // ---------------------------------------------------------------------------------------------
    pub fn count(&self) -> mongodb::error::Result<i64> {
        match self {
            Self::Data(data) => {
                let find_options = data.1.clone().unwrap();
                let mut options = mongodb::options::CountOptions::default();
                options.hint = find_options.hint;
                options.limit = find_options.limit;
                options.max_time = find_options.max_time;
                options.skip = find_options.skip;
                options.collation = find_options.collation;
                data.2.count_documents(data.0.clone(), Some(options))
            }
        }
    }
}
