//! InputImage

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InputImage<'a> {
    pub id: &'a str, // The value is determined automatically. Format: "model-name--field-name".
    pub label: &'a str, // Web form field name.
    pub widget: &'a str, // Widget name.
    pub input_type: &'a str, // The value is determined automatically.
    pub name: &'a str, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub accept: &'a str, // Example: "image/jpeg,image/png,image/gif"
    pub placeholder: &'a str, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub thumbnails: Vec<(&'a str, u32)>, // From one to four inclusive. Example: vec![("xs", 150),("sm", 300),("md", 600),("lg", 1200)] Hint: An Intel i7-4770 processor or better is recommended.
    pub is_hide: bool,                   // Hide field from user.
    pub other_attrs: &'a str, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: &'a str, // Example: "class-name-1 class-name-2".
    pub hint: &'a str,        // Additional explanation for the user.
    pub warning: String,      // The value is determined automatically.
    pub error: String,        // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl<'a> Default for InputImage<'a> {
    fn default() -> Self {
        Self {
            id: "",
            label: "",
            widget: "InputImage",
            input_type: "file",
            name: "",
            value: None,
            accept: "",
            placeholder: "",
            required: false,
            disabled: false,
            readonly: false,
            thumbnails: Vec::new(),
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

impl<'a> InputImage<'a> {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
