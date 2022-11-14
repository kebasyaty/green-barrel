//! NumberI32 - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NumberI32 {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<i32>, // Sets the value of an element.
    pub default: Option<i32>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub step: i32,  // Increment step for numeric fields.
    pub min: i32,   // The lower value for entering a number or date.
    pub max: i32,   // The top value for entering a number or date.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for NumberI32 {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("NumberI32"),
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
            min: 0,
            max: i32::MAX,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 10,
        }
    }
}

impl NumberI32 {
    pub fn get(&self) -> Option<i32> {
        self.value
    }
    pub fn set(&mut self, value: i32) {
        self.value = Some(value);
    }
}
