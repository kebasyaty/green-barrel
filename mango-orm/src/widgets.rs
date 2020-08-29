//! # Fields
//!
//! Field types for models.

pub use abstractions::*;
pub use relationship_widgets::*;
pub use selection_widgets::*;
pub use standard_widgets::*;

// ABSTRACTIONS ====================================================================================
pub mod abstractions {
    /// Abstract widget
    pub trait Widget<T> {
        fn attrs(&self) -> T;
    }
}

// WIDGETS FOR CHOICES ITEMS =======================================================================
pub mod selection_widgets {
    use super::abstractions::Widget;

    /// Select StrStr type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct SelectStrStrWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String,
        pub disabled: bool,
        pub multiple: bool,
        pub required: bool,
        pub hint: String,
        pub other_attrs: String,   // "autofocus size='3'"
        pub other_classes: String, // "class-name class-name ..."
        pub select: Vec<(String, String)>,
    }

    impl Widget<Self> for SelectStrStrWidget {
        // Get widget attributes
        fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                disabled: self.disabled.clone(),
                multiple: self.multiple.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
                select: self.select.clone(),
            }
        }
    }

    /// Select StrI64 type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct SelectStrI64Widget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: i64,
        pub disabled: bool,
        pub multiple: bool,
        pub required: bool,
        pub hint: String,
        pub other_attrs: String,   // "autofocus size='3'"
        pub other_classes: String, // "class-name class-name ..."
        pub select: Vec<(String, i64)>,
    }

    impl Widget<Self> for SelectStrI64Widget {
        // Get widget attributes
        fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                disabled: self.disabled.clone(),
                multiple: self.multiple.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
                select: self.select.clone(),
            }
        }
    }

    /// Select StrU64 type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct SelectStrU64Widget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: u64,
        pub disabled: bool,
        pub multiple: bool,
        pub required: bool,
        pub hint: String,
        pub other_attrs: String,   // "autofocus size='3'"
        pub other_classes: String, // "class-name class-name ..."
        pub select: Vec<(String, u64)>,
    }

    impl Widget<Self> for SelectStrU64Widget {
        // Get widget attributes
        fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                disabled: self.disabled.clone(),
                multiple: self.multiple.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
                select: self.select.clone(),
            }
        }
    }

    /// Select StrF64 type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct SelectStrF64Widget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: f64,
        pub disabled: bool,
        pub multiple: bool,
        pub required: bool,
        pub hint: String,
        pub other_attrs: String,   // "autofocus size='3'"
        pub other_classes: String, // "class-name class-name ..."
        pub select: Vec<(String, f64)>,
    }

    impl Widget<Self> for SelectStrF64Widget {
        // Get widget attributes
        fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                disabled: self.disabled.clone(),
                multiple: self.multiple.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
                select: self.select.clone(),
            }
        }
    }
}

// STANDARD WIDGETS ================================================================================
pub mod standard_widgets {
    use super::abstractions::Widget;

    /// Boolean type Widget
    /// Use for:
    /// <input type="checkbox">
    /// <input type="radio">
    #[derive(Default, Debug)]
    pub struct BooleanWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: bool, // true or false
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }

    impl Widget<Self> for BooleanWidget {
        // Get widget attributes
        fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                readonly: self.readonly.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                hidden: self.hidden.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
            }
        }
    }

    /// Color type Widget --------------------------------------------------------------------------
    /// Use for:
    /// <input type="color">
    /// <input type="text">
    #[derive(Default, Debug)]
    pub struct ColorWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // example: "#ffffff" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Date type Widget
    /// Use for:
    /// <input type="date">
    /// <input type="text">
    #[derive(Default, Debug)]
    pub struct DateWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // Date in UNIX format "0000-00-00" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Email type Widget
    /// Use for:
    /// <input type="email">
    #[derive(Default, Debug)]
    pub struct EmailWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // email address or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// File type Widget
    /// Use for:
    /// <input type="file">
    #[derive(Default, Debug)]
    pub struct FileWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Float type Widget
    /// Use for:
    /// <input type="number">
    #[derive(Default, Debug)]
    pub struct FloatWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: f64, // number 0.0_f64
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Image type Widget
    /// Use for:
    /// <input type="file">
    #[derive(Default, Debug)]
    pub struct ImageWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Integer type Widget
    /// Use for:
    /// <input type="number">
    #[derive(Default, Debug)]
    pub struct IntegerWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: i64, // number 0_i64
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// IPAddress type Widget
    /// Use for:
    /// 1. <input type="text">
    /// 2. <input type="text" size="16"
    /// pattern="^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$">
    /// 3. https://stackoverflow.com/questions/49306970/correct-input-type-for-ip-address
    #[derive(Default, Debug)]
    pub struct IPAddressWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // IP or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Positive Integer type Widget
    /// Use for:
    /// <input type="number">
    #[derive(Default, Debug)]
    pub struct PositiveIntegerWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: u64, // number 0_u64
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Slug type Widget
    /// Use for:
    /// <input type="text">
    #[derive(Default, Debug)]
    pub struct SlugWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // slug-text or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Text type Widget
    /// Use for:
    /// <input type="text">
    #[derive(Default, Debug)]
    pub struct TextWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // some text or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// TextArea type Widget
    /// Use for:
    /// <textarea></textarea>
    #[derive(Default, Debug)]
    pub struct TextAreaWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // some text or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Time type Widget
    /// Use for:
    /// <input type="time">
    #[derive(Default, Debug)]
    pub struct TimeWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // date in UNIX format "00:00:00" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// URL type Widget
    /// Use for:
    /// <input type="url">
    #[derive(Default, Debug)]
    pub struct URLWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // URL or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Password type Widget
    /// Use for:
    /// <input type="password">
    #[derive(Default, Debug)]
    pub struct PasswordWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, // password text or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// Phone type Widget
    /// Use for:
    /// <input type="tel">
    #[derive(Default, Debug)]
    pub struct PhoneWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub default: String, //  phone number or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
}

// WIDGETS  FOR RELATIONSHIP FIELDS ================================================================
pub mod relationship_widgets {
    /// ForeignKey type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct ForeignKeyWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// ManyToMany type Widget
    /// Use for:
    /// <select multiple></select>
    #[derive(Default, Debug)]
    pub struct ManyToManyWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
    /// OneToOne type Widget
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct OneToOneWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // FIELDS FOR CHOICES ITEMS --------------------------------------------------------------------
    #[test]
    fn test_select_string_widget() {
        let field: SelectStrStrWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.disabled, false);
        assert_eq!(field.multiple, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
        assert_eq!(field.select, vec![]);
    }
    #[test]
    fn test_select_integer_widget() {
        let field: SelectStrI64Widget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_i64);
        assert_eq!(field.disabled, false);
        assert_eq!(field.multiple, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
        assert_eq!(field.select, vec![]);
    }
    #[test]
    fn test_select_positive_integer_widget() {
        let field: SelectStrU64Widget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_u64);
        assert_eq!(field.disabled, false);
        assert_eq!(field.multiple, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
        assert_eq!(field.select, vec![]);
    }
    #[test]
    fn test_select_float_widget() {
        let field: SelectStrF64Widget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_f64);
        assert_eq!(field.disabled, false);
        assert_eq!(field.multiple, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
        assert_eq!(field.select, vec![]);
    }

    // STANDARD FIELDS -----------------------------------------------------------------------------
    #[test]
    fn test_boolean_field() {
        let field: BooleanWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, false);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_color_field() {
        let field: ColorWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_date_field() {
        let field: DateWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_email_field() {
        let field: EmailWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_file_field() {
        let field: FileWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_float_field() {
        let field: FloatWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0.0_f64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_image_field() {
        let field: ImageWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_integer_field() {
        let field: IntegerWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_i64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_ip_address_field() {
        let field: IPAddressWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_positive_integer_field() {
        let field: PositiveIntegerWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_u64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_slug_field() {
        let field: SlugWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_text_field() {
        let field: TextWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_text_area_field() {
        let field: TextAreaWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_time_field() {
        let field: TimeWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_url_field() {
        let field: URLWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_password_field() {
        let field: PasswordWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_phone_field() {
        let field: PhoneWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    // RELATIONSHIP FIELDS -------------------------------------------------------------------------
    #[test]
    fn test_foreign_key_field() {
        let field: ForeignKeyWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_many_to_many_field() {
        let field: ManyToManyWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }

    #[test]
    fn test_one_to_one_field() {
        let field: OneToOneWidget = Default::default();
        assert_eq!(field.id, "".to_string());
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
        assert_eq!(field.other_attrs, "".to_string());
        assert_eq!(field.other_classes, "".to_string());
    }
}
