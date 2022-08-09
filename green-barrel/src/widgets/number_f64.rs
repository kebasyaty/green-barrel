//! NumberF64

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NumberF64<'a> {
    pub id: &'a str, // The value is determined automatically. Format: "model-name--field-name".
    pub label: &'a str, // Web form field name.
    pub widget: &'a str, // Widget name.
    pub input_type: &'a str, // The value is determined automatically.
    pub name: &'a str, // The value is determined automatically.
    pub value: Option<f64>, // Default value.
    pub placeholder: &'a str, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub step: f64,   // Increment step for numeric fields.
    pub min: Option<f64>, // The lower value for entering a number or date.
    pub max: Option<f64>, // The top value for entering a number or date.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: &'a str, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: &'a str, // Example: "class-name-1 class-name-2".
    pub hint: &'a str, // Additional explanation for the user.
    pub warning: String, // The value is determined automatically.
    pub error: String, // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl<'a> Default for NumberF64<'a> {
    fn default() -> Self {
        Self {
            id: "",
            label: "",
            widget: "NumberF64",
            input_type: "number",
            name: "",
            value: None,
            placeholder: "",
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: 1.0,
            min: None,
            max: None,
            is_hide: false,
            other_attrs: "",
            css_classes: "",
            hint: "",
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
        }
    }
}

impl<'a> NumberF64<'a> {
    pub fn set(&mut self, value: f64) {
        self.value = Some(value);
    }
}
