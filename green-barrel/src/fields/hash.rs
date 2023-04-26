//! This type was created specifically for the hash field.

use core::fmt::Debug;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hash {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // "hidden|text" - The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<String>, // Sets the value of an element.
    pub placeholder: String, // Displays prompt text.
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool, // Mandatory field.
    pub unique: bool, // The unique value of a field in a collection.
    pub disabled: bool, // Blocks access and modification of the element.
    pub readonly: bool, // Specifies that the field cannot be modified by the user.
    pub is_hide: bool, // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for Hash {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("Hash"),
            input_type: String::from("hidden"),
            name: String::new(),
            value: None,
            placeholder: String::new(),
            minlength: 12,
            maxlength: 12,
            required: false,
            unique: false,
            disabled: true,
            readonly: false,
            is_hide: true,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            alert: String::new(),
            group: 1,
        }
    }
}

impl Hash {
    pub fn get(&self) -> Option<String> {
        self.value.clone()
    }
    pub fn set(&mut self, value: &str) {
        self.value = Some(String::from(value));
    }

    pub fn obj_id(&self) -> Result<Option<ObjectId>, Box<dyn Error>> {
        let hash = self.value.clone().unwrap_or_default();
        if let Ok(obj_id) = ObjectId::parse_str(hash.as_str()) {
            return Ok(Some(obj_id));
        }
        Ok(None)
    }
    pub fn set_obj_id(&mut self, obj_id: ObjectId) {
        self.value = Some(obj_id.to_hex());
    }
}
