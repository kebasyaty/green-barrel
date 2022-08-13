//! For describe a model with user defaults.

/// For describe a model with user defaults.
///
/// # Example:
///
/// ```
/// #[Model]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     title: InputText,
///     slug: autoSlug,
/// }
///
/// impl Creator for ModelName {
///     fn custom_default() -> Self {
///         Self {
///             title: InputText {
///                 label: String::from("Title"),
///                 required: true,
///                 unique: true,
///                 ..Default::default()
///             },
///             ..Default::default()
///         }
///     }
/// }
/// ```
///
pub trait Creator {
    fn custom_default() -> Self;
}
