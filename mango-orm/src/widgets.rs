//! # Fields
//!
//! Field types for models.

pub use widgets::*;

// WIDGETS =========================================================================================
pub mod widgets {

    /// Widget for choices items
    /// Use for:
    /// <select></select>
    #[derive(Default, Debug)]
    pub struct SelectionWidget {
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

    impl SelectionWidget {
        // Get attributes
        pub fn attrs(&self) -> Self {
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
    // Standard widgets ----------------------------------------------------------------------------
    /// Use for:
    /// <input type="checkbox">
    /// <input type="color">
    /// <input type="date">
    /// <input type="email">
    /// <input type="file">
    /// <input type="hidden">
    /// <input type="image">
    /// <input type="number">
    /// <input type="password">
    /// <input type="radio">
    /// <input type="range">
    /// <input type="tel">
    /// <input type="text">
    /// <input type="time">
    /// <input type="url">
    #[derive(Default, Debug)]
    pub struct StandardWidget {
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

    impl StandardWidget {
        // Get attributes
        pub fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                default: self.default.clone(),
                readonly: self.readonly.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                unique: self.unique.clone(),
                hidden: self.hidden.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
            }
        }
    }
    // widgets  for relationship fields ------------------------------------------------------------
    #[derive(Debug, Clone)]
    pub enum RelationshipType {
        ForeignKey,
        ManyToMany,
        OneToOne,
    }

    impl Default for RelationshipType {
        fn default() -> Self {
            RelationshipType::ForeignKey
        }
    }

    /// Use for:
    /// <select></select>
    /// <select multiple></select> for ManyToMany type
    #[derive(Default, Debug)]
    pub struct RelationWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub relationship_type: RelationshipType, // Default = ForeignKey
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
        pub other_attrs: String,   // "autofocus ..."
        pub other_classes: String, // "class-name class-name ..."
    }

    impl RelationWidget {
        // Get attributes
        pub fn attrs(&self) -> Self {
            Self {
                id: self.id.clone(),
                label: self.label.clone(),
                relationship_type: self.relationship_type.clone(),
                readonly: self.readonly.clone(),
                required: self.required.clone(),
                hint: self.hint.clone(),
                hidden: self.hidden.clone(),
                other_attrs: self.other_attrs.clone(),
                other_classes: self.other_classes.clone(),
            }
        }
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
