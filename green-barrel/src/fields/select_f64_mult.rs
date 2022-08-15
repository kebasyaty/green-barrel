//! SelectF64Mult

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectF64Mult {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub widget: String, // Widget name.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<Vec<f64>>, // Default value.
    pub default: Option<Vec<f64>>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub min: Option<f64>, // The lower value for entering a number.
    pub max: Option<f64>, // The top value for entering a number.
    pub options: Vec<(f64, String)>, // Html tag: <option value="value">Title</option> ; Example: vec![(5, "Title"), (25, "Title 2")].
    pub is_hide: bool,               // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl Default for SelectF64Mult {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            widget: String::from("SelectF64Mult"),
            input_type: String::from("select"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            min: None,
            max: None,
            options: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
        }
    }
}

impl SelectF64Mult {
    pub fn set(&mut self, value: Vec<f64>) {
        self.value = Some(value);
    }
}
