//! # widgets
//!
//! Widgets for Forms.

// WIDGETS =========================================================================================
/// Mediator for transporting widget attributes
#[derive(Default, Debug)]
pub struct Transport {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub field_type: String,
    pub field_name: String,
    pub value: String,
    pub maxlength: u32,
    pub required: bool,
    pub readonly: bool, // For <input type="...">
    pub disabled: bool, // For <select></select>
    pub multiple: bool, // For <select></select>
    pub checked: bool,  // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,   // "autofocus ..."
    pub other_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, String)>,
}
/// Field Types ------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum FieldType {
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
    Select,
    ForeignKey,
    ManyToMany,
    OneToOne,
}
impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text
    }
}
impl FieldType {
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
            Self::Select => "select".to_string(),
            Self::ForeignKey => "m2o".to_string(),
            Self::ManyToMany => "m2m".to_string(),
            Self::OneToOne => "o2o".to_string(),
        }
    }
}

/// Data types for the `value` attribute -----------------------------------------------------------
#[derive(Debug, Clone)]
pub enum DataType {
    Text(&'static str),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}
impl Default for DataType {
    fn default() -> Self {
        DataType::Text("")
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

// Attributes for the widget -----------------------------------------------------------------------
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
/// <select></select>
/// <textarea></textarea>
#[derive(Default, Debug)]
pub struct Widget {
    pub id: &'static str, // "id-name" or auto
    pub label: &'static str,
    pub field_type: FieldType,
    pub value: DataType,
    pub maxlength: u32,
    pub required: bool,
    pub readonly: bool, // For <input type="...">
    pub disabled: bool, // For <select></select>
    pub multiple: bool, // For <select></select>
    pub checked: bool,  // For <input type="checkbox|radio">
    pub hint: &'static str,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: &'static str,   // "autofocus ..."
    pub other_classes: &'static str, // "class-name class-name ..."
    pub select: Vec<(&'static str, DataType)>,
}

impl Widget {
    // Get attributes of a widget
    pub fn get_attrs(&self, field_name: &str) -> Transport {
        Transport {
            id: self.id.to_string(),
            label: self.label.to_string(),
            field_type: self.field_type.get_type(),
            field_name: field_name.to_string(),
            value: self.value.get_data(),
            maxlength: self.maxlength.clone(),
            required: self.required.clone(),
            readonly: self.readonly.clone(),
            disabled: self.disabled.clone(),
            multiple: self.multiple.clone(),
            checked: self.checked.clone(),
            hint: self.hint.to_string(),
            unique: self.unique.clone(),
            hidden: self.hidden.clone(),
            other_attrs: self.other_attrs.to_string(),
            other_classes: self.other_classes.to_string(),
            select: self
                .select
                .iter()
                .map(|item| (item.0.to_string(), item.1.get_data()))
                .collect::<Vec<(String, String)>>(),
            ..Default::default()
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Standard type -------------------------------------------------------------------------------
    #[test]
    fn test_standard_type() {
        assert_eq!(FieldType::CheckBox.get_type(), "checkbox".to_string());
        assert_eq!(FieldType::Color.get_type(), "color".to_string());
        assert_eq!(FieldType::Date.get_type(), "date".to_string());
        assert_eq!(FieldType::Email.get_type(), "email".to_string());
        assert_eq!(FieldType::Hidden.get_type(), "hidden".to_string());
        assert_eq!(FieldType::Image.get_type(), "image".to_string());
        assert_eq!(FieldType::Number.get_type(), "number".to_string());
        assert_eq!(FieldType::Password.get_type(), "password".to_string());
        assert_eq!(FieldType::Radio.get_type(), "radio".to_string());
        assert_eq!(FieldType::Range.get_type(), "range".to_string());
        assert_eq!(FieldType::Tel.get_type(), "tel".to_string());
        assert_eq!(FieldType::Text.get_type(), "text".to_string());
        assert_eq!(FieldType::Time.get_type(), "time".to_string());
        assert_eq!(FieldType::Url.get_type(), "url".to_string());
        assert_eq!(FieldType::TextArea.get_type(), "textarea".to_string());
        assert_eq!(FieldType::Select.get_type(), "select".to_string());
        assert_eq!(FieldType::ForeignKey.get_type(), "m2o".to_string());
        assert_eq!(FieldType::ManyToMany.get_type(), "m2m".to_string());
        assert_eq!(FieldType::OneToOne.get_type(), "o2o".to_string());
    }

    // Default data type ---------------------------------------------------------------------------
    #[test]
    fn test_default_data_type() {
        assert_eq!(
            DataType::Text("Some text").get_data(),
            "Some text".to_string()
        );
        assert_eq!(DataType::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(DataType::U64(10_u64).get_data(), 10_u64.to_string());
        assert_eq!(DataType::F64(10_f64).get_data(), 10_f64.to_string());
        assert_eq!(DataType::Bool(true).get_data(), true.to_string());
    }

    // Widget --------------------------------------------------------------------------------------
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![("", DataType::Text(""))];
        // Fields
        assert_eq!(widget.id, "");
        assert_eq!(widget.label, "");
        assert_eq!(widget.field_type.get_type(), FieldType::Text.get_type());
        assert_eq!(widget.value.get_data(), DataType::Text("").get_data());
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, false);
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.disabled, false);
        assert_eq!(widget.multiple, false);
        assert_eq!(widget.checked, false);
        assert_eq!(widget.hint, "");
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "");
        assert_eq!(widget.other_classes, "");
        assert_eq!(widget.select[0].0, "");
        assert_eq!(widget.select[0].1.get_data(), String::new());
        // Methods
        let mut attrs = widget.get_attrs("some_name");
        attrs.select = vec![(String::new(), DataType::Text("").get_data())];

        assert_eq!(attrs.id, String::new());
        assert_eq!(attrs.label, String::new());
        assert_eq!(attrs.field_type, "text".to_string());
        assert_eq!(attrs.field_name, "some_name".to_string());
        assert_eq!(attrs.value, String::new());
        assert_eq!(attrs.maxlength, 0);
        assert_eq!(attrs.required, false);
        assert_eq!(attrs.readonly, false);
        assert_eq!(attrs.disabled, false);
        assert_eq!(attrs.multiple, false);
        assert_eq!(attrs.checked, false);
        assert_eq!(attrs.hint, String::new());
        assert_eq!(attrs.unique, false);
        assert_eq!(attrs.hidden, false);
        assert_eq!(attrs.other_attrs, String::new());
        assert_eq!(attrs.other_classes, String::new());
        assert_eq!(attrs.select[0].0, String::new());
        assert_eq!(attrs.select[0].1, String::new());
    }

    #[test]
    fn test_transport() {
        let trans: Transport = Default::default();
        // Fields
        assert_eq!(trans.id, String::new());
        assert_eq!(trans.label, String::new());
        assert_eq!(trans.field_type, String::new());
        assert_eq!(trans.field_name, String::new());
        assert_eq!(trans.value, String::new());
        assert_eq!(trans.maxlength, 0);
        assert_eq!(trans.required, false);
        assert_eq!(trans.readonly, false);
        assert_eq!(trans.disabled, false);
        assert_eq!(trans.multiple, false);
        assert_eq!(trans.checked, false);
        assert_eq!(trans.hint, String::new());
        assert_eq!(trans.unique, false);
        assert_eq!(trans.hidden, false);
        assert_eq!(trans.other_attrs, String::new());
        assert_eq!(trans.other_classes, String::new());
        assert_eq!(trans.select, vec![]);
        // Methods
    }
}
