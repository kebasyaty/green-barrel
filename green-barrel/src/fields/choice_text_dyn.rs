//! Type of selective field with dynamic addition of elements.
//! For simulate relationship Many-to-One.
//! Elements are added via the `ModelName::update_dyn_field()` method.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceTextDynField {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element.
    pub placeholder: String, // Displays prompt text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub multiple: String, // Specifies that multiple options can be selected at once. Changing the default value is not recommended.
    pub choices: Vec<(String, String)>, // Elements are added via the ModelName::update_dyn_field() method.
    pub is_hide: bool,                  // Hide field from user.
    /// Example: `r# "autofocus tabindex="some number" size="some number"#`.    
    pub other_attrs: String,
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub errors: Vec<String>, // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for ChoiceTextDynField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("ChoiceTextDynField"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            // Changing the default value is not recommended.
            multiple: String::new(),
            choices: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 5,
        }
    }
}

impl ChoiceTextDynField {
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
