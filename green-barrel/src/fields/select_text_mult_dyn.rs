//! SelectTextMultDyn

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SelectTextMultDyn {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<Vec<String>>, // Default value.
    pub default: Option<Vec<String>>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub options: Vec<(String, String)>, // Html tag: <option value="value">Title</option> ; Example: vec![("value", "Title"), ("value 2", "Title 2")].
    pub is_hide: bool,                  // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
}

impl Default for SelectTextMultDyn {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("SelectTextMultDyn"),
            input_type: String::from("select"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            minlength: 0,
            maxlength: 256,
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            options: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
        }
    }
}

impl SelectTextMultDyn {
    pub fn set(&mut self, value: Vec<&str>) {
        let value = value
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        self.value = Some(value);
    }
}
