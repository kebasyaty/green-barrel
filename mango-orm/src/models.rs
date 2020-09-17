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
/// Defining common behavior of models
#[async_trait]
pub trait Model {
    // Get raw attributes for further processing
    fn raw_attrs() -> HashMap<&'static str, Widget>;
    // Get pure attributes for a page templating engine
    fn form_attrs() -> HashMap<String, Transport> {
        let raw_attrs: HashMap<&str, Widget> = Self::raw_attrs();
        let mut clean_attrs: HashMap<String, Transport> = HashMap::new();
        for (field, widget) in &raw_attrs {
            clean_attrs.insert(field.to_string(), widget.get_clean_attrs(field));
        }
        clean_attrs
    }
}
// For Migration -----------------------------------------------------------------------------------
/// Creation and updating of a technical database for monitoring the state of models
#[derive(Serialize, Deserialize)]
pub struct MangoOrmModelState {
    pub database: String,
    pub collection: String,
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
        // Create a technical database if it is missing
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
                        let mut model_state: MangoOrmModelState =
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
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }
    // Reorganize databases state
    // (full delete of orphaned documents, collections and databases)
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
                    let model_state: MangoOrmModelState =
                        bson::de::from_document(document).unwrap();
                    if !model_state.status {
                        // Delete Collection
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
                Err(e) => panic!("{}", e),
            }
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
