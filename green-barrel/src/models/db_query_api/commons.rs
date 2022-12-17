//! Common query methods.

use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, document::Document, Bson},
    options::{
        AggregateOptions, CountOptions, CreateIndexOptions, DeleteOptions, DistinctOptions,
        DropCollectionOptions, EstimatedDocumentCountOptions, FindOneAndDeleteOptions,
        FindOneOptions, FindOptions,
    },
    results::CreateIndexResult,
    Client, IndexModel, Namespace,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::error::Error;

use crate::{
    meta_store::META_STORE,
    models::{caching::Caching, converters::Converters, output_data::OutputData, Main},
};

/// Common query methods.
#[async_trait(?Send)]
pub trait QCommons: Main + Caching + Converters {
    /// Creates the given index on this collection.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.create_index
    ///
    /// # Example:
    ///
    /// ```
    /// let options = IndexOptions::builder().unique(true).build();
    /// let index = IndexModel::builder()
    ///     .keys(doc! { "username": 1 })
    ///     .options(options)
    ///     .build();
    /// let result  = ModelName::create_index(&client, index, None).await?;
    /// println!("{:?}", result;
    /// ```
    ///
    async fn create_index(
        client: &Client,
        index: IndexModel,
        options: impl Into<Option<CreateIndexOptions>>,
    ) -> Result<CreateIndexResult, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `create_index()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        Ok(client
            .database(&database_name)
            .collection::<Document>(&collection_name)
            .create_index(index, options)
            .await?)
    }

    /// Runs an aggregation operation.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.aggregate
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let pipeline = vec![doc! {}];
    /// let doc_list  = ModelName::aggregate(pipeline, &client, None).await?;
    /// println!("{:?}", doc_list);
    /// ```
    ///
    async fn aggregate(
        pipeline: Vec<Document>,
        client: &Client,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll
            .aggregate(pipeline, options)
            .await?
            .map(|item| item.unwrap())
            .collect()
            .await)
    }

    /// Gets the number of documents matching filter.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.count_documents
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let filter = doc!{};
    /// let count  = ModelName::count_documents &client, Some(filter), None).await?;
    /// println!("{}", count);
    /// ```
    ///
    async fn count_documents(
        client: &Client,
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> Result<u64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.count_documents(filter, options).await?)
    }

    /// Deletes all documents stored in the collection matching query.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.delete_many
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let query = doc!{};
    /// let output_data  = ModelName::delete_many(query, &client, None).await?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn delete_many(
        query: Document,
        client: &Client,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, is_del_doc) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.is_del_doc,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = is_del_doc;
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
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_many(query, options).await?.deleted_count;
            true
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Deletes up to one document found matching query.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.delete_one
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let query = doc!{};
    /// let output_data  = ModelName::delete_one(query, &client, None).await?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn delete_one(
        query: Document,
        client: &Client,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, is_del_doc) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.is_del_doc,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = is_del_doc;
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
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_one(query, options).await?.deleted_count;
            true
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Finds the distinct values of the field specified by field_name across the collection.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.distinct
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    ///
    /// let field_name = "";
    /// let filter = doc!{};
    /// let output_data  = ModelName::distinct(field_name, &client, Some(filter), None).await?;
    /// println!("{:?}", output_data);
    /// ```
    ///
    async fn distinct(
        field_name: &str,
        client: &Client,
        filter: Option<Document>,
        options: Option<DistinctOptions>,
    ) -> Result<Vec<Bson>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.distinct(field_name, filter, options).await?)
    }

    /// Drops the collection, deleting all data and indexes stored in it.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.drop
    ///
    /// # Example:
    ///
    /// ```
    /// let output_data  = ModelName::drop &client, None).await?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    async fn drop(
        client: &Client,
        options: Option<DropCollectionOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, is_del_doc) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.is_del_doc,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = is_del_doc;
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
                .database(database_name.as_str())
                .collection::<Document>(collection_name.as_str());
            // Execute query.
            coll.drop(options).await.is_ok()
        } else {
            false
        };
        let deleted_count = u64::from(result_bool);
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Estimates the number of documents in the collection using collection metadata.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.estimated_document_count
    ///
    /// # Example:
    ///
    /// ```
    /// let count  = ModelName::estimated_document_count &client, None).await?;
    /// println!("{}", count);
    /// ```
    ///
    async fn estimated_document_count(
        client: &Client,
        options: Option<EstimatedDocumentCountOptions>,
    ) -> Result<u64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.estimated_document_count(options).await?)
    }

    /// Finds the documents in the collection matching filter and
    /// return document list ( missing fields type ).
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_doc_list &client, None, None).await?;
    /// if let Some(doc_list) = result {
    ///     println!("{:?}", doc_list);
    /// }
    /// ```
    ///
    async fn find_many_to_doc_list(
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<Vec<Document>>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, db_query_docs_limit) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.db_query_docs_limit,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if let Some(mut options) = options {
            if options.limit == Some(0) {
                options.limit = Some(db_query_docs_limit as i64);
            }
            options
        } else {
            FindOptions::builder()
                .limit(Some(db_query_docs_limit as i64))
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
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_json &client, None, None).await?;
    /// if let Some(json_line) = result {
    ///     println!("{}", json_line);
    /// }
    /// ```
    ///
    async fn find_many_to_json(
        client: &Client,
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<String>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (
            database_name,
            collection_name,
            db_query_docs_limit,
            ignore_fields,
            field_type_map,
            model_name,
        ) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.db_query_docs_limit,
                    meta.ignore_fields.clone(),
                    meta.field_type_map.clone(),
                    meta.model_name.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if let Some(mut options) = options {
            if options.limit == Some(0) {
                options.limit = Some(db_query_docs_limit as i64);
            }
            options
        } else {
            FindOptions::builder()
                .limit(Some(db_query_docs_limit as i64))
                .build()
        };
        // Execute query.
        Self::many_to_json(
            filter,
            Some(options),
            coll,
            &ignore_fields,
            &field_type_map,
            model_name.as_str(),
        )
        .await
    }

    /// Finds a single document in the collection matching filter and
    /// return in Doc format ( missing fields type ).
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find_one
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result = ModelName::find_one_to_doc(filter, &client, None).await?;
    /// if let Some(doc) = result {
    ///     println!("{:?}", doc);
    /// }
    /// ```
    ///
    async fn find_one_to_doc(
        filter: Document,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.find_one(filter, options).await?)
    }

    /// Finds a single document in the collection matching filter and
    /// return in JSON format.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find_one
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let json = ModelName::find_one_to_json(filter, &client, None).await?;
    /// println!("{}", json);
    /// ```
    ///
    async fn find_one_to_json(
        filter: Document,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (
            database_name,
            collection_name,
            ignore_fields,
            field_type_map,
            model_name,
            model_json,
            fields_name,
        ) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.ignore_fields.clone(),
                    meta.field_type_map.clone(),
                    meta.model_name.clone(),
                    meta.model_json.clone(),
                    meta.fields_name.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Get document from database and convert to model instance in jsob-line format.
        if let Ok(Some(db_doc)) = coll.find_one(filter, options).await {
            let mut model_json = model_json.clone();
            Self::one_to_json_val(
                db_doc,
                &ignore_fields,
                &field_type_map,
                &model_name,
                &fields_name,
                &mut model_json,
            )?;
            return Ok(serde_json::to_string(&model_json)?);
        }
        //
        Ok(String::new())
    }

    /// Finds a single document in the collection matching filter and
    /// return as model instance.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find_one
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_instance(filter, &client, None).await?;
    /// if let Some(instance) = result {
    ///     println!("{:?}", instance);
    /// }
    /// ```
    ///
    async fn find_one_to_instance(
        filter: Document,
        client: &Client,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (
            database_name,
            collection_name,
            ignore_fields,
            field_type_map,
            model_name,
            model_json,
            fields_name,
        ) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.ignore_fields.clone(),
                    meta.field_type_map.clone(),
                    meta.model_name.clone(),
                    meta.model_json.clone(),
                    meta.fields_name.clone(),
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Get document from database and convert to model instance.
        if let Ok(Some(db_doc)) = coll.find_one(filter, options).await {
            let mut model_json = model_json.clone();
            Self::one_to_json_val(
                db_doc,
                &ignore_fields,
                &field_type_map,
                &model_name,
                &fields_name,
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
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.find_one_and_delete
    ///
    /// # Example:
    ///
    /// ```
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_and_delete(filter, &client, None).await?;
    /// if let Some(doc) = result) {
    ///     println!("{:?}", doc);
    /// }
    /// ```
    ///
    async fn find_one_and_delete(
        filter: Document,
        client: &Client,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name, is_del_doc) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (
                    meta.database_name.clone(),
                    meta.collection_name.clone(),
                    meta.is_del_doc,
                )
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Get permission to delete the document.
        let is_permission_delete: bool = is_del_doc;
        //
        if !is_permission_delete {
            Err("It is forbidden to perform delete.".to_string())?
        }
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.find_one_and_delete(filter, options).await?)
    }

    /// Gets the name of the Collection.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.name
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = ModelName::name &client).await?;
    /// println!("{}", name);
    /// ```
    ///
    async fn collection_name(client: &Client) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.name().to_string())
    }

    /// Gets the namespace of the Collection.
    /// https://docs.rs/mongodb/latest/mongodb/struct.Collection.html#method.namespace
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = ModelName::namespace &client).await?;
    /// println!("{:?}", name);
    /// ```
    ///
    async fn namespace(client: &Client) -> Result<Namespace, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        let (database_name, collection_name) = {
            // Get a key to access the metadata store.
            let key = Self::key()?;
            // Get metadata store.
            let store = META_STORE.lock().await;
            // Get metadata of Model.
            if let Some(meta) = store.get(&key) {
                (meta.database_name.clone(), meta.collection_name.clone())
            } else {
                Err(format!(
                    "Model key: `{key}` ; Method: `aggregate()` => \
                    Failed to get data from cache.",
                ))?
            }
        };
        // Access collection.
        let coll = client
            .database(database_name.as_str())
            .collection::<Document>(collection_name.as_str());
        // Execute query.
        Ok(coll.namespace())
    }
}
