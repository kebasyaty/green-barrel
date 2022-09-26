//! SelectF64MultDyn - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SelectF64MultDyn {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<Vec<f64>>, // Sets the value of an element.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub options: Vec<(f64, String)>, // Html tag: <option value="value">Title</option> ; Example: vec![(5.0, "Title"), (25.0, "Title 2")].
    pub is_hide: bool,               // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for SelectF64MultDyn {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("SelectF64MultDyn"),
            input_type: String::from("select"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            options: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 7_u32,
        }
    }
}

impl SelectF64MultDyn {
    pub fn get(&self) -> Option<Vec<f64>> {
        self.value.clone()
    }
    pub fn set(&mut self, value: Vec<f64>) {
        self.value = Some(value);
    }
}
