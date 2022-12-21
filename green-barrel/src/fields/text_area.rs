//! TextArea -

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextArea {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element.
    pub default: Option<String>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub pattern: String, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool,  // Mandatory field.
    pub unique: bool,    // The unique value of a field in a collection.
    pub disabled: bool,  // Blocks access and modification of the element.
    pub readonly: bool,  // Specifies that the field cannot be modified by the user.
    pub is_hide: bool,   // Hide field from user.
    pub dir: String,     // Specifies the direction and display of text. Options: ltr | rtl | auto.
    pub wrap: String, // Specifies how the text in a text area is to be wrapped when submitted in a form - wrap="soft | hard | off".
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for TextArea {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("TextArea"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0,
            maxlength: 10_000,
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            is_hide: false,
            dir: String::from("ltr"),
            wrap: String::from("soft"),
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 1,
        }
    }
}

impl TextArea {
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
