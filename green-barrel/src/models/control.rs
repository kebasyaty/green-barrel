//! For customizing fields controls.

/// For customizing fields controls.
///
/// # Example:
///
/// ```
/// #[Model]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     title: InputText,
///     slug: AutoSlug,
/// }
///
/// impl Control for ModelName {
///     fn custom_default() -> Self {
///         Self {
///             title: InputText {
///                 label: String::from("Title"),
///                 maxlength: 60,
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
pub trait Control {
    fn custom_default() -> Self;
}
