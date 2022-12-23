//! -

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectTextMult {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub name: String, // The value is determined automatically.
    pub value: Option<Vec<String>>, // Sets the value of an element.
    pub default: Option<Vec<String>>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub multiple: String, // Specifies that multiple options can be selected at once.
    pub options: Vec<(String, String)>, // Html tag: <option value="value">Title</option> ; Example: vec![("value", "Title"), ("value 2", "Title 2")].
    pub is_hide: bool,                  // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for SelectTextMult {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("SelectTextMult"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            multiple: String::from("multiple"),
            options: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 6,
        }
    }
}

impl SelectTextMult {
    pub fn get(&self) -> Option<Vec<String>> {
        self.value.clone()
    }
    pub fn set(&mut self, value: Vec<&str>) {
        let value = value
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        self.value = Some(value);
    }
}
