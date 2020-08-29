//! # widgets
//!
//! Widgets for Forms.

pub use widgets::*;

// WIDGETS =========================================================================================
pub mod widgets {

    // Standard widgets ----------------------------------------------------------------------------
    /// Enumeration for standard types
    #[derive(Debug, Clone)]
    pub enum StandardType {
        CheckBox,
        Color,
        Date,
        Email,
        File,
        Hidden,
        Image,
        Number,
        Password,
        Radio,
        Range,
        Tel,
        Text,
        Time,
        Url,
    }

    impl Default for StandardType {
        fn default() -> Self {
            StandardType::Text
        }
    }

    /// For standard widgets
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
        pub input_type: StandardType,
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
                input_type: self.input_type.clone(),
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
    // Widget for choices items --------------------------------------------------------------------
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
    // Widget  for relation fields -----------------------------------------------------------------
    /// Enumeration of relationship types
    #[derive(Debug, Clone)]
    pub enum RelationType {
        ForeignKey,
        ManyToMany,
        OneToOne,
    }

    impl Default for RelationType {
        fn default() -> Self {
            RelationType::ForeignKey
        }
    }

    /// Widget for relation fields
    /// Use for:
    /// <select></select>
    /// <select multiple></select> for ManyToMany type
    #[derive(Default, Debug)]
    pub struct RelationWidget {
        pub id: String, // "id-name" or auto
        pub label: String,
        pub relation_type: RelationType, // Default = ForeignKey
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
                relation_type: self.relation_type.clone(),
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

    // Standard widgets ----------------------------------------------------------------------------
    #[test]
    fn test_boolean_widget() {
        let widget: StandardWidget = Default::default();
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(widget.default, false);
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
    }

    // Widget for choices items --------------------------------------------------------------------
    #[test]
    fn test_select_string_widget() {
        let widget: SelectionWidget = Default::default();
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(widget.default, "".to_string());
        assert_eq!(widget.disabled, false);
        assert_eq!(widget.multiple, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
        assert_eq!(widget.select, vec![]);
    }

    // Widget for relation fields ------------------------------------------------------------------
    #[test]
    fn test_foreign_key_widget() {
        let widget: RelationWidget = Default::default();
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
    }
}
