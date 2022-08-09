//! InputDate

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InputDate<'a> {
    pub id: &'a str, // The value is determined automatically. Format: "model-name--field-name".
    pub label: &'a str, // Web form field name.
    pub widget: &'a str, // Widget name.
    pub input_type: &'a str, // The value is determined automatically.
    pub name: &'a str, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub placeholder: &'a str, // Displays prompt text.
    pub pattern: &'a str, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub required: bool,   // Mandatory field.
    pub unique: bool,     // The unique value of a field in a collection.
    pub disabled: bool,   // Blocks access and modification of the element.
    pub readonly: bool,   // Specifies that the field cannot be modified by the user.
    pub min: &'a str,     // The lower value for entering a number or date.
    pub max: &'a str,     // The top value for entering a number or date.
    pub is_hide: bool,    // Hide field from user.
    pub other_attrs: &'a str, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: &'a str, // Example: "class-name-1 class-name-2".
    pub hint: &'a str,    // Additional explanation for the user.
    pub warning: String,  // The value is determined automatically.
    pub error: String,    // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl<'a> Default for InputDate<'a> {
    fn default() -> Self {
        Self {
            id: "",
            label: "",
            widget: "InputDate",
            input_type: "date",
            name: "",
            value: None,
            placeholder: "",
            pattern: "",
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            min: "",
            max: "",
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

impl<'a> InputDate<'a> {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
