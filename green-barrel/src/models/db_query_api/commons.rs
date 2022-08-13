//! Common query methods.

use mongodb::{
    bson::{document::Document, Bson},
    options::AggregateOptions,
    options::{
        CountOptions, DeleteOptions, DistinctOptions, DropCollectionOptions,
        EstimatedDocumentCountOptions, FindOneAndDeleteOptions, FindOneOptions, FindOptions,
    },
    sync::Collection,
    Namespace,
};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::{collections::HashMap, error::Error};

use crate::models::{
    caching::Caching, converters::Converters, output_data::OutputData, Main, Meta,
};

/// Common query methods.
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
    /// let documents  = ModelName::aggregate(pipeline, None)?;
    /// println!("{:?}", documents);
    /// ```
    ///
    fn aggregate(
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll
            .aggregate(pipeline, options)?
            .map(|item| item.unwrap())
            .collect())
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
    /// let count  = ModelName::count_documents(Some(filter), None)?;
    /// println!("{}", count);
    /// ```
    ///
    fn count_documents(
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> Result<i64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.count_documents(filter, options)?)
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
    /// let output_data  = ModelName::delete_many(query, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_many(
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
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
        let mut deleted_count = 0_i64;
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_many(query, options)?.deleted_count;
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
    /// let output_data  = ModelName::delete_one(query, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_one(
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
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
        let mut deleted_count = 0_i64;
        let result_bool = if is_permission_delete {
            // Access collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            deleted_count = coll.delete_one(query, options)?.deleted_count;
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
    /// let output_data  = ModelName::distinct(field_name, Some(filter), None)?;
    /// println!("{:?}", output_data);
    /// ```
    ///
    fn distinct(
        field_name: &str,
        filter: Option<Document>,
        options: Option<DistinctOptions>,
    ) -> Result<Vec<Bson>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.distinct(field_name, filter, options)?)
    }

    /// Drops the collection, deleting all data and indexes stored in it.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.drop
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let output_data  = ModelName::drop(None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn drop(options: Option<DropCollectionOptions>) -> Result<OutputData, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
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
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            coll.drop(options).is_ok()
        } else {
            false
        };
        let deleted_count = if result_bool { 1_i64 } else { 0_i64 };
        Ok(OutputData::Delete((result_bool, err_msg, deleted_count)))
    }

    /// Estimates the number of documents in the collection using collection metadata.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.estimated_document_count
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let count  = ModelName::estimated_document_count(None)?;
    /// println!("{}", count);
    /// ```
    ///
    fn estimated_document_count(
        options: Option<EstimatedDocumentCountOptions>,
    ) -> Result<i64, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.estimated_document_count(options)?)
    }

    /// Finds the documents in the collection matching filter and
    /// return document list ( missing widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_doc_list(None, None)?;
    /// if result.is_some() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_many_to_doc_list(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<Vec<Document>>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if options.is_some() {
            let mut options = options.unwrap();
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
        let doc_list = Self::many_to_doc_list(filter, Some(options), coll)?;
        if doc_list.is_empty() {
            return Ok(None);
        }
        Ok(Some(doc_list))
    }

    /// Finds the documents in the collection matching filter and
    /// return in JSON format ( missing widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let result = ModelName::find_many_to_json_line(None, None);
    /// if result.is_ok() {
    ///     println!("{}", result?);
    /// }
    /// ```
    ///
    fn find_many_to_json_line(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Apply parameter `db_query_docs_limit`.
        // (if necessary)
        let options = if options.is_some() {
            let mut options = options.unwrap();
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
        Self::many_to_json_line(
            filter,
            Some(options),
            coll,
            &meta.ignore_fields,
            &meta.widget_type_map,
            meta.model_name.as_str(),
        )
    }

    /// Finds a single document in the collection matching filter and
    /// return in Doc format ( missing widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_doc(filter, None)?;
    /// if result.is_some() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_doc(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.find_one(filter, options)?)
    }

    /// Finds a single document in the collection matching filter and
    /// return in JSON format ( presence of widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_json_line(filter, None);
    /// if result.is_ok() {
    ///     println!("{}", result);
    /// }
    /// ```
    ///
    fn find_one_to_json_line(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        let mut model_json = &model_cache.model_json.clone();
        if let Ok(Some(db_doc)) = coll.find_one(filter, options) {
            Self::one_doc_to_json_val(
                db_doc,
                &meta.ignore_fields,
                &meta.widget_type_map,
                &meta.model_name,
                &meta.fields_name,
                model_json,
            )?;
            return Ok(serde_json::to_string(model_json)?);
        }
        //
        Ok(String::new())
    }

    /// Finds a single document in the collection matching filter and
    /// return widget map.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_wig(filter, None)?;
    /// if result.is_some()) {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_wig(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<HashMap<String, Widget>>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Self::one_to_wig(
            coll.find_one(filter, options)?,
            &meta.ignore_fields,
            &meta.widget_type_map,
            &meta.model_name,
            &meta.fields_name,
            model_cache.widget_map.clone(),
        )
    }

    /// Finds a single document in the collection matching filter and
    /// return as model instance ( missing widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_to_model_instance(filter, None);
    /// if result.is_ok() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_model_instance(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Self::to_model_instance(
            coll.find_one(filter, options)?,
            &meta.ignore_fields,
            &meta.widget_type_map,
            meta.model_name.as_str(),
        )
    }

    /// Atomically finds up to one document in the collection matching filter and
    /// deletes it ( missing widgets ).
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
    /// let result  = ModelName::find_one_and_delete_to_doc(filter, None);
    /// if result.is_ok() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_doc(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if !is_permission_delete {
            Err("It is forbidden to perform delete.".to_string())?
        }
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.find_one_and_delete(filter, options)?)
    }

    /// Atomically finds up to one document in the collection matching filter and
    /// deletes it ( missing widgets ).
    /// Returns the deleted document (in JSON format).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one_and_delete
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_and_delete_to_json(filter, None);
    /// if result.is_ok() {
    ///     println!("{}", result);
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_json(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if !is_permission_delete {
            Err("It is forbidden to perform delete.".to_string())?
        }
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        let doc = coll.find_one_and_delete(filter, options)?;
        if doc.is_none() {
            return Ok(String::new());
        }
        let widget_map = Self::one_to_wig(
            doc,
            &meta.ignore_fields,
            &meta.widget_type_map,
            &meta.model_name,
            &meta.fields_name,
            model_cache.widget_map.clone(),
        )?;
        if widget_map.is_none() {
            return Ok(String::new());
        }
        Self::widget_map_to_json(widget_map.unwrap())
    }

    /// Atomically finds up to one document in the collection matching filter and
    /// deletes it ( missing widgets ).
    /// Returns the deleted document (in Model instance).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find_one_and_delete
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// ```
    /// use mongodb::bson::doc;
    /// let filter = doc!{"username": "user_1"};
    /// let result  = ModelName::find_one_and_delete_to_model_instance(filter, None)?;
    /// if result.is_some() {
    ///     println!("{}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_model_instance(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if !is_permission_delete {
            Err("It is forbidden to perform delete.".to_string())?
        }
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Self::to_model_instance(
            coll.find_one_and_delete(filter, options)?,
            &meta.ignore_fields,
            &meta.widget_type_map,
            meta.model_name.as_str(),
        )
    }

    /// Gets the name of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.name
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = ModelName::name()?;
    /// println!("{}", name);
    /// ```
    ///
    fn name() -> Result<String, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
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
    /// let name  = ModelName::namespace()?;
    /// println!("{:?}", name);
    /// ```
    ///
    fn namespace() -> Result<Namespace, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Sized,
    {
        // Get cached Model data.
        let (model_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = model_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.namespace())
    }
}
