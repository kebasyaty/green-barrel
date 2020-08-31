//! # widgets
//!
//! Widgets for Forms.

// WIDGETS =========================================================================================
// Standard widgets --------------------------------------------------------------------------------
/// Standard Form Element Types
#[derive(Debug, Clone)]
pub enum InputType {
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
    TextArea,
}
impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}
impl InputType {
    pub fn get_type(&self) -> String {
        match self {
            Self::CheckBox => "checkbox".to_string(),
            Self::Color => "color".to_string(),
            Self::Date => "date".to_string(),
            Self::Email => "email".to_string(),
            Self::File => "file".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::Image => "image".to_string(),
            Self::Number => "number".to_string(),
            Self::Password => "password".to_string(),
            Self::Radio => "radio".to_string(),
            Self::Range => "range".to_string(),
            Self::Tel => "tel".to_string(),
            Self::Text => "text".to_string(),
            Self::Time => "time".to_string(),
            Self::Url => "url".to_string(),
            Self::TextArea => "textarea".to_string(),
        }
    }
}

/// Data types for the `value` field
#[derive(Debug, Clone)]
pub enum DataType {
    Text(String),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}
impl Default for DataType {
    fn default() -> Self {
        DataType::Text(String::new())
    }
}
impl DataType {
    pub fn get_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_string(),
            Self::I64(data) => data.to_string(),
            Self::U64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
            Self::Bool(data) => data.to_string(),
        }
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
/// <textarea></textarea>
#[derive(Default, Debug)]
pub struct StandardWidget {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub input_type: InputType,
    pub value: DataType,
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,   // "autofocus ..."
    pub other_classes: String, // "class-name class-name ..."
}

impl StandardWidget {
    // Get attributes of a widget
    pub fn get_attrs(&self) -> String {
        String::new()
    }
}

// Widget for choices items ------------------------------------------------------------------------
/// Data types for the `value` field
#[derive(Debug, Clone)]
pub enum DataType2 {
    Text(String),
    I64(i64),
    U64(u64),
    F64(f64),
}
impl Default for DataType2 {
    fn default() -> Self {
        DataType2::Text(String::new())
    }
}
impl DataType2 {
    pub fn get_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_string(),
            Self::I64(data) => data.to_string(),
            Self::U64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
        }
    }
}

/// Widget for choices items
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct SelectionWidget {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub value: DataType2,
    pub disabled: bool,
    pub multiple: bool,
    pub required: bool,
    pub hint: String,
    pub other_attrs: String,   // "autofocus size='3'"
    pub other_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, DataType2)>,
}

impl SelectionWidget {
    // Get attributes of a widget
    pub fn get_attrs(&self) -> String {
        String::new()
    }
}

// Widget  for relation fields ---------------------------------------------------------------------
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
impl RelationType {
    pub fn get_token(&self) -> String {
        match self {
            Self::ForeignKey => "m2o".to_string(),
            Self::ManyToMany => "m2m".to_string(),
            Self::OneToOne => "o2o".to_string(),
        }
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
    // Get attributes of a widget
    pub fn get_attrs(&self) -> String {
        String::new()
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Standard type -------------------------------------------------------------------------------
    #[test]
    fn test_standard_type() {
        assert_eq!(InputType::CheckBox.get_type(), "checkbox".to_string());
        assert_eq!(InputType::Color.get_type(), "color".to_string());
        assert_eq!(InputType::Date.get_type(), "date".to_string());
        assert_eq!(InputType::Email.get_type(), "email".to_string());
        assert_eq!(InputType::Hidden.get_type(), "hidden".to_string());
        assert_eq!(InputType::Image.get_type(), "image".to_string());
        assert_eq!(InputType::Number.get_type(), "number".to_string());
        assert_eq!(InputType::Password.get_type(), "password".to_string());
        assert_eq!(InputType::Radio.get_type(), "radio".to_string());
        assert_eq!(InputType::Range.get_type(), "range".to_string());
        assert_eq!(InputType::Tel.get_type(), "tel".to_string());
        assert_eq!(InputType::Text.get_type(), "text".to_string());
        assert_eq!(InputType::Time.get_type(), "time".to_string());
        assert_eq!(InputType::Url.get_type(), "url".to_string());
        assert_eq!(InputType::TextArea.get_type(), "textarea".to_string());
    }

    // Default data type ---------------------------------------------------------------------------
    #[test]
    fn test_default_data_type() {
        assert_eq!(
            DataType::Text("Some text".to_string()).get_data(),
            "Some text".to_string()
        );
        assert_eq!(DataType::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(DataType::U64(10_u64).get_data(), 10_u64.to_string());
        assert_eq!(DataType::F64(10_f64).get_data(), 10_f64.to_string());
        assert_eq!(DataType::Bool(true).get_data(), true.to_string());
    }

    #[test]
    fn test_default_data_type_2() {
        assert_eq!(
            DataType2::Text("Some text".to_string()).get_data(),
            "Some text".to_string()
        );
        assert_eq!(DataType2::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(DataType2::U64(10_u64).get_data(), 10_u64.to_string());
        assert_eq!(DataType2::F64(10_f64).get_data(), 10_f64.to_string());
    }

    // Standard widgets ----------------------------------------------------------------------------
    #[test]
    fn test_standard_widget() {
        let widget: StandardWidget = Default::default();
        // Fields
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(widget.input_type.get_type(), InputType::Text.get_type());
        assert_eq!(
            widget.value.get_data(),
            DataType::Text(String::new()).get_data()
        );
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
        // Methods
    }

    // Widget for choices items --------------------------------------------------------------------
    #[test]
    fn test_select_widget() {
        let mut widget: SelectionWidget = Default::default();
        widget.select = vec![(String::new(), DataType2::Text(String::new()))];
        // Fields
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(
            widget.value.get_data(),
            DataType2::Text(String::new()).get_data()
        );
        assert_eq!(widget.disabled, false);
        assert_eq!(widget.multiple, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(
            widget.select[0].1.get_data(),
            DataType2::Text(String::new()).get_data()
        );
        // Methods
    }

    // Relation type -------------------------------------------------------------------------------
    #[test]
    fn test_relation_type() {
        assert_eq!(RelationType::ForeignKey.get_token(), "m2o".to_string());
        assert_eq!(RelationType::ManyToMany.get_token(), "m2m".to_string());
        assert_eq!(RelationType::OneToOne.get_token(), "o2o".to_string());
    }

    // Widget for relation fields ------------------------------------------------------------------
    #[test]
    fn test_relation_widget() {
        let widget: RelationWidget = Default::default();
        // Fields
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(
            widget.relation_type.get_token(),
            RelationType::ForeignKey.get_token()
        );
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
        // Methods
    }
}
