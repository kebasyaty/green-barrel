//! TextArea

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct TextArea<'a> {
    pub id: &'a str, // The value is determined automatically. Format: "model-name--field-name".
    pub label: &'a str, // Web form field name.
    pub widget: &'a str, // Widget name.
    pub input_type: &'a str, // The value is determined automatically.
    pub name: &'a str, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub placeholder: &'a str, // Displays prompt text.
    pub pattern: &'a str, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool,   // Mandatory field.
    pub unique: bool,     // The unique value of a field in a collection.
    pub disabled: bool,   // Blocks access and modification of the element.
    pub readonly: bool,   // Specifies that the field cannot be modified by the user.
    pub is_hide: bool,    // Hide field from user.
    pub cols: u32,        // 	Specifies the visible width of a text area.
    pub rows: u32,        // Specifies the visible number of lines in a text area.
    pub wrap: &'a str, // Specifies how the text in a text area is to be wrapped when submitted in a form - wrap="soft | hard".
    pub other_attrs: &'a str, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: &'a str, // Example: "class-name-1 class-name-2".
    pub hint: &'a str, // Additional explanation for the user.
    pub warning: String, // The value is determined automatically.
    pub error: String, // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self {
            id: "",
            label: "",
            widget: "TextArea",
            input_type: "text",
            name: "",
            value: None,
            placeholder: "",
            pattern: "",
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            is_hide: false,
            cols: 20,
            rows: 2,
            wrap: "soft",
            other_attrs: "",
            css_classes: "",
            hint: "",
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
        }
    }
}

impl<'a> TextArea<'a> {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
