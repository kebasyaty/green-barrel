//! InputFile - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::models::helpers::FileData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputFile {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<FileData>, // Sets the value of an element.
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
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for InputFile {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("InputFile"),
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
            group: 8_u32,
        }
    }
}

impl InputFile {
    pub fn get(&self) -> Option<FileData> {
        self.value.clone()
    }
    pub fn set(&mut self, value: FileData) {
        self.value = Some(value);
    }
}
