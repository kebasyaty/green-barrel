//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).
//! `Model` - Defining common behavior of models.
//! `Monitor` - Creation and updating of a technical database for monitoring the state of models.

use crate::widgets::{Transport, Widget};
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson, bson::document::Document, options::UpdateModifications, Client, Collection, Cursor,
    Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// MODELS ==========================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}

// Model -------------------------------------------------------------------------------------------
/// Custom behavior definition for models
#[async_trait]
pub trait Model {
    // Define attributes for widgets of fields
    fn raw_attrs() -> HashMap<&'static str, Widget>;
    // Define (If necessary) HTML form for page templates
    fn form(attrs: HashMap<String, Transport>) -> String {
        let mut form_text = String::from("<form action=\"/\" method=\"GET\">");
        for (_, trans) in attrs {
            match trans.field_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "checkbox" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.checked { "checked" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "radio" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    let mut tags = String::new();
                    for item in trans.select {
                        tags = format!(
                            "{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}>",
                            label,
                            trans.id,
                            trans.field_type,
                            trans.name,
                            item.1,
                            if trans.checked { "checked" } else { "" },
                            trans.some_classes,
                            trans.other_attrs
                        );
                    }
                    form_text = format!("{}\n{}", form_text, tags);
                }
                "date" | "datetime" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "file" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "image" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "number" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "range" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "textarea" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>\n{}\n</textarea>",
                        form_text,
                        label,
                        trans.id,
                        trans.name,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs,
                        trans.value,
                    );
                }
                "select" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    let mut options = String::new();
                    for item in trans.select {
                        options = format!(
                            "{}\n<option {} value=\"{}\">{}</option>",
                            options,
                            if trans.value == item.1 {
                                "selected"
                            } else {
                                ""
                            },
                            item.1,
                            item.0
                        );
                    }
                    form_text = format!(
                        "{}\n{}\n<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>\n{}\n</select>",
                        form_text,
                        label,
                        trans.id,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs,
                        options,
                    );
                }
                "hidden" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                _ => panic!("Invalid input type."),
            }
        }
        format!("{}\n</form>", form_text)
    }
}

// For Migration -----------------------------------------------------------------------------------
/// Creation and updating of a technical database for monitoring the state of models
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub status: bool,
}

pub struct Monitor<'a> {
    pub keyword: &'a str,
    pub client: &'a Client,
}

impl<'a> Monitor<'a> {
    // Refresh models state
    pub async fn refresh(&self) {
        // Keyword Validation
        let re = Regex::new(r"^[_a-zA-Z\d]{8,16}$").unwrap();
        if !re.is_match(self.keyword) {
            panic!("Keyword - Valid characters: _|a-z|A-Z|0-9 ; Size: 8-16.");
        }
        // Establish a connection with the technical database of the project
        let mango_orm_keyword: String = format!("mango_orm_{}", self.keyword);
        let collection_name: &'static str = "models";
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
                    Err(err) => panic!("{}", err),
                }
            }
        }
    }
    // Reorganize databases state
    // (full delete of orphaned collections and databases)
    pub async fn napalm(&self) {
        // Establish a connection with the technical database of the project
        let mango_orm_keyword: String = format!("mango_orm_{}", self.keyword);
        let collection_name: &'static str = "models";
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
                        // Delete a document with a record about the state of the model from the technical base
                        let query: Document = bson::doc! {
                            "database": &model_state.database,
                            "collection": &model_state.collection
                        };
                        mango_orm_collection.delete_one(query, None).await.unwrap();
                    }
                }
                Err(err) => panic!("{}", err),
            }
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
