//! # Widgets
//!
//! Widgets for form elements.

use serde::Serialize;

// WIDGETS
// #################################################################################################
/// Field types for Widgets
#[derive(PartialEq, Clone, Debug)]
pub enum FieldType {
    Hash,
    InputCheckBoxText(String),
    InputCheckBoxI32(i32),
    InputCheckBoxU32(u32),
    InputCheckBoxI64(i64),
    InputCheckBoxF64(f64),
    InputRadioText(String),
    InputRadioI32(i32),
    InputRadioU32(u32),
    InputRadioI64(i64),
    InputRadioF64(f64),
    InputColor(String),
    InputDate(String),
    InputDateTime(String),
    InputEmail(String),
    InputFile,
    InputImage,
    InputNumberI32(i32),
    InputNumberU32(u32),
    InputNumberI64(i64),
    InputNumberF64(f64),
    InputPassword(String),
    InputRangeI32(i32),
    InputRangeU32(u32),
    InputRangeI64(i64),
    InputRangeF64(f64),
    InputTel(String),
    InputText(String),
    InputUrl(String),
    TextArea(String),
    SelectText(String),
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
    pub fn get_input_type(&self) -> &'static str {
        match self {
            Self::Hash => "hidden",
            Self::InputCheckBoxText(_) => "checkbox",
            Self::InputCheckBoxI32(_) => "checkbox",
            Self::InputCheckBoxU32(_) => "checkbox",
            Self::InputCheckBoxI64(_) => "checkbox",
            Self::InputCheckBoxF64(_) => "checkbox",
            Self::InputColor(_) => "color",
            Self::InputDate(_) => "date",
            Self::InputDateTime(_) => "datetime",
            Self::InputEmail(_) => "email",
            Self::InputFile => "file",
            Self::InputImage => "image",
            Self::InputNumberI32(_) => "number",
            Self::InputNumberU32(_) => "number",
            Self::InputNumberI64(_) => "number",
            Self::InputNumberF64(_) => "number",
            Self::InputPassword(_) => "password",
            Self::InputRadioText(_) => "radio",
            Self::InputRadioI32(_) => "radio",
            Self::InputRadioU32(_) => "radio",
            Self::InputRadioI64(_) => "radio",
            Self::InputRadioF64(_) => "radio",
            Self::InputRangeI32(_) => "range",
            Self::InputRangeU32(_) => "range",
            Self::InputRangeI64(_) => "range",
            Self::InputRangeF64(_) => "range",
            Self::InputTel(_) => "tel",
            Self::InputText(_) => "text",
            Self::InputUrl(_) => "url",
            Self::TextArea(_) => "textarea",
            Self::SelectText(_) => "select",
            Self::SelectI32(_) => "select",
            Self::SelectU32(_) => "select",
            Self::SelectI64(_) => "select",
            Self::SelectF64(_) => "select",
            Self::ForeignKey => "select",
            Self::ManyToMany => "select",
            Self::OneToOne => "hidden",
        }
    }

    pub fn get_raw_data(&self) -> String {
        match self {
            Self::Hash => String::new(),
            Self::InputCheckBoxText(data) => data.to_string(),
            Self::InputCheckBoxI32(data) => data.to_string(),
            Self::InputCheckBoxU32(data) => data.to_string(),
            Self::InputCheckBoxI64(data) => data.to_string(),
            Self::InputCheckBoxF64(data) => data.to_string(),
            Self::InputColor(data) => data.to_string(),
            Self::InputDate(data) => data.to_string(),
            Self::InputDateTime(data) => data.to_string(),
            Self::InputEmail(data) => data.to_string(),
            Self::InputFile => String::new(),
            Self::InputImage => String::new(),
            Self::InputNumberI32(data) => data.to_string(),
            Self::InputNumberU32(data) => data.to_string(),
            Self::InputNumberI64(data) => data.to_string(),
            Self::InputNumberF64(data) => data.to_string(),
            Self::InputPassword(data) => data.to_string(),
            Self::InputRadioText(data) => data.to_string(),
            Self::InputRadioI32(data) => data.to_string(),
            Self::InputRadioU32(data) => data.to_string(),
            Self::InputRadioI64(data) => data.to_string(),
            Self::InputRadioF64(data) => data.to_string(),
            Self::InputRangeI32(data) => data.to_string(),
            Self::InputRangeU32(data) => data.to_string(),
            Self::InputRangeI64(data) => data.to_string(),
            Self::InputRangeF64(data) => data.to_string(),
            Self::InputTel(data) => data.to_string(),
            Self::InputText(data) => data.to_string(),
            Self::InputUrl(data) => data.to_string(),
            Self::TextArea(data) => data.to_string(),
            Self::SelectText(data) => data.to_string(),
            Self::SelectI32(data) => data.to_string(),
            Self::SelectU32(data) => data.to_string(),
            Self::SelectI64(data) => data.to_string(),
            Self::SelectF64(data) => data.to_string(),
            Self::ForeignKey => String::new(),
            Self::ManyToMany => String::new(),
            Self::OneToOne => String::new(),
        }
    }

    pub fn get_data_type(&self) -> &'static str {
        match self {
            Self::Hash => "String",
            Self::InputCheckBoxText(_) => "String",
            Self::InputCheckBoxI32(_) => "i32",
            Self::InputCheckBoxU32(_) => "i64",
            Self::InputCheckBoxI64(_) => "i64",
            Self::InputCheckBoxF64(_) => "f64",
            Self::InputColor(_) => "String",
            Self::InputDate(_) => "String",
            Self::InputDateTime(_) => "String",
            Self::InputEmail(_) => "String",
            Self::InputFile => "none",
            Self::InputImage => "none",
            Self::InputNumberI32(_) => "i32",
            Self::InputNumberU32(_) => "i64",
            Self::InputNumberI64(_) => "i64",
            Self::InputNumberF64(_) => "f64",
            Self::InputPassword(_) => "String",
            Self::InputRadioText(_) => "String",
            Self::InputRadioI32(_) => "i32",
            Self::InputRadioU32(_) => "i64",
            Self::InputRadioI64(_) => "i64",
            Self::InputRadioF64(_) => "f64",
            Self::InputRangeI32(_) => "i32",
            Self::InputRangeU32(_) => "i64",
            Self::InputRangeI64(_) => "i64",
            Self::InputRangeF64(_) => "f64",
            Self::InputTel(_) => "String",
            Self::InputText(_) => "String",
            Self::InputUrl(_) => "String",
            Self::TextArea(_) => "String",
            Self::SelectText(_) => "String",
            Self::SelectI32(_) => "i32",
            Self::SelectU32(_) => "i64",
            Self::SelectI64(_) => "i64",
            Self::SelectF64(_) => "f64",
            Self::ForeignKey => "none",
            Self::ManyToMany => "none",
            Self::OneToOne => "none",
        }
    }

    pub fn get_enum_type(&self) -> &'static str {
        match self {
            Self::Hash => "Hash",
            Self::InputCheckBoxText(_) => "InputCheckBoxText",
            Self::InputCheckBoxI32(_) => "InputCheckBoxI32",
            Self::InputCheckBoxU32(_) => "InputCheckBoxU32",
            Self::InputCheckBoxI64(_) => "InputCheckBoxI64",
            Self::InputCheckBoxF64(_) => "InputCheckBoxF64",
            Self::InputColor(_) => "InputColor",
            Self::InputDate(_) => "InputDate",
            Self::InputDateTime(_) => "InputDateTime",
            Self::InputEmail(_) => "InputEmail",
            Self::InputFile => "InputFile",
            Self::InputImage => "InputImage",
            Self::InputNumberI32(_) => "InputNumberI32",
            Self::InputNumberU32(_) => "InputNumberU32",
            Self::InputNumberI64(_) => "InputNumberI64",
            Self::InputNumberF64(_) => "InputNumberF64",
            Self::InputPassword(_) => "InputPassword",
            Self::InputRadioText(_) => "InputRadioText",
            Self::InputRadioI32(_) => "InputRadioI32",
            Self::InputRadioU32(_) => "InputRadioU32",
            Self::InputRadioI64(_) => "InputRadioI64",
            Self::InputRadioF64(_) => "InputRadioF64",
            Self::InputRangeI32(_) => "InputRangeI32",
            Self::InputRangeU32(_) => "InputRangeU32",
            Self::InputRangeI64(_) => "InputRangeI64",
            Self::InputRangeF64(_) => "InputRangeF64",
            Self::InputTel(_) => "InputTel",
            Self::InputText(_) => "InputText",
            Self::InputUrl(_) => "InputUrl",
            Self::TextArea(_) => "TextArea",
            Self::SelectText(_) => "SelectText",
            Self::SelectI32(_) => "SelectI32",
            Self::SelectU32(_) => "SelectU32",
            Self::SelectI64(_) => "SelectI64",
            Self::SelectF64(_) => "SelectF64",
            Self::ForeignKey => "ForeignKey",
            Self::ManyToMany => "ManyToMany",
            Self::OneToOne => "OneToOne",
        }
    }
}

/// Data types for the `select` attribute
// *************************************************************************************************
#[derive(Clone, Debug)]
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
    pub fn get_raw_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_owned(),
            Self::I32(data) => data.to_string(),
            Self::U32(data) => data.to_string(),
            Self::I64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
        }
    }

    pub fn get_data_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "String",
            Self::I32(_) => "i32",
            Self::U32(_) => "i64",
            Self::I64(_) => "i64",
            Self::F64(_) => "f64",
        }
    }

    pub fn get_enum_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "Text",
            Self::I32(_) => "I32",
            Self::U32(_) => "U32",
            Self::I64(_) => "I64",
            Self::F64(_) => "F64",
        }
    }
}

/// Datatypes for the `step`,` min` and `max` attributes
// *************************************************************************************************
#[derive(PartialEq, Clone, Debug)]
pub enum StepMinMax {
    I32(i32),
    U32(u32),
    I64(i64),
    F64(f64),
}

impl Default for StepMinMax {
    fn default() -> Self {
        StepMinMax::I32(0_i32)
    }
}

impl StepMinMax {
    pub fn get_raw_data(&self) -> String {
        match self {
            Self::I32(data) => data.to_string(),
            Self::U32(data) => data.to_string(),
            Self::I64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
        }
    }

    pub fn get_data_type(&self) -> &'static str {
        match self {
            Self::I32(_) => "i32",
            Self::U32(_) => "i64",
            Self::I64(_) => "i64",
            Self::F64(_) => "f64",
        }
    }

    pub fn get_enum_type(&self) -> &'static str {
        match self {
            Self::I32(_) => "I32",
            Self::U32(_) => "U32",
            Self::I64(_) => "I64",
            Self::F64(_) => "F64",
        }
    }
}

/// Mediator for transporting widget attributes
// *************************************************************************************************
#[derive(Serialize, Default, Clone, Debug)]
pub struct Transport {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub value: String,
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub step: String,
    pub min: String,
    pub max: String,
    pub other_attrs: String,  // "autofocus size=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, String)>,
    pub error: String,
}

/// Attributes for the widget
// *************************************************************************************************
/// For standard widgets:
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
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub step: StepMinMax,
    pub min: StepMinMax,
    pub max: StepMinMax,
    pub other_attrs: String,  // "autofocus size=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, SelectDataType)>,
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            label: String::new(),
            relation_model: String::new(),
            value: FieldType::default(),
            maxlength: 0_usize,
            required: false,
            checked: false, // For <input type="checkbox|radio">
            hint: String::new(),
            unique: false,
            hidden: false,
            step: StepMinMax::default(),
            min: StepMinMax::default(),
            max: StepMinMax::default(),
            other_attrs: String::new(),
            some_classes: String::new(),
            select: vec![],
        }
    }
}

impl Widget {
    // Get pure attributes from a widget
    pub fn clean_attrs(&self, name: &str) -> Result<Transport, Box<dyn std::error::Error>> {
        let field_type = match self.hidden {
            true => "hidden".to_string(),
            false => self.value.get_input_type().to_string(),
        };
        let other_attrs = match self.value {
            FieldType::ManyToMany => match self.other_attrs.contains("multiple") {
                true => self.other_attrs.clone(),
                false => format!("multiple {}", self.other_attrs),
            },
            _ => self.other_attrs.clone(),
        };

        Ok(Transport {
            id: name.to_string(),
            label: self.label.clone(),
            field_type: field_type,
            name: name.to_string(),
            value: self.value.get_raw_data(),
            maxlength: self.maxlength.clone(),
            required: self.required.clone(),
            checked: self.checked.clone(),
            hint: self.hint.clone(),
            unique: self.unique.clone(),
            hidden: self.hidden.clone(),
            step: self.step.get_raw_data(),
            min: self.min.get_raw_data(),
            max: self.max.get_raw_data(),
            other_attrs: other_attrs,
            some_classes: self.some_classes.clone(),
            select: self
                .select
                .iter()
                .map(|item| (item.0.clone(), item.1.get_raw_data()))
                .collect::<Vec<(String, String)>>(),
            error: String::new(),
        })
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    use super::*;

    // Testing enum FieldTypes
    // *********************************************************************************************
    #[test]
    fn test_field_types() {
        // Method get_input_type()
        // -----------------------------------------------------------------------------------------
        assert_eq!(FieldType::Hash.get_input_type(), "hidden");
        assert_eq!(
            FieldType::InputCheckBoxText(String::new()).get_input_type(),
            "checkbox"
        );
        assert_eq!(
            FieldType::InputCheckBoxI32(-1_i32).get_input_type(),
            "checkbox"
        );
        assert_eq!(
            FieldType::InputCheckBoxU32(0_u32).get_input_type(),
            "checkbox"
        );
        assert_eq!(
            FieldType::InputCheckBoxI64(-1_i64).get_input_type(),
            "checkbox"
        );
        assert_eq!(
            FieldType::InputCheckBoxF64(1.3_f64).get_input_type(),
            "checkbox"
        );
        assert_eq!(
            FieldType::InputColor(String::new()).get_input_type(),
            "color"
        );
        assert_eq!(FieldType::InputDate(String::new()).get_input_type(), "date");
        assert_eq!(
            FieldType::InputDateTime(String::new()).get_input_type(),
            "datetime"
        );
        assert_eq!(
            FieldType::InputEmail(String::new()).get_input_type(),
            "email"
        );
        assert_eq!(FieldType::InputFile.get_input_type(), "file");
        assert_eq!(FieldType::InputImage.get_input_type(), "image");
        assert_eq!(FieldType::InputNumberI32(-1_i32).get_input_type(), "number");
        assert_eq!(FieldType::InputNumberU32(0_u32).get_input_type(), "number");
        assert_eq!(FieldType::InputNumberI64(-1_i64).get_input_type(), "number");
        assert_eq!(
            FieldType::InputNumberF64(-1.3_f64).get_input_type(),
            "number"
        );
        assert_eq!(
            FieldType::InputPassword(String::new()).get_input_type(),
            "password"
        );
        assert_eq!(
            FieldType::InputRadioText(String::new()).get_input_type(),
            "radio"
        );
        assert_eq!(FieldType::InputRadioI32(-1_i32).get_input_type(), "radio");
        assert_eq!(FieldType::InputRadioU32(0_u32).get_input_type(), "radio");
        assert_eq!(FieldType::InputRadioI64(-1_i64).get_input_type(), "radio");
        assert_eq!(FieldType::InputRadioF64(1.3_f64).get_input_type(), "radio");
        assert_eq!(FieldType::InputRangeI32(-1_i32).get_input_type(), "range");
        assert_eq!(FieldType::InputRangeU32(0_u32).get_input_type(), "range");
        assert_eq!(FieldType::InputRangeI64(-1_i64).get_input_type(), "range");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).get_input_type(), "range");
        assert_eq!(FieldType::InputTel(String::new()).get_input_type(), "tel");
        assert_eq!(FieldType::InputText(String::new()).get_input_type(), "text");
        assert_eq!(FieldType::InputUrl(String::new()).get_input_type(), "url");
        assert_eq!(
            FieldType::TextArea(String::new()).get_input_type(),
            "textarea"
        );
        assert_eq!(
            FieldType::SelectText(String::new()).get_input_type(),
            "select"
        );
        assert_eq!(FieldType::SelectI32(-1_i32).get_input_type(), "select");
        assert_eq!(FieldType::SelectU32(0_u32).get_input_type(), "select");
        assert_eq!(FieldType::SelectI64(-1_i64).get_input_type(), "select");
        assert_eq!(FieldType::SelectF64(-1.3_f64).get_input_type(), "select");
        assert_eq!(FieldType::ForeignKey.get_input_type(), "select");
        assert_eq!(FieldType::ManyToMany.get_input_type(), "select");
        assert_eq!(FieldType::OneToOne.get_input_type(), "hidden");

        // Method get_raw_data()
        // -----------------------------------------------------------------------------------------
        assert_eq!(FieldType::Hash.get_raw_data(), String::new());
        assert_eq!(
            FieldType::InputCheckBoxText("Some text".to_string()).get_raw_data(),
            "Some text".to_string()
        );
        assert_eq!(FieldType::InputCheckBoxI32(-1_i32).get_raw_data(), "-1");
        assert_eq!(FieldType::InputCheckBoxU32(0_u32).get_raw_data(), "0");
        assert_eq!(FieldType::InputCheckBoxI64(-1_i64).get_raw_data(), "-1");
        assert_eq!(FieldType::InputCheckBoxF64(1.3_f64).get_raw_data(), "1.3");
        assert_eq!(
            FieldType::InputColor(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputDate(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputDateTime(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputEmail(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(FieldType::InputFile.get_raw_data(), String::new());
        assert_eq!(FieldType::InputImage.get_raw_data(), String::new());
        assert_eq!(FieldType::InputNumberI32(-1_i32).get_raw_data(), "-1");
        assert_eq!(FieldType::InputNumberU32(0_u32).get_raw_data(), "0");
        assert_eq!(FieldType::InputNumberI64(-1_i64).get_raw_data(), "-1");
        assert_eq!(FieldType::InputNumberF64(-1.3_f64).get_raw_data(), "-1.3");
        assert_eq!(
            FieldType::InputPassword(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputRadioText("Some text".to_string()).get_raw_data(),
            "Some text"
        );
        assert_eq!(FieldType::InputRadioI32(-1_i32).get_raw_data(), "-1");
        assert_eq!(FieldType::InputRadioU32(0_u32).get_raw_data(), "0");
        assert_eq!(FieldType::InputRadioI64(-1_i64).get_raw_data(), "-1");
        assert_eq!(FieldType::InputRadioF64(1.3_f64).get_raw_data(), "1.3");
        assert_eq!(FieldType::InputRangeI32(-1_i32).get_raw_data(), "-1");
        assert_eq!(FieldType::InputRangeU32(0_u32).get_raw_data(), "0");
        assert_eq!(FieldType::InputRangeI64(-1_i64).get_raw_data(), "-1");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).get_raw_data(), "-1.3");
        assert_eq!(
            FieldType::InputTel(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputText(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputUrl(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::TextArea(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::SelectText(String::new()).get_raw_data(),
            String::new()
        );
        assert_eq!(FieldType::SelectI32(-1_i32).get_raw_data(), "-1");
        assert_eq!(FieldType::SelectU32(0_u32).get_raw_data(), "0");
        assert_eq!(FieldType::SelectI64(-1_i64).get_raw_data(), "-1");
        assert_eq!(FieldType::SelectF64(-1.3_f64).get_raw_data(), "-1.3");
        assert_eq!(FieldType::ForeignKey.get_raw_data(), String::new());
        assert_eq!(FieldType::ManyToMany.get_raw_data(), String::new());
        assert_eq!(FieldType::OneToOne.get_raw_data(), String::new());

        // Method get_data_type()
        // -----------------------------------------------------------------------------------------
        assert_eq!(FieldType::Hash.get_data_type(), "String");
        assert_eq!(
            FieldType::InputCheckBoxText(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(FieldType::InputCheckBoxI32(-1_i32).get_data_type(), "i32");
        assert_eq!(FieldType::InputCheckBoxU32(0_u32).get_data_type(), "i64");
        assert_eq!(FieldType::InputCheckBoxI64(-1_i64).get_data_type(), "i64");
        assert_eq!(FieldType::InputCheckBoxF64(1.3_f64).get_data_type(), "f64");
        assert_eq!(
            FieldType::InputColor(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(
            FieldType::InputDate(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(
            FieldType::InputDateTime(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(
            FieldType::InputEmail(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(FieldType::InputFile.get_data_type(), "none");
        assert_eq!(FieldType::InputImage.get_data_type(), "none");
        assert_eq!(FieldType::InputNumberI32(-1_i32).get_data_type(), "i32");
        assert_eq!(FieldType::InputNumberU32(0_u32).get_data_type(), "i64");
        assert_eq!(FieldType::InputNumberI64(-1_i64).get_data_type(), "i64");
        assert_eq!(FieldType::InputNumberF64(-1.3_f64).get_data_type(), "f64");
        assert_eq!(
            FieldType::InputPassword(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(
            FieldType::InputRadioText(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(FieldType::InputRadioI32(-1_i32).get_data_type(), "i32");
        assert_eq!(FieldType::InputRadioU32(0_u32).get_data_type(), "i64");
        assert_eq!(FieldType::InputRadioI64(-1_i64).get_data_type(), "i64");
        assert_eq!(FieldType::InputRadioF64(1.3_f64).get_data_type(), "f64");
        assert_eq!(FieldType::InputRangeI32(-1_i32).get_data_type(), "i32");
        assert_eq!(FieldType::InputRangeU32(0_u32).get_data_type(), "i64");
        assert_eq!(FieldType::InputRangeI64(-1_i64).get_data_type(), "i64");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).get_data_type(), "f64");
        assert_eq!(FieldType::InputTel(String::new()).get_data_type(), "String");
        assert_eq!(
            FieldType::InputText(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(FieldType::InputUrl(String::new()).get_data_type(), "String");
        assert_eq!(FieldType::TextArea(String::new()).get_data_type(), "String");
        assert_eq!(
            FieldType::SelectText(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(FieldType::SelectI32(-1_i32).get_data_type(), "i32");
        assert_eq!(FieldType::SelectU32(0_u32).get_data_type(), "i64");
        assert_eq!(FieldType::SelectI64(-1_i64).get_data_type(), "i64");
        assert_eq!(FieldType::SelectF64(-1.3_f64).get_data_type(), "f64");
        assert_eq!(FieldType::ForeignKey.get_data_type(), "none");
        assert_eq!(FieldType::ManyToMany.get_data_type(), "none");
        assert_eq!(FieldType::OneToOne.get_data_type(), "none");

        // Method get_enum_type()
        // -----------------------------------------------------------------------------------------
        assert_eq!(FieldType::Hash.get_enum_type(), "Hash");
        assert_eq!(
            FieldType::InputCheckBoxText(String::new()).get_enum_type(),
            "InputCheckBoxText"
        );
        assert_eq!(
            FieldType::InputCheckBoxI32(-1_i32).get_enum_type(),
            "InputCheckBoxI32"
        );
        assert_eq!(
            FieldType::InputCheckBoxU32(0_u32).get_enum_type(),
            "InputCheckBoxU32"
        );
        assert_eq!(
            FieldType::InputCheckBoxI64(-1_i64).get_enum_type(),
            "InputCheckBoxI64"
        );
        assert_eq!(
            FieldType::InputCheckBoxF64(1.3_f64).get_enum_type(),
            "InputCheckBoxF64"
        );
        assert_eq!(
            FieldType::InputColor(String::new()).get_enum_type(),
            "InputColor"
        );
        assert_eq!(
            FieldType::InputDate(String::new()).get_enum_type(),
            "InputDate"
        );
        assert_eq!(
            FieldType::InputDateTime(String::new()).get_enum_type(),
            "InputDateTime"
        );
        assert_eq!(
            FieldType::InputEmail(String::new()).get_enum_type(),
            "InputEmail"
        );
        assert_eq!(FieldType::InputFile.get_enum_type(), "InputFile");
        assert_eq!(FieldType::InputImage.get_enum_type(), "InputImage");
        assert_eq!(
            FieldType::InputNumberI32(-1_i32).get_enum_type(),
            "InputNumberI32"
        );
        assert_eq!(
            FieldType::InputNumberU32(0_u32).get_enum_type(),
            "InputNumberU32"
        );
        assert_eq!(
            FieldType::InputNumberI64(-1_i64).get_enum_type(),
            "InputNumberI64"
        );
        assert_eq!(
            FieldType::InputNumberF64(-1.3_f64).get_enum_type(),
            "InputNumberF64"
        );
        assert_eq!(
            FieldType::InputPassword(String::new()).get_enum_type(),
            "InputPassword"
        );
        assert_eq!(
            FieldType::InputRadioText(String::new()).get_enum_type(),
            "InputRadioText"
        );
        assert_eq!(
            FieldType::InputRadioI32(-1_i32).get_enum_type(),
            "InputRadioI32"
        );
        assert_eq!(
            FieldType::InputRadioU32(0_u32).get_enum_type(),
            "InputRadioU32"
        );
        assert_eq!(
            FieldType::InputRadioI64(-1_i64).get_enum_type(),
            "InputRadioI64"
        );
        assert_eq!(
            FieldType::InputRadioF64(1.3_f64).get_enum_type(),
            "InputRadioF64"
        );
        assert_eq!(
            FieldType::InputRangeI32(-1_i32).get_enum_type(),
            "InputRangeI32"
        );
        assert_eq!(
            FieldType::InputRangeU32(0_u32).get_enum_type(),
            "InputRangeU32"
        );
        assert_eq!(
            FieldType::InputRangeI64(-1_i64).get_enum_type(),
            "InputRangeI64"
        );
        assert_eq!(
            FieldType::InputRangeF64(-1.3_f64).get_enum_type(),
            "InputRangeF64"
        );
        assert_eq!(
            FieldType::InputTel(String::new()).get_enum_type(),
            "InputTel"
        );
        assert_eq!(
            FieldType::InputText(String::new()).get_enum_type(),
            "InputText"
        );
        assert_eq!(
            FieldType::InputUrl(String::new()).get_enum_type(),
            "InputUrl"
        );
        assert_eq!(
            FieldType::TextArea(String::new()).get_enum_type(),
            "TextArea"
        );
        assert_eq!(
            FieldType::SelectText(String::new()).get_enum_type(),
            "SelectText"
        );
        assert_eq!(FieldType::SelectI32(-1_i32).get_enum_type(), "SelectI32");
        assert_eq!(FieldType::SelectU32(0_u32).get_enum_type(), "SelectU32");
        assert_eq!(FieldType::SelectI64(-1_i64).get_enum_type(), "SelectI64");
        assert_eq!(FieldType::SelectF64(-1.3_f64).get_enum_type(), "SelectF64");
        assert_eq!(FieldType::ForeignKey.get_enum_type(), "ForeignKey");
        assert_eq!(FieldType::ManyToMany.get_enum_type(), "ManyToMany");
        assert_eq!(FieldType::OneToOne.get_enum_type(), "OneToOne");
    }

    // Testing data types for the `select` attribute
    // *********************************************************************************************
    #[test]
    fn test_select_data_types() {
        // Method get_raw_data()
        // -----------------------------------------------------------------------------------------
        assert_eq!(
            SelectDataType::Text("Some text".to_string()).get_raw_data(),
            "Some text".to_string()
        );
        assert_eq!(
            SelectDataType::I32(-10_i32).get_raw_data(),
            (-10_i32).to_string()
        );
        assert_eq!(
            SelectDataType::U32(10_u32).get_raw_data(),
            10_u32.to_string()
        );
        assert_eq!(
            SelectDataType::I64(-10_i64).get_raw_data(),
            (-10_i64).to_string()
        );
        assert_eq!(
            SelectDataType::F64(-10_f64).get_raw_data(),
            (-10_f64).to_string()
        );

        // Method get_data_type()
        // -----------------------------------------------------------------------------------------
        assert_eq!(
            SelectDataType::Text(String::new()).get_data_type(),
            "String"
        );
        assert_eq!(SelectDataType::I32(-10_i32).get_data_type(), "i32");
        assert_eq!(SelectDataType::U32(10_u32).get_data_type(), "i64");
        assert_eq!(SelectDataType::I64(-10_i64).get_data_type(), "i64");
        assert_eq!(SelectDataType::F64(-10_f64).get_data_type(), "f64");
    }

    // Testing data Types for the `step`,` min` and `max` attributes
    // *********************************************************************************************
    #[test]
    fn test_step_min_max_data_types() {
        // Method get_raw_data()
        // -----------------------------------------------------------------------------------------
        assert_eq!(
            StepMinMax::I32(-10_i32).get_raw_data(),
            (-10_i32).to_string()
        );
        assert_eq!(StepMinMax::U32(10_u32).get_raw_data(), 10_u32.to_string());
        assert_eq!(
            StepMinMax::I64(-10_i64).get_raw_data(),
            (-10_i64).to_string()
        );
        assert_eq!(
            StepMinMax::F64(-10_f64).get_raw_data(),
            (-10_f64).to_string()
        );

        // Method get_data_type()
        // -----------------------------------------------------------------------------------------
        assert_eq!(StepMinMax::I32(-10_i32).get_data_type(), "i32");
        assert_eq!(StepMinMax::U32(10_u32).get_data_type(), "i64");
        assert_eq!(StepMinMax::I64(-10_i64).get_data_type(), "i64");
        assert_eq!(StepMinMax::F64(-10_f64).get_data_type(), "f64");
    }

    // Testing Transport structure
    // *********************************************************************************************
    #[test]
    fn test_transport() {
        let trans: Transport = Default::default();
        // Fields
        // -----------------------------------------------------------------------------------------
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
        assert_eq!(trans.step, String::new());
        assert_eq!(trans.min, String::new());
        assert_eq!(trans.max, String::new());
        assert_eq!(trans.other_attrs, String::new());
        assert_eq!(trans.some_classes, String::new());
        assert_eq!(trans.select, vec![]);
        assert_eq!(trans.error, String::new());
    }

    // Testing Widget structure
    // *********************************************************************************************
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), SelectDataType::Text(String::new()))];
        // Fields
        // -----------------------------------------------------------------------------------------
        assert_eq!(widget.label, String::new());
        assert_eq!(
            widget.value.get_input_type(),
            FieldType::InputText(String::new()).get_input_type()
        );
        assert_eq!(widget.relation_model, String::new());
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, String::new());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(
            widget.step.get_data_type(),
            StepMinMax::default().get_data_type()
        );
        assert_eq!(
            widget.min.get_data_type(),
            StepMinMax::default().get_data_type()
        );
        assert_eq!(
            widget.max.get_data_type(),
            StepMinMax::default().get_data_type()
        );
        assert_eq!(widget.other_attrs, String::new());
        assert_eq!(widget.some_classes, String::new());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(widget.select[0].1.get_raw_data(), String::new());

        // Methods
        // -----------------------------------------------------------------------------------------
        let mut attrs = widget.clean_attrs("").unwrap();
        attrs.select = vec![(
            String::new(),
            SelectDataType::Text(String::new()).get_raw_data(),
        )];

        assert_eq!(attrs.id, String::new());
        assert_eq!(attrs.label, String::new());
        assert_eq!(attrs.field_type, "text".to_string());
        assert_eq!(attrs.name, String::new());
        assert_eq!(attrs.value, String::new());
        assert_eq!(attrs.maxlength, 0);
        assert_eq!(attrs.required, false);
        assert_eq!(attrs.checked, false);
        assert_eq!(attrs.hint, String::new());
        assert_eq!(attrs.unique, false);
        assert_eq!(attrs.hidden, false);
        assert_eq!(attrs.step, "0".to_string());
        assert_eq!(attrs.min, "0".to_string());
        assert_eq!(attrs.max, "0".to_string());
        assert_eq!(attrs.other_attrs, String::new());
        assert_eq!(attrs.some_classes, String::new());
        assert_eq!(attrs.select[0].0, String::new());
        assert_eq!(attrs.select[0].1, String::new());
        assert_eq!(attrs.error, String::new());
    }
}
