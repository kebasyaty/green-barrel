//! NumberU32

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NumberU32 {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub widget: String, // Widget name.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<u32>, // Default value.
    pub default: Option<u32>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub step: u32,  // Increment step for numeric fields.
    pub min: Option<u32>, // The lower value for entering a number or date.
    pub max: Option<u32>, // The top value for entering a number or date.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl Default for NumberU32 {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            widget: String::from("NumberU32"),
            input_type: String::from("number"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: 1,
            min: None,
            max: None,
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

impl NumberU32 {
    pub fn set(&mut self, value: u32) {
        self.value = Some(value);
    }
}
