//! Type of selective field with dynamic addition of elements.
//! For simulate relationship Many-to-One.
//! Elements are added via the `ModelName::update_dyn_field()` method.

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChoiceI32DynField {
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
    pub value: Option<i32>,
    /// Displays prompt text.
    pub placeholder: String,
    /// Mandatory field.
    pub required: bool,
    /// Blocks access and modification of the element.
    pub disabled: bool,
    /// Specifies that the field cannot be modified by the user.
    pub readonly: bool,
    /// Specifies that multiple options can be selected at once.
    /// Changing the default value is not recommended.
    pub multiple: String,
    /// Elements are added via the `ModelName::update_dyn_field()` method.
    /// Html tag: `<select><option value="value">Title</option></select>`.
    /// Example: `vec![(5, "Title"), (25, "Title 2")]`.
    pub choices: Vec<(i32, String)>,
    /// Hide field from user.
    pub is_hide: bool,
    /// Example: `r# "autofocus tabindex="some number" size="some number"#`.    
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

impl Default for ChoiceI32DynField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("ChoiceI32DynField"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            required: false,
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

impl ChoiceI32DynField {
    /// Getter
    pub fn get(&self) -> Option<i32> {
        self.value
    }
    /// Setter
    pub fn set(&mut self, value: i32) {
        self.value = Some(value);
    }
}
