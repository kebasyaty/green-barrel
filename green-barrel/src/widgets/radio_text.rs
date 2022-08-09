//! RadioText

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RadioText<'a> {
    pub id: &'a str, // The value is determined automatically. Format: "model-name--field-name".
    pub label: &'a str, // Web form field name.
    pub widget: &'a str, // Widget name.
    pub input_type: &'a str, // The value is determined automatically.
    pub name: &'a str, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub placeholder: &'a str, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub options: Vec<(&'a str, &'a str)>, // Html tag: <option value="value">Title</option> ; Example: vec![("value", "Title"), ("value 2", "Title 2")].
    pub is_hide: bool,                    // Hide field from user.
    pub other_attrs: &'a str, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: &'a str, // Example: "class-name-1 class-name-2".
    pub hint: &'a str,        // Additional explanation for the user.
    pub warning: String,      // The value is determined automatically.
    pub error: String,        // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl<'a> Default for RadioText<'a> {
    fn default() -> Self {
        Self {
            id: "",
            label: "",
            widget: "RadioText",
            input_type: "radio",
            name: "",
            value: None,
            placeholder: "",
            required: false,
            disabled: false,
            readonly: false,
            options: Vec::new(),
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

impl<'a> RadioText<'a> {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
