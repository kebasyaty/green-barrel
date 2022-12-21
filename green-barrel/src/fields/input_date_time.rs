//! InputDateTime - A field for entering a date и времени in the format **1970-01-01T00:00**.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputDateTime {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element. Example: 1970-01-01T00:00
    pub default: Option<String>, // Value by default Example: 1970-01-01T00:00
    pub placeholder: String, // Displays prompt text.
    pub pattern: String, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub required: bool,  // Mandatory field.
    pub unique: bool,    // The unique value of a field in a collection.
    pub disabled: bool,  // Blocks access and modification of the element.
    pub readonly: bool,  // Specifies that the field cannot be modified by the user.
    pub min: String,     // The lower value for entering a date and time.
    pub max: String,     // The top value for entering a date and time.
    pub is_hide: bool,   // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for InputDateTime {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("InputDateTime"),
            input_type: String::from("datetime"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            pattern: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            min: String::new(),
            max: String::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 3,
        }
    }
}

impl InputDateTime {
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
