//! InputImage - Controller (field type)

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

use crate::models::helpers::ImageData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputImage {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<ImageData>, // Sets the value of an element.
    pub default: Option<ImageData>, // Value by default
    pub media_root: String, // Root partition for storing files.
    pub media_url: String, // Url address to the root section.
    pub target_dir: String, // Directory for images inside media directory (inner path). Example: "images/avatars".
    pub accept: String,     // Example: "image/jpeg,image/png,image/gif"
    pub placeholder: String, // Displays prompt text.
    pub required: bool,     // Mandatory field.
    pub disabled: bool,     // Blocks access and modification of the element.
    pub readonly: bool,     // Specifies that the field cannot be modified by the user.
    pub thumbnails: Vec<(String, u32)>, // From one to four inclusive. Example: vec![("xs", 150),("sm", 300),("md", 600),("lg", 1200)] Hint: An Intel i7-4770 processor or better is recommended.
    pub is_quality: bool,               // Create thumbnails - Fast or qualitatively?
    pub is_hide: bool,                  // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub error: String,       // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for InputImage {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("InputImage"),
            input_type: String::from("file"),
            name: String::new(),
            value: None,
            default: None,
            media_root: String::from("./resources/media"),
            media_url: String::from("/media"),
            target_dir: String::from("images"),
            accept: String::new(),
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            thumbnails: Vec::new(),
            is_quality: true,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            group: 9,
        }
    }
}

impl InputImage {
    pub fn get(&self) -> Option<ImageData> {
        self.value.clone()
    }
    pub fn set(&mut self, value: &str) {
        self.value = Some(ImageData {
            path: value.into(),
            ..Default::default()
        });
    }
}
