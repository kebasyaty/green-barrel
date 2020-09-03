//! # Widgets
//!
//! Widgets for form elements.

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
/// Field types for Widgets ------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum WidgetFields {
    InputCheckBox,
    InputColor,
    InputDate,
    InputEmail,
    InputFile,
    InputImage,
    InputNumber,
    InputPassword,
    InputRadio,
    InputRange,
    InputTel,
    InputText,
    InputTime,
    InputUrl,
    TextArea,
    Select,
    ForeignKey,
    ManyToMany,
    OneToOne,
}
impl Default for WidgetFields {
    fn default() -> Self {
        WidgetFields::InputText
    }
}
impl WidgetFields {
    pub fn get_type(&self) -> String {
        match self {
            Self::InputCheckBox => "checkbox".to_string(),
            Self::InputColor => "color".to_string(),
            Self::InputDate => "date".to_string(),
            Self::InputEmail => "email".to_string(),
            Self::InputFile => "file".to_string(),
            Self::InputImage => "image".to_string(),
            Self::InputNumber => "number".to_string(),
            Self::InputPassword => "password".to_string(),
            Self::InputRadio => "radio".to_string(),
            Self::InputRange => "range".to_string(),
            Self::InputTel => "tel".to_string(),
            Self::InputText => "text".to_string(),
            Self::InputTime => "time".to_string(),
            Self::InputUrl => "url".to_string(),
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

/// Attributes for the widget ----------------------------------------------------------------------
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
    pub label: String,
    pub field_type: WidgetFields,
    pub value: DataType,
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
    pub select: Vec<(String, DataType)>,
}

impl Widget {
    // Get pure attributes from a widget
    pub fn get_clean_attrs(&self, field_name: &str) -> Transport {
        Transport {
            id: field_name.to_string(),
            label: self.label.clone(),
            field_type: self.field_type.get_type(),
            field_name: field_name.to_string(),
            value: self.value.get_data(),
            maxlength: self.maxlength.clone(),
            required: self.required.clone(),
            readonly: self.readonly.clone(),
            disabled: self.disabled.clone(),
            multiple: self.multiple.clone(),
            checked: self.checked.clone(),
            hint: self.hint.clone(),
            unique: self.unique.clone(),
            hidden: self.hidden.clone(),
            other_attrs: self.other_attrs.clone(),
            other_classes: self.other_classes.clone(),
            select: self
                .select
                .iter()
                .map(|item| (item.0.clone(), item.1.get_data()))
                .collect::<Vec<(String, String)>>(),
            ..Default::default()
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Testing Transport structure -----------------------------------------------------------------
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

    // Testing field types for Widget --------------------------------------------------------------
    #[test]
    fn test_standard_type() {
        assert_eq!(
            WidgetFields::InputCheckBox.get_type(),
            "checkbox".to_string()
        );
        assert_eq!(WidgetFields::InputColor.get_type(), "color".to_string());
        assert_eq!(WidgetFields::InputDate.get_type(), "date".to_string());
        assert_eq!(WidgetFields::InputEmail.get_type(), "email".to_string());
        assert_eq!(WidgetFields::InputImage.get_type(), "image".to_string());
        assert_eq!(WidgetFields::InputNumber.get_type(), "number".to_string());
        assert_eq!(
            WidgetFields::InputPassword.get_type(),
            "password".to_string()
        );
        assert_eq!(WidgetFields::InputRadio.get_type(), "radio".to_string());
        assert_eq!(WidgetFields::InputRange.get_type(), "range".to_string());
        assert_eq!(WidgetFields::InputTel.get_type(), "tel".to_string());
        assert_eq!(WidgetFields::InputText.get_type(), "text".to_string());
        assert_eq!(WidgetFields::InputTime.get_type(), "time".to_string());
        assert_eq!(WidgetFields::InputUrl.get_type(), "url".to_string());
        assert_eq!(WidgetFields::TextArea.get_type(), "textarea".to_string());
        assert_eq!(WidgetFields::Select.get_type(), "select".to_string());
        assert_eq!(WidgetFields::ForeignKey.get_type(), "m2o".to_string());
        assert_eq!(WidgetFields::ManyToMany.get_type(), "m2m".to_string());
        assert_eq!(WidgetFields::OneToOne.get_type(), "o2o".to_string());
    }

    // Testing Data types --------------------------------------------------------------------------
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

    // Testing Widget structure --------------------------------------------------------------------
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), DataType::Text(""))];
        // Fields
        assert_eq!(widget.label, String::new());
        assert_eq!(
            widget.field_type.get_type(),
            WidgetFields::InputText.get_type()
        );
        assert_eq!(widget.value.get_data(), DataType::Text("").get_data());
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, false);
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.disabled, false);
        assert_eq!(widget.multiple, false);
        assert_eq!(widget.checked, false);
        assert_eq!(widget.hint, String::new());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, String::new());
        assert_eq!(widget.other_classes, String::new());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(widget.select[0].1.get_data(), String::new());
        // Methods
        let mut attrs = widget.get_clean_attrs("");
        attrs.select = vec![(String::new(), DataType::Text("").get_data())];

        assert_eq!(attrs.id, String::new());
        assert_eq!(attrs.label, String::new());
        assert_eq!(attrs.field_type, "text".to_string());
        assert_eq!(attrs.field_name, String::new());
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
}
