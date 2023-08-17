//! Field for entering float numbers.
//! For Html <input type="**number**|**radio**|**range**".

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct F64Field {
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
    pub value: Option<f64>,
    /// Value by default.
    pub default: Option<f64>,
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
    pub step: f64,
    /// The lower value for entering a number or date.
    pub min: f64,
    /// The top value for entering a number or date.
    /// For Html `<input type="range" />` default = 100.0
    pub max: f64,
    /// For Html `<input type="radio" />`.
    /// Format: [(Value, Title), ...]
    pub choices: Vec<(f64, String)>,
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

impl Default for F64Field {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("F64Field"),
            input_type: String::from("number"), // number|radio|range
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: 1.0,
            min: 0.0,
            max: f64::MAX,       // For Html <input type="range" /> default = 100.0
            choices: Vec::new(), // For Html <input type="radio" />. Format: [(Value, Title), ...]
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 12,
        }
    }
}

impl F64Field {
    /// Getter
    pub fn get(&self) -> Option<f64> {
        self.value
    }
    /// Setter
    pub fn set(&mut self, value: f64) {
        self.value = Some(value);
    }
}
