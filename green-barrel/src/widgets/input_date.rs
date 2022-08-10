//! InputDate

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InputDate {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub widget: String, // Widget name.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub placeholder: String, // Displays prompt text.
    pub pattern: String, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub required: bool,  // Mandatory field.
    pub unique: bool,    // The unique value of a field in a collection.
    pub disabled: bool,  // Blocks access and modification of the element.
    pub readonly: bool,  // Specifies that the field cannot be modified by the user.
    pub min: String,     // The lower value for entering a date.
    pub max: String,     // The top value for entering a date.
    pub is_hide: bool,   // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl Default for InputDate {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            widget: String::from("InputDate"),
            input_type: String::from("date"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            pattern: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            min: String::new(),
            max: String::new(),
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

impl InputDate {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
