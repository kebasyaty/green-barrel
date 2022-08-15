//! InputFile

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::helpers::structures::FileData;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InputFile {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub widget: String, // Widget name.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<FileData>, // Default value.
    pub default: Option<FileData>, // Value by default
    pub accept: String, // Example: "image/jpeg,image/png,image/gif"
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl Default for InputFile {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            widget: String::from("InputFile"),
            input_type: String::from("file"),
            name: String::new(),
            value: None,
            default: None,
            accept: String::new(),
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
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

impl InputFile {
    pub fn set(&mut self, value: FileData) {
        self.value = Some(value);
    }
}
