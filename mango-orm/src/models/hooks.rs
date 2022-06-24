//! Methods that are called at different stages when accessing the database.

use crate::models::db_query_api::paladins::QPaladins;

/// Hooks methods.
pub trait Hooks: QPaladins {
    /// Вызывается перед созданием нового документа в базе данных.
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
    ///     fn pre_save(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    fn pre_save(&self) {
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
    ///     fn post_save(&self) {
    ///         Some code ...
    ///    }
    /// }
    /// ```
    ///
    fn post_save(&self) {
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
