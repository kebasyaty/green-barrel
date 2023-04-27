//! A field for entering unsigned 32-bit integers.
//! For Html <input type="**number**|**radio**|**range**".

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct U32 {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // For Html <input type="number|radio|range">
    pub name: String, // The value is determined automatically.
    pub value: Option<u32>, // Sets the value of an element.
    pub default: Option<u32>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub step: u32,  // Increment step for numeric fields.
    pub min: u32,   // The lower value for entering a number or date.
    pub max: u32,   // The top value for entering a number or date.
    pub choices: Vec<(u32, String)>, // For Html <input type="radio" />. Format: [(Value, Title), ...]
    pub is_hide: bool,               // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: Vec<String>,  // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for U32 {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("U32"),
            input_type: String::from("number"), // number|radio|range
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
            max: u32::MAX,       // For Html <input type="range" /> default = 100
            choices: Vec::new(), // For Html <input type="radio" />. Format: [(Value, Title), ...]
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: Vec::new(),
            group: 11,
        }
    }
}

impl U32 {
    pub fn get(&self) -> Option<u32> {
        self.value
    }
    pub fn set(&mut self, value: u32) {
        self.value = Some(value);
    }
}
