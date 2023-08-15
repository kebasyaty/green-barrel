//! Boolean field.
//! The values are defined in the **checked** parameter.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoolField {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<bool>, // Sets the value of an element.
    pub default: Option<bool>, // Value by default
    pub placeholder: String, // Displays prompt text.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String, // Additional explanation for the user.
    pub warning: String, // Warning information.
    pub errors: Vec<String>, // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for BoolField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("BoolField"),
            input_type: String::from("checkbox"),
            name: String::new(),
            value: None,
            default: Some(false),
            placeholder: String::new(),
            disabled: false,
            readonly: false,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 13,
        }
    }
}

impl BoolField {
    pub fn get(&self) -> Option<bool> {
        self.value
    }
    pub fn set(&mut self, value: bool) {
        self.value = Some(value);
    }
}
