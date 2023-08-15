//! For customizing field types.

/// For customizing field types.
///
/// # Example:
///
/// ```
/// #[Model]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     title: TextField,
///     slug: SlugField,
/// }
///
/// impl Control for ModelName {
///     fn custom() -> Self {
///         Self {
///             title: TextField {
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
    fn custom() -> Self;
}
