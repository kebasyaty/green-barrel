//! A field for entering a **text** string.
//! For Html <input type="**text**|**radio**" and **textarea**(multiline=true)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextField {
    /// The value is determined automatically.
    /// Format: "model-name--field-name".
    pub id: String,
    /// Web form field name.
    pub label: String,
    /// Field type.
    pub field_type: String,
    /// `<input type="text|radio">`.
    pub input_type: String,
    /// true - for textarea.
    pub multiline: bool,
    /// The value is determined automatically.
    pub name: String,
    /// Sets the value of an element.
    pub value: Option<String>,
    /// Value by default.
    pub default: Option<String>,
    /// Displays prompt text.
    pub placeholder: String,
    /// A regular expression to validate the value.
    pub regex: String,
    /// To customize error message.
    pub regex_err_msg: String,
    /// The minimum number of characters allowed in the text.
    pub minlength: usize,
    /// The maximum number of characters allowed in the text.
    pub maxlength: usize,
    /// Mandatory field.
    pub required: bool,
    /// The unique value of a field in a collection.
    pub unique: bool,
    /// Blocks access and modification of the element.
    pub disabled: bool,
    /// Specifies that the field cannot be modified by the user.
    pub readonly: bool,
    /// For Html `<input type="radio" />`.
    /// Format: [(Value, Title), ...]
    pub choices: Vec<(String, String)>,
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

impl Default for TextField {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("TextField"),
            input_type: String::from("text"), // text|radio
            multiline: false,                 // true - for textarea.
            name: String::new(),
            value: None,
            default: None,
            placeholder: String::new(),
            regex: String::new(),
            regex_err_msg: String::new(),
            minlength: 0,
            maxlength: 256,
            required: false,
            unique: false,
            disabled: false,
            readonly: false,
            // For Html <input type="radio" />.
            // Format: [(value, Title), ...]
            choices: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 1,
        }
    }
}

impl TextField {
    /// Getter
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
    /// Setter
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }
}
