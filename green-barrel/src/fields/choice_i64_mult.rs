//! Type of selective field with static of elements.
//! With multiple choice.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceI64MultField {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub name: String, // The value is determined automatically.
    pub value: Option<Vec<i64>>, // Sets the value of an element.
    pub default: Option<Vec<i64>>, // Value by default.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub multiple: String, // Specifies that multiple options can be selected at once. Changing the default value is not recommended.
    pub choices: Vec<(i64, String)>, // Html tag: <option value="value">Title</option> ; Example: vec![(5, "Title"), (25, "Title 2")].
    pub is_hide: bool,               // Hide field from user.
    /// Example: `r# "autofocus tabindex="some number" size="some number"#`.    
    pub other_attrs: String,
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub errors: Vec<String>, // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for ChoiceI64MultField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("ChoiceI64MultField"),
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            // Changing the default value is not recommended.
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

impl ChoiceI64MultField {
    pub fn get(&self) -> Option<Vec<i64>> {
        self.value.clone()
    }
    pub fn set(&mut self, value: Vec<i64>) {
        self.value = Some(value);
    }
}
