//! # Common database query methods.
//!
//! Trait:
//! `QCommon` - Common database query methods.
//! Methods:
//! 'aggregate' - Runs an aggregation operation.
//! `count_documents` - Gets the number of documents matching filter.
//! `delete_many` - Deletes all documents stored in the collection matching query.
//! `delete_one` - Finds a single document in the collection matching filter.
//! `distinct` - Finds the distinct values of the field specified by field_name across the collection.
//! `drop` - Drops the collection, deleting all data and indexes stored in it.
//! `estimated_document_count` - Estimates the number of documents in the collection using collection metadata.
//! `find` - Finds the documents in the collection matching filter.
//! `find_one` - Finds a single document in the collection matching filter.
//! `find_one_and_delete` - Atomically finds up to one document in the collection matching filter and deletes it.
//! `name` - Gets the name of the Collection.
//! `namespace` - Gets the namespace of the Collection.
//!

use crate::models::{
    output_data::{OutputDataMany, OutputDataOne},
    Meta, ToModel,
};

pub trait QCommon: ToModel {
    // Runs an aggregation operation.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.aggregate
    // ---------------------------------------------------------------------------------------------
    fn aggregate(
        pipeline: Vec<mongodb::bson::document::Document>,
        options: Option<mongodb::options::AggregateOptions>,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll
            .aggregate(pipeline, options)?
            .map(|item| item.unwrap())
            .collect())
    }

    // Gets the number of documents matching filter.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.count_documents
    // ---------------------------------------------------------------------------------------------
    fn count_documents(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::CountOptions>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.count_documents(filter, options)?)
    }

    // Deletes all documents stored in the collection matching query.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.delete_many
    // ---------------------------------------------------------------------------------------------
    fn delete_many(
        query: mongodb::bson::document::Document,
        options: Option<mongodb::options::DeleteOptions>,
    ) -> Result<mongodb::results::DeleteResult, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.delete_many(query, options)?)
    }

    // Deletes up to one document found matching query.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.delete_one
    // ---------------------------------------------------------------------------------------------
    fn delete_one(
        query: mongodb::bson::document::Document,
        options: Option<mongodb::options::DeleteOptions>,
    ) -> Result<mongodb::results::DeleteResult, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.delete_one(query, options)?)
    }

    // Finds the distinct values of the field specified by field_name across the collection.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.distinct
    // ---------------------------------------------------------------------------------------------
    fn distinct(
        field_name: &str,
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::DistinctOptions>,
    ) -> Result<Vec<mongodb::bson::Bson>, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.distinct(field_name, filter, options)?)
    }

    // Drops the collection, deleting all data and indexes stored in it.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.drop
    // ---------------------------------------------------------------------------------------------
    fn drop(
        options: Option<mongodb::options::DropCollectionOptions>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.drop(options)?)
    }

    // Estimates the number of documents in the collection using collection metadata.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.estimated_document_count
    // ---------------------------------------------------------------------------------------------
    fn estimated_document_count(
        options: Option<mongodb::options::EstimatedDocumentCountOptions>,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.estimated_document_count(options)?)
    }

    // Finds the documents in the collection matching filter.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    fn find(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::FindOptions>,
    ) -> Result<OutputDataMany, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        let mut options_mod = mongodb::options::FindOptions::default();
        if options.is_some() {
            options_mod = options.unwrap();
        };
        options_mod.limit = Some(meta.db_query_docs_limit as i64);
        // Database Query
        Ok(OutputDataMany::Data((
            filter,
            Some(options_mod),
            coll,
            meta.ignore_fields.clone(),
            meta.map_widget_type.clone(),
            meta.model_name.clone(),
        )))
    }

    // Finds a single document in the collection matching filter.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    fn find_one(
        filter: Option<mongodb::bson::document::Document>,
        options: Option<mongodb::options::FindOneOptions>,
    ) -> Result<OutputDataOne, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(if let Some(doc) = coll.find_one(filter, options)? {
            Self::to_prepared_doc(doc, &meta)?
        } else {
            OutputDataOne::default()
        })
    }

    // Atomically finds up to one document in the collection matching filter and deletes it.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.find_one_and_delete
    // ---------------------------------------------------------------------------------------------
    fn find_one_and_delete(
        filter: mongodb::bson::document::Document,
        options: Option<mongodb::options::FindOneAndDeleteOptions>,
    ) -> Result<OutputDataOne, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(
            if let Some(doc) = coll.find_one_and_delete(filter, options)? {
                Self::to_prepared_doc(doc, &meta)?
            } else {
                OutputDataOne::default()
            },
        )
    }

    // Gets the name of the Collection.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.name
    // ---------------------------------------------------------------------------------------------
    fn name() -> Result<String, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.name().to_string())
    }

    // Gets the namespace of the Collection.
    // https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.namespace
    // ---------------------------------------------------------------------------------------------
    fn namespace() -> Result<mongodb::Namespace, Box<dyn std::error::Error>> {
        // Get cached Model data
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection
        let coll: mongodb::sync::Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Database Query
        Ok(coll.namespace())
    }

    // Prepared doc for `OutputDataOne`.
    // (Convert data types to a convenient format.)
    // ---------------------------------------------------------------------------------------------
    fn to_prepared_doc(
        doc: mongodb::bson::document::Document,
        meta: &Meta,
    ) -> Result<OutputDataOne, Box<dyn std::error::Error>> {
        let map_widget_type = meta.map_widget_type.clone();
        let ignore_fields = meta.ignore_fields.clone();
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
                            "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                Missing document identifier `_id`.",
                            meta.model_name.clone()
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

        Ok(OutputDataOne::Doc((
            prepared_doc,
            meta.ignore_fields.clone(),
            meta.map_widget_type.clone(),
        )))
    }
}
