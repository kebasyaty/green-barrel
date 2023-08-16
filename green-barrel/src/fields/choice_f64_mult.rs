//! -

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceF64MultField {
    /// The value is determined automatically.
    /// Format: "model-name--field-name".
    pub id: String,
    /// Web form field name.
    pub label: String,
    /// Field type.
    pub field_type: String,
    /// The value is determined automatically.
    pub name: String,
    /// Sets the value of an element.
    pub value: Option<Vec<f64>>,
    /// Value by default.
    pub default: Option<Vec<f64>>,
    /// Displays prompt text.
    pub placeholder: String,
    /// Mandatory field.
    pub required: bool,
    /// Blocks access and modification of the element.
    pub disabled: bool,
    /// Specifies that the field cannot be modified by the user.
    pub readonly: bool,
    /// Specifies that multiple options can be selected at once.
    pub multiple: String,
    /// Html tag: <option value="value">Title</option>.
    /// Example: vec![(5.0, "Title"), (25.0, "Title 2")].
    pub choices: Vec<(f64, String)>,
    /// Hide field from user.
    pub is_hide: bool,
    /// Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
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

impl Default for ChoiceF64MultField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("ChoiceF64MultField"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            multiple: String::from("multiple"),
            choices: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 6,
        }
    }
}

impl ChoiceF64MultField {
    /// Getter
    pub fn get(&self) -> Option<Vec<f64>> {
        self.value.clone()
    }
    /// Setter
    pub fn set(&mut self, value: Vec<f64>) {
        self.value = Some(value);
    }
}
