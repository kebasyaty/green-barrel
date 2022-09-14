//! HiddenHash - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HiddenHash {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // "hidden | text" - The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element.
    pub placeholder: String, // Displays prompt text.
    pub pattern: String, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool,  // Mandatory field.
    pub unique: bool,    // The unique value of a field in a collection.
    pub disabled: bool,  // Blocks access and modification of the element.
    pub readonly: bool,  // Specifies that the field cannot be modified by the user.
    pub is_hide: bool,   // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for HiddenHash {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("HiddenHash"),
            input_type: String::from("hidden"), // "hidden | text"
            name: String::new(),
            value: None,
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 12,
            maxlength: 12,
            required: false,
            unique: false,
            disabled: true,
            readonly: false,
            is_hide: true,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
            group: 1_u32,
        }
    }
}

impl HiddenHash {
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
