//! A field for entering integer 32-bit numbers.
//! For Html <input type="**number**|**radio**|**range**".

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct I32Field {
    /// The value is determined automatically.
    /// Format: "model-name--field-name".
    pub id: String,
    /// Web form field name.
    pub label: String,
    /// Field type.
    pub field_type: String,
    /// For Html `<input type="number|radio|range">`.
    pub input_type: String,
    /// The value is determined automatically.
    pub name: String,
    /// Sets the value of an element.
    pub value: Option<i32>,
    /// Value by default.
    pub default: Option<i32>,
    /// Displays prompt text.
    pub placeholder: String,
    /// Mandatory field.
    pub required: bool,
    /// The unique value of a field in a collection.
    pub unique: bool,
    /// Blocks access and modification of the element.
    pub disabled: bool,
    /// Specifies that the field cannot be modified by the user.
    pub readonly: bool,
    /// Increment step for numeric fields.
    pub step: i32,
    /// The lower value for entering a number or date.
    pub min: i32,
    /// The top value for entering a number or date.
    pub max: i32,
    /// For Html `<input type="radio" />`.
    /// Format: [(Value, Title), ...]
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

impl Default for I32Field {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("I32Field"),
            input_type: String::from("number"), // number|radio|range
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: 1,
            min: 0,
            max: i32::MAX,       // For Html <input type="range" /> default = 100
            choices: Vec::new(), // For Html <input type="radio" />. Format: [(Value, Title), ...]
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 10,
        }
    }
}

impl I32Field {
    /// Getter
    pub fn get(&self) -> Option<i32> {
        self.value
    }
    /// Setter
    pub fn set(&mut self, value: i32) {
        self.value = Some(value);
    }
}
