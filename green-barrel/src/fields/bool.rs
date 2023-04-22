//! Boolean field.
//! The values are defined in the **checked** parameter.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bool {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub checked: Option<bool>, // Use as the current value and default values.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String, // Additional explanation for the user.
    pub warning: String, // Warning information.
    pub error: String, // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for Bool {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("Bool"),
            input_type: String::from("checkbox"),
            name: String::new(),
            placeholder: String::new(),
            required: false,
            checked: Some(false), // Use as the current value and default values.
            disabled: false,
            readonly: false,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 13,
        }
    }
}

impl Bool {
    pub fn get(&self) -> Option<bool> {
        self.checked
    }
    pub fn set(&mut self, value: bool) {
        self.checked = Some(value);
    }
}
