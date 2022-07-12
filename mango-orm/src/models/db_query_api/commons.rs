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
use std::{collections::HashMap, error::Error};

use crate::{
    models::{caching::CachingModel, converters::Converters, Main, Meta},
    widgets::{output_data::OutputData, Widget},
};

pub trait QCommons: Main + CachingModel + Converters {
    /// Runs an aggregation operation.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.aggregate
    /// See the documentation https://docs.mongodb.com/manual/aggregation/ for more information on aggregations.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let pipeline = doc!{};
    /// let documents  = UserProfile::aggregate(pipeline, None)?;
    /// println!("{:?}", documents);
    /// ```
    ///
    fn aggregate(
        pipeline: Vec<Document>,
        options: Option<AggregateOptions>,
    ) -> Result<Vec<Document>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
    /// let filter = doc!{};
    /// let count  = UserProfile::count_documents(Some(filter), None)?;
    /// println!("{}", count);
    /// ```
    ///
    fn count_documents(
        filter: Option<Document>,
        options: Option<CountOptions>,
    ) -> Result<i64, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
    /// let query = doc!{};
    /// let output_data  = UserProfile::delete_many(query, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_many(
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
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
            coll.delete_many(query, options).is_ok()
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg)))
    }

    /// Deletes up to one document found matching query.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.delete_one
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let query = doc!{};
    /// let output_data  = UserProfile::delete_one(query, None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn delete_one(
        query: Document,
        options: Option<DeleteOptions>,
    ) -> Result<OutputData, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        // Error message for the client.
        // (Main use for admin panel.)
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
            coll.delete_one(query, options).is_ok()
        } else {
            false
        };
        Ok(OutputData::Delete((result_bool, err_msg)))
    }

    /// Finds the distinct values of the field specified by field_name across the collection.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.distinct
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let field_name = "";
    /// let filter = doc!{};
    /// let output_data  = UserProfile::distinct(field_name, Some(filter), None)?;
    /// println!("{:?}", output_data);
    /// ```
    ///
    fn distinct(
        field_name: &str,
        filter: Option<Document>,
        options: Option<DistinctOptions>,
    ) -> Result<Vec<Bson>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
    /// let output_data  = UserProfile::drop(None)?;
    /// if !output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    fn drop(options: Option<DropCollectionOptions>) -> Result<OutputData, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
        Ok(OutputData::Delete((result_bool, err_msg)))
    }

    /// Estimates the number of documents in the collection using collection metadata.
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.estimated_document_count
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let count  = UserProfile::estimated_document_count(None)?;
    /// println!("{}", count);
    /// ```
    ///
    fn estimated_document_count(
        options: Option<EstimatedDocumentCountOptions>,
    ) -> Result<i64, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
    /// let result = UserProfile::find_many_to_doc(None, None)?;
    /// if result.is_some() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_many_to_doc(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<Option<Vec<Document>>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
        let docs = Self::many_to_docs(
            filter,
            Some(options),
            coll,
            &meta.ignore_fields,
            &meta.map_widget_type,
            meta.model_name.as_str(),
        )?;
        if !docs.is_empty() {
            Ok(Some(docs))
        } else {
            Ok(None)
        }
    }

    /// Finds the documents in the collection matching filter and
    /// return in JSON format ( missing widgets ).
    /// https://docs.rs/mongodb/1.2.5/mongodb/struct.Collection.html#method.find
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let result = UserProfile::find_many_to_json(None, None);
    /// if result.is_ok() {
    ///     println!("{}", result?);
    /// }
    /// ```
    ///
    fn find_many_to_json(
        filter: Option<Document>,
        options: Option<FindOptions>,
    ) -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
        Self::many_to_json(
            filter,
            Some(options),
            coll,
            &meta.ignore_fields,
            &meta.map_widget_type,
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_to_doc(filter, None)?;
    /// if result.is_some() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_doc(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        let doc = coll.find_one(filter, options)?;
        if doc.is_some() {
            Ok(Some(Self::to_prepared_doc(
                doc.unwrap(),
                &meta.ignore_fields,
                &meta.map_widget_type,
                meta.model_name.as_str(),
            )?))
        } else {
            Ok(None)
        }
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_to_json(filter, None);
    /// if result.is_ok() {
    ///     println!("{}", result);
    /// }
    /// ```
    ///
    fn find_one_to_json(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        let widget_map = Self::one_to_wig(
            coll.find_one(filter, options)?,
            &meta.ignore_fields,
            &meta.map_widget_type,
            &meta.model_name,
            &meta.fields_name,
            form_cache.map_widgets.clone(),
        )?;

        if widget_map.is_some() {
            let json = serde_json::to_value(widget_map.unwrap())?;
            let json = serde_json::to_string(&json)?;
            Ok(json)
        } else {
            Ok(String::new())
        }
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_to_wig(filter, None)?;
    /// if result.is_some()) {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_wig(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<HashMap<String, Widget>>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Self::one_to_wig(
            coll.find_one(filter, options)?,
            &meta.ignore_fields,
            &meta.map_widget_type,
            &meta.model_name,
            &meta.fields_name,
            form_cache.map_widgets.clone(),
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_to_model_instance::<UserProfile>(filter, None);
    /// if result.is_ok() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_to_model_instance<T>(
        filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<T>, Box<dyn Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Self::to_model_instance::<T>(
            coll.find_one(filter, options)?,
            &meta.ignore_fields,
            &meta.map_widget_type,
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_and_delete_to_doc(filter, None);
    /// if result.is_ok() {
    ///     println!("{:?}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_doc(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<Document>, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if is_permission_delete {
            // Access collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            let doc = coll.find_one_and_delete(filter, options)?;
            if doc.is_some() {
                Ok(Some(Self::to_prepared_doc(
                    doc.unwrap(),
                    &meta.ignore_fields,
                    &meta.map_widget_type,
                    meta.model_name.as_str(),
                )?))
            } else {
                Ok(None)
            }
        } else {
            Err("It is forbidden to perform delete.".to_string())?
        }
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_and_delete_to_json(filter, None);
    /// if result.is_ok() {
    ///     println!("{}", result);
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_json(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if is_permission_delete {
            // Access collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            let doc = coll.find_one_and_delete(filter, options)?;
            if doc.is_some() {
                Ok(Bson::Document(Self::to_prepared_doc(
                    doc.unwrap(),
                    &meta.ignore_fields,
                    &meta.map_widget_type,
                    meta.model_name.as_str(),
                )?)
                .into_relaxed_extjson()
                .to_string())
            } else {
                Ok(String::new())
            }
        } else {
            Err("It is forbidden to perform delete.".to_string())?
        }
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
    /// let filter = doc!{"username", "user_1"};
    /// let result  = UserProfile::find_one_and_delete_to_model_instance::<UserProfile>(filter, None)?;
    /// if result.is_some() {
    ///     println!("{}", result.unwrap());
    /// }
    /// ```
    ///
    fn find_one_and_delete_to_model_instance<T>(
        filter: Document,
        options: Option<FindOneAndDeleteOptions>,
    ) -> Result<Option<T>, Box<dyn Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Get permission to delete the document.
        let is_permission_delete: bool = meta.is_del_docs;
        //
        if is_permission_delete {
            // Access collection.
            let coll: Collection = client_cache
                .database(meta.database_name.as_str())
                .collection(meta.collection_name.as_str());
            // Execute query.
            Self::to_model_instance::<T>(
                coll.find_one_and_delete(filter, options)?,
                &meta.ignore_fields,
                &meta.map_widget_type,
                meta.model_name.as_str(),
            )
        } else {
            Err("It is forbidden to perform delete.".to_string())?
        }
    }

    /// Gets the name of the Collection.
    /// https://docs.rs/mongodb/1.1.1/mongodb/struct.Collection.html#method.name
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let name  = UserProfile::name()?;
    /// println!("{}", name);
    /// ```
    ///
    fn name() -> Result<String, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
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
    /// let name  = UserProfile::namespace()?;
    /// println!("{:?}", name);
    /// ```
    ///
    fn namespace() -> Result<Namespace, Box<dyn Error>> {
        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        let meta: Meta = form_cache.meta;
        // Access collection.
        let coll: Collection = client_cache
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        // Execute query.
        Ok(coll.namespace())
    }
}
