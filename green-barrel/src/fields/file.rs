//! File upload field.

use core::fmt::Debug;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use uuid::Uuid;

use crate::models::helpers::FileData;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
    pub label: String, // Web form field name.
    pub field_type: String, // Field type.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: Option<FileData>, // Sets the value of an element.
    pub default: Option<FileData>, // Value by default
    pub media_root: String, // Root partition for storing files.
    pub media_url: String, // Url address to the root section.
    pub target_dir: String, // Directory for files inside media directory (inner path). Example: "files/resume".
    pub accept: String,     // Example: "image/jpeg,image/png,image/gif"
    pub placeholder: String, // Displays prompt text.
    pub required: bool,     // Mandatory field.
    pub disabled: bool,     // Blocks access and modification of the element.
    pub readonly: bool,     // Specifies that the field cannot be modified by the user.
    pub is_hide: bool,      // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some numberString::new()#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // Warning information.
    pub errors: Vec<String>, // The value is determined automatically.
    pub group: u32, // To optimize field traversal in the `paladins/check()` method. Hint: It is recommended not to change.
}

impl Default for File {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            field_type: String::from("File"),
            input_type: String::from("file"),
            name: String::new(),
            value: None,
            default: None,
            media_root: String::from("./resources/media"),
            media_url: String::from("/media"),
            target_dir: String::from("files"),
            accept: String::new(),
            placeholder: String::new(),
            required: false,
            disabled: false,
            readonly: false,
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            errors: Vec::new(),
            group: 8,
        }
    }
}

impl File {
    pub fn get(&self) -> Option<FileData> {
        self.value.clone()
    }
    pub fn set(&mut self, file_path: &str, is_delete: bool, media_root: Option<&str>) {
        if Regex::new(r"(?:(?:/|\\)\d{4}\-\d{2}\-\d{2}\-barrel(?:/|\\))")
            .unwrap()
            .is_match(file_path)
        {
            Err(format!(
                "This file is not allowed to be reused - {file_path}"
            ))
            .unwrap()
        }
        let file_path = if !file_path.is_empty() {
            Self::copy_file_to_tmp(file_path, media_root).unwrap()
        } else {
            String::new()
        };
        self.value = Some(FileData {
            path: file_path,
            is_delete,
            ..Default::default()
        });
    }
    // Copy file to media_root}/tmp directory
    pub fn copy_file_to_tmp(
        file_path: &str,
        media_root: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        let media_root = if let Some(media_root) = media_root {
            media_root.to_string()
        } else {
            "./resources/media".to_string()
        };
        let f_path = Path::new(file_path);
        if !f_path.is_file() {
            Err(format!("File is missing - {file_path}"))?
        }
        let dir_tmp = format!("{media_root}/tmp");
        fs::create_dir_all(dir_tmp.clone())?;
        let f_name = Uuid::new_v4().to_string();
        let ext = f_path.extension().unwrap().to_str().unwrap();
        let f_tmp = format!("{dir_tmp}/{f_name}.{ext}");
        fs::copy(file_path, f_tmp.clone())?;
        Ok(f_tmp)
    }
}
