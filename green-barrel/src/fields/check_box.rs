//! CheckBox

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CheckBox {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<bool>, // Default value.
    pub default: Option<bool>, // Value by default
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub checked: bool, // A pre-activated radio button or checkbox.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String, // Additional explanation for the user.
    pub warning: String, // The value is determined automatically.
    pub error: String, // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("CheckBox"),
            input_type: String::from("checkbox"),
            name: String::new(),
            value: Some(false),
            default: None,
            placeholder: String::new(),
            required: false,
            checked: false,
            disabled: false,
            readonly: false,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
        }
    }
}

impl CheckBox {
    pub fn set(&mut self, value: bool) {
        self.value = Some(value);
    }
}