//! Boolean field.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoolField {
    /// The value is determined automatically.
    /// Format: "model-name--field-name".
    pub id: String,
    /// Web form field name.
    pub label: String,
    /// Field type.
    pub field_type: String,
    /// The value is determined automatically.
    pub input_type: String,
    /// The value is determined automatically.
    pub name: String,
    /// Sets the value of an element.
    pub value: Option<bool>,
    /// Value by default
    pub default: Option<bool>,
    /// Displays prompt text.
    pub placeholder: String,
    /// Blocks access and modification of the element.
    pub disabled: bool,
    /// Specifies that the field cannot be modified by the user.
    pub readonly: bool,
    /// Hide field from user.
    pub is_hide: bool,
    /// Example: r# "autofocus tabindex="some number" size="some number""#.
    pub other_attrs: String,
    /// Example: "class-name-1 class-name-2".
    pub css_classes: String,
    /// Additional explanation for the user.
    pub hint: String,
    /// Warning information.
    pub warning: String,
    /// The value is determined automatically.
    pub errors: Vec<String>,
    /// To optimize field traversal in the `paladins/check()` method.
    /// Hint: It is recommended not to change.
    pub group: u32,
}

impl Default for BoolField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("BoolField"),
            input_type: String::from("checkbox"),
            name: String::new(),
            value: None,
            default: Some(false),
            placeholder: String::new(),
            disabled: false,
            readonly: false,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 13,
        }
    }
}

impl BoolField {
    /// Getter
    pub fn get(&self) -> Option<bool> {
        self.value
    }
    /// Setter
    pub fn set(&mut self, value: bool) {
        self.value = Some(value);
    }
}
