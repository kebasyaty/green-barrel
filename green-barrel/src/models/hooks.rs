//! Methods that are called at different stages when accessing the database.

use crate::models::Main;

/// Hooks methods.
pub trait Hooks: Main {
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
    fn pre_create(&self) {
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
    fn post_create(&self) {
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
    fn pre_update(&self) {
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
    fn post_update(&self) {
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
    fn pre_delete(&self) {
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
    fn post_delete(&self) {
        //
    }
}
