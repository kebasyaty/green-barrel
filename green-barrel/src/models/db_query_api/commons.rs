//! Common query methods.

use async_lock::RwLock;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson::{document::Document, Bson},
    options::AggregateOptions,
    options::{
        CountOptions, DeleteOptions, DistinctOptions, DropCollectionOptions,
        EstimatedDocumentCountOptions, FindOneAndDeleteOptions, FindOneOptions, FindOptions,
    },
    Client, Namespace,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::sync::Arc;
use std::{collections::HashMap, error::Error};

use crate::models::{
    caching::Caching, converters::Converters, output_data::OutputData, Main, Meta,
};

/// Common query methods.
#[async_trait(?Send)]
pub trait QCommons: Main + Caching + Converters {
    /// Runs an aggregation operation.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.aggregate
    /// See the documentation https://docs.mongodb.com/manual/aggregation/ for more information on aggregations.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let pipeline = vec![doc! {}];
    /// let doc_list  = ModelName::aggregate(pipeline, &meta_store, &client, None)?;
    /// println!("{:?}", doc_list);
    /// ```
    ///
    async fn aggregate(
        pipeline: Vec<Document>,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `aggregate()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll
            .aggregate(pipeline, options)
            .await?
            .map(|item| item.unwrap())
            .collect()
            .await)
    }

    /// Gets the number of documents matching filter.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.count_documents
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let filter = doc!{};
    /// let count  = ModelName::count_documents(&meta_store, &client, Some(filter), None)?;
    /// println!("{}", count);
    /// ```
    ///
    async fn count_documents(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> Result<u64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `count_documents()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.count_documents(filter, options).await?)
    }

    /// Deletes all documents stored in the collection matching query.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.delete_many
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let query = doc!{};
    /// let output_data  = ModelName::delete_many(query, &meta_store, &client, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn delete_many(
        query: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `delete_many()` => \
                Failed to get data from cache.",
            ))?
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        //
        let mut deleted_count = 0;
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll = client
                .database(meta.database_name.as_str())
                .collection::<Document>(meta.collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_many(query, options).await?.deleted_count;
            true
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Deletes up to one document found matching query.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.delete_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let query = doc!{};
    /// let output_data  = ModelName::delete_one(query, &meta_store, &client, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn delete_one(
        query: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `delete_one()` => \
                Failed to get data from cache.",
            ))?
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        //
        let mut deleted_count = 0;
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll = client
                .database(meta.database_name.as_str())
                .collection::<Document>(meta.collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_one(query, options).await?.deleted_count;
            true
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Finds the distinct values of the field specified by field_name across the collection.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.distinct
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let field_name = "";
    /// let filter = doc!{};
    /// let output_data  = ModelName::distinct(field_name, &meta_store, &client, Some(filter), None)?;
    /// println!("{:?}", output_data);
    /// ```
    ///
    async fn distinct(
        field_name: &str,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        filter: Option<Document>,
        options: Option<DistinctOptions>,
    ) -> Result<Vec<Bson>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `distinct()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.distinct(field_name, filter, options).await?)
    }

    /// Drops the collection, deleting all data and indexes stored in it.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.drop
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let output_data  = ModelName::drop(&meta_store, &client, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn drop(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<DropCollectionOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `drop()` => \
                Failed to get data from cache.",
            ))?
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        let err_msg = if is_permission_delete {
            String::new()
        } else {
            "It is forbidden to perform delete.".to_string()
        };
        // Get a logical result.
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll = client
                .database(meta.database_name.as_str())
                .collection::<Document>(meta.collection_name.as_str());
            // Execute query.
            coll.drop(options).await.is_ok()
        } else {
            false
        };
        let deleted_count = u64::from(result_bool);
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Estimates the number of documents in the collection using collection metadata.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.estimated_document_count
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let count  = ModelName::estimated_document_count(&meta_store, &client, None)?;
    /// println!("{}", count);
    /// ```
    ///
    async fn estimated_document_count(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<EstimatedDocumentCountOptions>,
    ) -> Result<u64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `estimated_document_count()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.estimated_document_count(options).await?)
    }

    /// Finds the documents in the collection matching filter and
    /// return document list ( missing fields type ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_doc_list(&meta_store, &client, None, None)?;
    /// if let Some(doc_list) = result {
    ///     println!("{:?}", doc_list);
    /// }
    /// ```
    ///
    async fn find_many_to_doc_list(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<Vec<Document>>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_many_to_doc_list()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if let Some(mut options) = options {
            if options.limit == Some(0_i64) {
                options.limit = Some(meta.db_query_docs_limit as i64);
            }
            options
        } else {
            FindOptions::builder()
                .limit(Some(meta.db_query_docs_limit as i64))
                .build()
        };
        // Execute query.
        let doc_list = Self::many_to_doc_list(filter, Some(options), coll).await?;
        if doc_list.is_empty() {
            return Ok(None);
        }
        Ok(Some(doc_list))
    }

    /// Finds the documents in the collection matching filter and
    /// return in JSON format ( missing fields type ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_json(&meta_store, &client, None, None)?;
    /// if let Some(json_line) = result {
    ///     println!("{}", json_line);
    /// }
    /// ```
    ///
    async fn find_many_to_json(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<String>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_many_to_json()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if let Some(mut options) = options {
            if options.limit == Some(0_i64) {
                options.limit = Some(meta.db_query_docs_limit as i64);
            }
            options
        } else {
            FindOptions::builder()
                .limit(Some(meta.db_query_docs_limit as i64))
                .build()
        };
        // Execute query.
        Self::many_to_json(
            filter,
            Some(options),
            coll,
            &meta.ignore_fields,
            &meta.field_type_map,
            meta.model_name.as_str(),
        )
        .await
    }

    /// Finds a single document in the collection matching filter and
    /// return in Doc format ( missing fields type ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result = ModelName::find_one_to_doc(filter, &meta_store, &client, None)?;
    /// if let Some(doc) = result {
    ///     println!("{:?}", doc);
    /// }
    /// ```
    ///
    async fn find_one_to_doc(
        filter: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_one_to_doc()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.find_one(filter, options).await?)
    }

    /// Finds a single document in the collection matching filter and
    /// return in JSON format.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let json = ModelName::find_one_to_json(filter, &meta_store, &client, None)?;
    /// println!("{}", json);
    /// ```
    ///
    async fn find_one_to_json(
        filter: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_one_to_json()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Get document from database and convert to model instance in jsob-line format.
        if let Ok(Some(db_doc)) = coll.find_one(filter, options).await {
            let mut model_json = meta.model_json.clone();
            Self::one_to_json_val(
                db_doc,
                &meta.ignore_fields,
                &meta.field_type_map,
                &meta.model_name,
                &meta.fields_name,
                &mut model_json,
            )?;
            return Ok(serde_json::to_string(&model_json)?);
        }
        //
        Ok(String::new())
    }

    /// Finds a single document in the collection matching filter and
    /// return as model instance.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_instance(filter, &meta_store, &client, None)?;
    /// if let Some(instance) = result {
    ///     println!("{:?}", instance);
    /// }
    /// ```
    ///
    async fn find_one_to_instance(
        filter: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_one_to_instance()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Get document from database and convert to model instance.
        if let Ok(Some(db_doc)) = coll.find_one(filter, options).await {
            let mut model_json = meta.model_json.clone();
            Self::one_to_json_val(
                db_doc,
                &meta.ignore_fields,
                &meta.field_type_map,
                &meta.model_name,
                &meta.fields_name,
                &mut model_json,
            )?;
            return Ok(Some(serde_json::from_value(model_json)?));
        }
        //
        Ok(None)
    }

    /// Atomically finds up to one document in the collection matching filter and
    /// deletes it ( missing fields type ).
    /// Returns the deleted document (in Doc format).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one_and_delete
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_and_delete(filter, &meta_store, &client, None)?;
    /// if let Some(doc) = result) {
    ///     println!("{:?}", doc);
    /// }
    /// ```
    ///
    async fn find_one_and_delete(
        filter: Document,
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `find_one_and_delete()` => \
                Failed to get data from cache.",
            ))?
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if !is_permission_delete {
            Err("It is forbidden to perform delete.".to_string())?
        }
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.find_one_and_delete(filter, options).await?)
    }

    /// Gets the name of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.name
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = ModelName::name(&meta_store, &client)?;
    /// println!("{}", name);
    /// ```
    ///
    async fn collection_name(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `collection_name()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.name().to_string())
    }

    /// Gets the namespace of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.namespace
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = ModelName::namespace(&meta_store, &client)?;
    /// println!("{:?}", name);
    /// ```
    ///
    async fn namespace(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
    ) -> Result<Namespace, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get a key to access the metadata store.
        let key = Self::key()?;
        // Get metadata store.
        let store = meta_store.read().await;
        // Get metadata of Model.
        let meta = if let Some(meta) = store.get(&key) {
            meta
        } else {
            Err(format!(
                "Model key: `{key}` ; Method: `namespace()` => \
                Failed to get data from cache.",
            ))?
        };
        // Access collection.
        let coll = client
            .database(meta.database_name.as_str())
            .collection::<Document>(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.namespace())
    }
}
