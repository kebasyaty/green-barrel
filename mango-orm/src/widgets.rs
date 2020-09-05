//! # Widgets
//!
//! Widgets for form elements.

use serde::Serialize;

// WIDGETS =========================================================================================
/// Field types for Widgets
#[derive(Debug, Clone)]
pub enum FieldType {
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
impl Default for FieldType {
    fn default() -> Self {
        FieldType::InputText
    }
}
impl FieldType {
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
            Self::ForeignKey => "select".to_string(),
            Self::ManyToMany => "select".to_string(),
            Self::OneToOne => "hidden".to_string(),
        }
    }
}

/// Relation model types ---------------------------------------------------------------------------
pub enum RelationModel<T> {
    ForeignKey(Option<T>),
    ManyToMany(Option<T>),
    OneToOne(Option<T>),
}
impl<T> RelationModel<T> {
    pub fn get_model(&self) -> <T> {
        match self {
            Self::ForeignKey(model) => model,
            Self::ManyToMany(model) => model,
            Self::OneToOne(model) => model,
        }
    } 
}

/// Primitive types for the `value` attribute ------------------------------------------------------
#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Text(String),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}
impl Default for PrimitiveType {
    fn default() -> Self {
        PrimitiveType::Text(String::new())
    }
}
impl PrimitiveType {
    pub fn get_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_owned(),
            Self::I64(data) => data.to_string(),
            Self::U64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
            Self::Bool(data) => data.to_string(),
        }
    }
}

/// Vector types for the `value` attribute ---------------------------------------------------------
/// Vector - Text
#[derive(Debug, Clone)]
pub enum VectorText {
    Data(Vec<String>),
}
impl VectorText {
    pub fn get_vector(&self) -> Vec<String> {
        match self {
            Self::Data(vector) => vector.to_vec(),
        }
    }
}
/// Vector - I64
#[derive(Debug, Clone)]
pub enum VectorI64 {
    Data(Vec<i64>),
}
impl VectorI64 {
    pub fn get_vector(&self) -> Vec<i64> {
        match self {
            Self::Data(vector) => vector.to_vec(),
        }
    }
}
/// Vector - U64
#[derive(Debug, Clone)]
pub enum VectorU64 {
    Data(Vec<u64>),
}
impl VectorU64 {
    pub fn get_vector(&self) -> Vec<u64> {
        match self {
            Self::Data(vector) => vector.to_vec(),
        }
    }
}
/// Vector - F64
#[derive(Debug, Clone)]
pub enum VectorF64 {
    Data(Vec<f64>),
}
impl VectorF64 {
    pub fn get_vector(&self) -> Vec<f64> {
        match self {
            Self::Data(vector) => vector.to_vec(),
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
#[derive(Default, Debug)]
pub struct Widget {
    pub label: String,
    pub field_type: FieldType,
    pub value: PrimitiveType,
    pub maxlength: u32,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,  // "autofocus step=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, PrimitiveType)>,
}

impl Widget {
    // Get pure attributes from a widget
    pub fn get_clean_attrs(&self, name: &str) -> Transport {
        let field_type = match self.hidden {
            true => "hidden".to_string(),
            false => self.field_type.get_type(),
        };
        let checked = match self.value {
            PrimitiveType::Bool(data) => data,
            _ => false,
        };
        let other_attrs = match self.field_type {
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
        assert_eq!(FieldType::InputEmail.get_type(), "email".to_string());
        assert_eq!(FieldType::InputImage.get_type(), "image".to_string());
        assert_eq!(FieldType::InputNumber.get_type(), "number".to_string());
        assert_eq!(FieldType::InputPassword.get_type(), "password".to_string());
        assert_eq!(FieldType::InputRadio.get_type(), "radio".to_string());
        assert_eq!(FieldType::InputRange.get_type(), "range".to_string());
        assert_eq!(FieldType::InputTel.get_type(), "tel".to_string());
        assert_eq!(FieldType::InputText.get_type(), "text".to_string());
        assert_eq!(FieldType::InputTime.get_type(), "time".to_string());
        assert_eq!(FieldType::InputUrl.get_type(), "url".to_string());
        assert_eq!(FieldType::TextArea.get_type(), "textarea".to_string());
        assert_eq!(FieldType::Select.get_type(), "select".to_string());
        assert_eq!(FieldType::ForeignKey.get_type(), "select".to_string());
        assert_eq!(FieldType::ManyToMany.get_type(), "select".to_string());
        assert_eq!(FieldType::OneToOne.get_type(), "hidden".to_string());
    }

    // Testing Primitive types ---------------------------------------------------------------------
    #[test]
    fn test_primitive_types() {
        assert_eq!(
            PrimitiveType::Text("Some text".to_string()).get_data(),
            "Some text".to_string()
        );
        assert_eq!(PrimitiveType::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(PrimitiveType::U64(10_u64).get_data(), 10_u64.to_string());
        assert_eq!(PrimitiveType::F64(10_f64).get_data(), 10_f64.to_string());
        assert_eq!(PrimitiveType::Bool(true).get_data(), true.to_string());
    }

    // Testing Vector types ------------------------------------------------------------------------
    #[test]
    fn test_vector_types() {
        assert_eq!(
            VectorText::Data(vec!["1".to_string(), "2".to_string()]).get_vector(),
            vec!["1".to_string(), "2".to_string()]
        );
        assert_eq!(VectorI64::Data(vec![1, -2]).get_vector(), vec![1, -2]);
        assert_eq!(VectorU64::Data(vec![1, 2]).get_vector(), vec![1, 2]);
        assert_eq!(VectorF64::Data(vec![1.0, 2.0]).get_vector(), vec![1.0, 2.0]);
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
        assert_eq!(trans.other_attrs, String::new());
        assert_eq!(trans.some_classes, String::new());
        assert_eq!(trans.select, vec![]);
        // Methods
    }

    // Testing Widget structure --------------------------------------------------------------------
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), PrimitiveType::Text(String::new()))];
        // Fields
        assert_eq!(widget.label, String::new());
        assert_eq!(
            widget.field_type.get_type(),
            FieldType::InputText.get_type()
        );
        assert_eq!(
            widget.value.get_data(),
            PrimitiveType::Text(String::new()).get_data()
        );
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, false);
        assert_eq!(widget.hint, String::new());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, String::new());
        assert_eq!(widget.some_classes, String::new());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(widget.select[0].1.get_data(), String::new());
        // Methods
        let mut attrs = widget.get_clean_attrs("");
        attrs.select = vec![(String::new(), PrimitiveType::Text(String::new()).get_data())];

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
        assert_eq!(attrs.other_attrs, String::new());
        assert_eq!(attrs.some_classes, String::new());
        assert_eq!(attrs.select[0].0, String::new());
        assert_eq!(attrs.select[0].1, String::new());
    }
}
