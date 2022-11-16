//! AutoSlug - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AutoSlug {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element.
    pub placeholder: String, // Displays prompt text.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub slug_sources: Vec<String>, // Example: vec!["title"] or vec!["hash", "username"] or vec!["email", "first_name", "last_name"].
    pub is_hide: bool,             // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for AutoSlug {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("AutoSlug"),
            input_type: String::from("text"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            unique: true,
            disabled: false,
            readonly: true,
            slug_sources: vec!["hash".into()],
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 2,
        }
    }
}

impl AutoSlug {
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
}
