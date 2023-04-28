//! Field for uploading images.

use core::fmt::Debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use uuid::Uuid;

use crate::models::helpers::ImageData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Image {
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
    pub is_quality: bool, // Create thumbnails - Fast=false or qualitatively=true? Default = true.
    pub is_hide: bool,    // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub errors: Vec<String>,  // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for Image {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("Image"),
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
            errors: Vec::new(),
            group: 9,
        }
    }
}

impl Image {
    pub fn get(&self) -> Option<ImageData> {
        self.value.clone()
    }
    pub fn set(&mut self, image_path: &str, is_delete: bool, media_root: Option<&str>) {
        if Regex::new(r"(?:(?:/|\\)\d{4}\-\d{2}\-\d{2}\-barrel(?:/|\\))")
            .unwrap()
            .is_match(image_path)
        {
            Err(format!(
                "This image is not allowed to be reused - {image_path}"
            ))
            .unwrap()
        }
        let image_path = if !image_path.is_empty() {
            Self::copy_file_to_tmp(image_path, media_root).unwrap()
        } else {
            String::new()
        };
        self.value = Some(ImageData {
            path: image_path,
            is_delete,
            ..Default::default()
        });
    }
    // Copy file to media_root}/tmp directory
    pub fn copy_file_to_tmp(
        image_path: &str,
        media_root: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let media_root = if let Some(media_root) = media_root {
            media_root.to_string()
        } else {
            "./resources/media".to_string()
        };
        let f_path = Path::new(image_path);
        if !f_path.is_file() {
            Err(format!("File is missing - {image_path}"))?
        }
        let dir_tmp = format!("{media_root}/tmp");
        fs::create_dir_all(dir_tmp.clone())?;
        let f_name = Uuid::new_v4().to_string();
        let ext = f_path.extension().unwrap().to_str().unwrap();
        let f_tmp = format!("{dir_tmp}/{f_name}.{ext}");
        fs::copy(image_path, f_tmp.clone())?;
        Ok(f_tmp)
    }
}
