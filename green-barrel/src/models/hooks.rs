//! Methods that are called at different stages when accessing the database.

use async_lock::RwLock;
use async_trait::async_trait;
use mongodb::Client;
use std::collections::HashMap;
use std::sync::Arc;

use crate::models::helpers::Meta;

/// Hooks methods.
#[async_trait(?Send)]
pub trait Hooks {
    /// Called before a new document is created in the database.
    ///
    /// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn pre_create(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn pre_create(
        &self,
        _meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        _client: &Client,
        _media_dir: &HashMap<String, String>,
    ) {
        //
    }
    /// Called after a new document has been created in the database.
    ///
    //// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn post_create(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn post_create(
        &self,
        _meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        _client: &Client,
        _media_dir: &HashMap<String, String>,
    ) {
        //
    }
    /// Called before updating an existing document in the database.
    ///
    //// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn pre_update(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn pre_update(
        &self,
        _meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        _client: &Client,
        _media_dir: &HashMap<String, String>,
    ) {
        //
    }
    /// Called after an existing document in the database is updated.
    ///
    //// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn post_update(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn post_update(
        &self,
        _meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        _client: &Client,
        _media_dir: &HashMap<String, String>,
    ) {
        //
    }
    /// Called before deleting an existing document in the database.
    ///
    //// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn pre_delete(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn pre_delete(&self, _meta_store: &Arc<RwLock<HashMap<String, Meta>>>, _client: &Client) {
        //
    }
    /// Called after an existing document in the database has been deleted.
    ///
    //// # Example:
    ///
    /// ```
    /// #[Model(
    ///     is_use_hooks = true
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl Hooks for ModelName {
    ///     fn post_delete(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    async fn post_delete(
        &self,
        _meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        _client: &Client,
    ) {
        //
    }
}
