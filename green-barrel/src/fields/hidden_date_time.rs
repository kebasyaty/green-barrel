//! HiddenDateTime

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HiddenDateTime {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // "hidden | datetime" - The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Default value.
    pub default: Option<String>, // Value by default
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
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
}

impl Default for HiddenDateTime {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("HiddenDateTime"),
            input_type: String::from("hidden"), // "hidden | datetime"
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            pattern: String::new(),
            required: false,
            unique: false,
            disabled: true,
            readonly: false,
            min: String::new(),
            max: String::new(),
            is_hide: true,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
        }
    }
}

impl HiddenDateTime {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
