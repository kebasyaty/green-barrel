//! Methods that are called at different stages when accessing the database.

use async_trait::async_trait;
use mongodb::Client;

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
    async fn pre_create(&self, _client: &Client) {
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
    async fn post_create(&self, _client: &Client) {
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
    async fn pre_update(&self, _client: &Client) {
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
    async fn post_update(&self, _client: &Client) {
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
    async fn pre_delete(&self, _client: &Client) {
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
    async fn post_delete(&self, _client: &Client) {
        //
    }
}
