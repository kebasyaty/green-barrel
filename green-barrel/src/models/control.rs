//! For customizing fields controls.

/// For customizing fields controls.
///
/// # Example:
///
/// ```
/// #[Model]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     title: Text,
///     slug: Slug,
/// }
///
/// impl Control for ModelName {
///     fn custom_default() -> Self {
///         Self {
///             title: Text {
///                 label: String::from("Title"),
///                 maxlength: 60,
///                 required: true,
///                 unique: true,
///                 ..Default::default()
///             },
///             ..Default::default() // For `hash`, `created_at`, `updated_at` etc
///         }
///     }
/// }
/// ```
///
pub trait Control {
    fn custom_default() -> Self;
}
