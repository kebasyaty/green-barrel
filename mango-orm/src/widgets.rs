//! # Widgets
//!
//! Widgets for form elements.

use serde::Serialize;

// WIDGETS =========================================================================================
/// Field types for Widgets
#[derive(Debug, Clone)]
pub enum FieldType {
    InputCheckBox(bool),
    InputColor(String),
    InputDate(String),
    InputDateTime(u32),
    InputEmail(String),
    InputFile(String),
    InputImage(String),
    InputNumberI32(i32),
    InputNumberU32(u32),
    InputNumberI64(i64),
    InputNumberF64(f64),
    InputPassword(String),
    InputRadio(bool),
    InputRangeI32(i32),
    InputRangeU32(u32),
    InputRangeI64(i64),
    InputRangeF64(f64),
    InputTel(String),
    InputText(String),
    InputUrl(String),
    TextArea(String),
    SelectI32(i32),
    SelectU32(u32),
    SelectI64(i64),
    SelectF64(f64),
    ForeignKey,
    ManyToMany,
    OneToOne,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::InputText(String::new())
    }
}

impl FieldType {
    pub fn input_type(&self) -> String {
        match self {
            Self::InputCheckBox(_) => "checkbox".to_string(),
            Self::InputColor(_) => "color".to_string(),
            Self::InputDate(_) => "date".to_string(),
            Self::InputDateTime(_) => "datetime".to_string(),
            Self::InputEmail(_) => "email".to_string(),
            Self::InputFile(_) => "file".to_string(),
            Self::InputImage(_) => "image".to_string(),
            Self::InputNumberI32(_) => "number".to_string(),
            Self::InputNumberU32(_) => "number".to_string(),
            Self::InputNumberI64(_) => "number".to_string(),
            Self::InputNumberF64(_) => "number".to_string(),
            Self::InputPassword(_) => "password".to_string(),
            Self::InputRadio(_) => "radio".to_string(),
            Self::InputRangeI32(_) => "range".to_string(),
            Self::InputRangeU32(_) => "range".to_string(),
            Self::InputRangeI64(_) => "range".to_string(),
            Self::InputRangeF64(_) => "range".to_string(),
            Self::InputTel(_) => "tel".to_string(),
            Self::InputText(_) => "text".to_string(),
            Self::InputUrl(_) => "url".to_string(),
            Self::TextArea(_) => "textarea".to_string(),
            Self::SelectI32(_) => "select".to_string(),
            Self::SelectU32(_) => "select".to_string(),
            Self::SelectI64(_) => "select".to_string(),
            Self::SelectF64(_) => "select".to_string(),
            Self::ForeignKey => "select".to_string(),
            Self::ManyToMany => "select".to_string(),
            Self::OneToOne => "hidden".to_string(),
        }
    }

    pub fn raw_data(&self) -> String {
        match self {
            Self::InputCheckBox(data) => data.to_string(),
            Self::InputColor(data) => data.to_string(),
            Self::InputDate(data) => data.to_string(),
            Self::InputDateTime(data) => data.to_string(),
            Self::InputEmail(data) => data.to_string(),
            Self::InputFile(data) => data.to_string(),
            Self::InputImage(data) => data.to_string(),
            Self::InputNumberI32(data) => data.to_string(),
            Self::InputNumberU32(data) => data.to_string(),
            Self::InputNumberI64(data) => data.to_string(),
            Self::InputNumberF64(data) => data.to_string(),
            Self::InputPassword(data) => data.to_string(),
            Self::InputRadio(data) => data.to_string(),
            Self::InputRangeI32(data) => data.to_string(),
            Self::InputRangeU32(data) => data.to_string(),
            Self::InputRangeI64(data) => data.to_string(),
            Self::InputRangeF64(data) => data.to_string(),
            Self::InputTel(data) => data.to_string(),
            Self::InputText(data) => data.to_string(),
            Self::InputUrl(data) => data.to_string(),
            Self::TextArea(data) => data.to_string(),
            Self::SelectI32(data) => data.to_string(),
            Self::SelectU32(data) => data.to_string(),
            Self::SelectI64(data) => data.to_string(),
            Self::SelectF64(data) => data.to_string(),
            Self::ForeignKey => String::new(),
            Self::ManyToMany => String::new(),
            Self::OneToOne => String::new(),
        }
    }

    pub fn data_type(&self) -> String {
        match self {
            Self::InputCheckBox(_) => "bool".to_string(),
            Self::InputColor(_) => "string".to_string(),
            Self::InputDate(_) => "string".to_string(),
            Self::InputDateTime(_) => "u32".to_string(),
            Self::InputEmail(_) => "string".to_string(),
            Self::InputFile(_) => "string".to_string(),
            Self::InputImage(_) => "string".to_string(),
            Self::InputNumberI32(_) => "i32".to_string(),
            Self::InputNumberU32(_) => "u32".to_string(),
            Self::InputNumberI64(_) => "i64".to_string(),
            Self::InputNumberF64(_) => "f64".to_string(),
            Self::InputPassword(_) => "string".to_string(),
            Self::InputRadio(_) => "bool".to_string(),
            Self::InputRangeI32(_) => "i32".to_string(),
            Self::InputRangeU32(_) => "u32".to_string(),
            Self::InputRangeI64(_) => "i64".to_string(),
            Self::InputRangeF64(_) => "f64".to_string(),
            Self::InputTel(_) => "string".to_string(),
            Self::InputText(_) => "string".to_string(),
            Self::InputUrl(_) => "string".to_string(),
            Self::TextArea(_) => "string".to_string(),
            Self::SelectI32(_) => "i32".to_string(),
            Self::SelectU32(_) => "u32".to_string(),
            Self::SelectI64(_) => "i64".to_string(),
            Self::SelectF64(_) => "f64".to_string(),
            Self::ForeignKey => String::new(),
            Self::ManyToMany => String::new(),
            Self::OneToOne => String::new(),
        }
    }
}

/// Data types for the `value` attribute -----------------------------------------------------------
#[derive(Debug, Clone, PartialEq)]
pub enum SelectDataType {
    Text(String),
    I32(i32),
    U32(u32),
    I64(i64),
    F64(f64),
}

impl Default for SelectDataType {
    fn default() -> Self {
        SelectDataType::Text(String::new())
    }
}

impl SelectDataType {
    pub fn raw_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_owned(),
            Self::I64(data) => data.to_string(),
            Self::I32(data) => data.to_string(),
            Self::U32(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
        }
    }

    pub fn data_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "Text",
            Self::I64(_) => "I64",
            Self::I32(_) => "I32",
            Self::U32(_) => "U32",
            Self::F64(_) => "F64",
        }
    }
}

/// Mediator for transporting widget attributes ----------------------------------------------------
#[derive(Serialize, Debug, Default)]
pub struct Transport {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub value: String,
    pub maxlength: u32,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,  // "autofocus step=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, String)>,
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
#[derive(Debug)]
pub struct Widget {
    pub label: String,
    pub relation_model: String,
    pub value: FieldType,
    pub maxlength: u32,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,  // "autofocus step=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, SelectDataType)>,
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            label: String::new(),
            relation_model: String::new(),
            value: FieldType::default(),
            maxlength: 0_u32,
            required: true,
            hint: String::new(),
            unique: false,
            hidden: false,
            other_attrs: String::new(),
            some_classes: String::new(),
            select: vec![],
        }
    }
}

impl Widget {
    // Get pure attributes from a widget
    pub fn get_clean_attrs(&self, name: &str) -> Transport {
        let field_type = match self.hidden {
            true => "hidden".to_string(),
            false => self.value.input_type(),
        };
        let checked = match self.value {
            FieldType::InputCheckBox(data) => data,
            FieldType::InputRadio(data) => data,
            _ => false,
        };
        let other_attrs = match self.value {
            FieldType::ManyToMany => match self.other_attrs.contains("multiple") {
                true => self.other_attrs.clone(),
                false => format!("multiple {}", self.other_attrs),
            },
            _ => self.other_attrs.clone(),
        };

        Transport {
            id: name.to_string(),
            label: self.label.clone(),
            field_type: field_type,
            name: name.to_string(),
            value: self.value.get_data(),
            maxlength: self.maxlength.clone(),
            required: self.required.clone(),
            checked: checked,
            hint: self.hint.clone(),
            unique: self.unique.clone(),
            hidden: self.hidden.clone(),
            other_attrs: other_attrs,
            some_classes: self.some_classes.clone(),
            select: self
                .select
                .iter()
                .map(|item| (item.0.clone(), item.1.get_data()))
                .collect::<Vec<(String, String)>>(),
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Testing field types for Widget --------------------------------------------------------------
    #[test]
    fn test_field_types() {
        assert_eq!(FieldType::InputCheckBox.get_type(), "checkbox".to_string());
        assert_eq!(FieldType::InputColor.get_type(), "color".to_string());
        assert_eq!(FieldType::InputDate.get_type(), "date".to_string());
        assert_eq!(FieldType::InputDateTime.get_type(), "datetime".to_string());
        assert_eq!(FieldType::InputEmail.get_type(), "email".to_string());
        assert_eq!(FieldType::InputImage.get_type(), "image".to_string());
        assert_eq!(FieldType::InputNumber.get_type(), "number".to_string());
        assert_eq!(FieldType::InputPassword.get_type(), "password".to_string());
        assert_eq!(FieldType::InputRadio.get_type(), "radio".to_string());
        assert_eq!(FieldType::InputRange.get_type(), "range".to_string());
        assert_eq!(FieldType::InputTel.get_type(), "tel".to_string());
        assert_eq!(FieldType::InputText.get_type(), "text".to_string());
        assert_eq!(FieldType::InputUrl.get_type(), "url".to_string());
        assert_eq!(FieldType::TextArea.get_type(), "textarea".to_string());
        assert_eq!(FieldType::Select.get_type(), "select".to_string());
        assert_eq!(FieldType::ForeignKey.get_type(), "select".to_string());
        assert_eq!(FieldType::ManyToMany.get_type(), "select".to_string());
        assert_eq!(FieldType::OneToOne.get_type(), "hidden".to_string());
    }

    // Testing Data types --------------------------------------------------------------------------
    #[test]
    fn test_data_types() {
        assert_eq!(
            DataType::Text("Some text".to_string()).get_data(),
            "Some text".to_string()
        );
        assert_eq!(DataType::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(DataType::I32(10_i32).get_data(), 10_i32.to_string());
        assert_eq!(DataType::U32(10_u32).get_data(), 10_u32.to_string());
        assert_eq!(DataType::F64(10_f64).get_data(), 10_f64.to_string());
        assert_eq!(DataType::Bool(true).get_data(), true.to_string());
        assert_eq!(DataType::None.get_data(), String::new());
    }

    // Testing Transport structure -----------------------------------------------------------------
    #[test]
    fn test_transport() {
        let trans: Transport = Default::default();
        // Fields
        assert_eq!(trans.id, String::new());
        assert_eq!(trans.label, String::new());
        assert_eq!(trans.field_type, String::new());
        assert_eq!(trans.name, String::new());
        assert_eq!(trans.value, String::new());
        assert_eq!(trans.maxlength, 0);
        assert_eq!(trans.required, false);
        assert_eq!(trans.checked, false);
        assert_eq!(trans.hint, String::new());
        assert_eq!(trans.unique, false);
        assert_eq!(trans.hidden, false);
        assert_eq!(trans.other_attrs, String::new());
        assert_eq!(trans.some_classes, String::new());
        assert_eq!(trans.select, vec![]);
        // Methods
    }

    // Testing Widget structure --------------------------------------------------------------------
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), DataType::Text(String::new()))];
        // Fields
        assert_eq!(widget.label, String::new());
        assert_eq!(
            widget.field_type.get_type(),
            FieldType::InputText.get_type()
        );
        assert_eq!(widget.relation_model, String::new());
        assert_eq!(
            widget.value.get_data(),
            DataType::Text(String::new()).get_data()
        );
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, true);
        assert_eq!(widget.hint, String::new());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, String::new());
        assert_eq!(widget.some_classes, String::new());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(widget.select[0].1.get_data(), String::new());
        // Methods
        let mut attrs = widget.get_clean_attrs("");
        attrs.select = vec![(String::new(), DataType::Text(String::new()).get_data())];

        assert_eq!(attrs.id, String::new());
        assert_eq!(attrs.label, String::new());
        assert_eq!(attrs.field_type, "text".to_string());
        assert_eq!(attrs.name, String::new());
        assert_eq!(attrs.value, String::new());
        assert_eq!(attrs.maxlength, 0);
        assert_eq!(attrs.required, true);
        assert_eq!(attrs.checked, false);
        assert_eq!(attrs.hint, String::new());
        assert_eq!(attrs.unique, false);
        assert_eq!(attrs.hidden, false);
        assert_eq!(attrs.other_attrs, String::new());
        assert_eq!(attrs.some_classes, String::new());
        assert_eq!(attrs.select[0].0, String::new());
        assert_eq!(attrs.select[0].1, String::new());
    }
}
