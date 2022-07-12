//! For control of fields on the server and client side.

pub mod generate_html_code;
pub mod output_data;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// CONTROLS FOR MODEL FIELDS.
// #################################################################################################
/// Helper structures for inputFile widgets.
// *************************************************************************************************
#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct FileData {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // in bytes
}

/// Helper structures for inputImage widgets.
#[derive(Default, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ImageData {
    #[serde(default)]
    pub path: String, // max size = original
    #[serde(default)]
    pub path_xs: String,
    #[serde(default)]
    pub path_sm: String,
    #[serde(default)]
    pub path_md: String,
    #[serde(default)]
    pub path_lg: String,
    #[serde(default)]
    pub url: String, // max size = original
    #[serde(default)]
    pub url_xs: String,
    #[serde(default)]
    pub url_sm: String,
    #[serde(default)]
    pub url_md: String,
    #[serde(default)]
    pub url_lg: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // in bytes
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub width: u32, // in pixels
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub height: u32, // in pixels
}

/// Field attributes.
// *************************************************************************************************
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Widget {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name"
    pub label: String, // Web form field name.
    pub widget: String, // Widget name.
    pub input_type: String, // The value is determined automatically.
    pub name: String, // The value is determined automatically.
    pub value: String, // Default value.
    pub accept: String, // Example: "image/jpeg,image/png,image/gif"
    pub placeholder: String, // Displays prompt text.
    pub pattern: String, // Validating a field using a client-side regex (Only for text, search, tel, url, email, and password controls).
    pub minlength: usize, // The minimum number of characters allowed in the text.
    pub maxlength: usize, // The maximum number of characters allowed in the text.
    pub required: bool,  // Mandatory field.
    pub checked: bool,   // A pre-activated radio button or checkbox.
    pub unique: bool,    // The unique value of a field in a collection.
    pub disabled: bool,  // Blocks access and modification of the element.
    pub readonly: bool,  // Specifies that the field cannot be modified by the user.
    pub step: String,    // Increment step for numeric fields.
    pub min: String,     // The lower value for entering a number or date.
    pub max: String,     // The top value for entering a number or date.
    pub options: Vec<(String, String)>, // <option value="value1">Title 1</option> - Example: r#"[[1,"Volvo"], [2,"Saab"]]"#
    pub thumbnails: Vec<(String, u32)>, // From one to four inclusive. Example: r#"[["xs",150],["sm",300],["md",600],["lg",1200]]"#
    pub slug_sources: Vec<String>, // Example: r#"["title"]"# or r#"["hash", "username"]"# or r#"["email", "first_name", "last_name"]"#
    pub is_hide: bool,             // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some number""#
    pub css_classes: String, // Example: "class-name-1 class-name-2"
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub common_msg: String, // Messages common to the entire Form. The value is determined automatically.
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            id: String::new(),
            label: String::new(),
            widget: String::from("inputText"),
            input_type: String::from("text"),
            name: String::new(),
            value: String::new(),
            accept: String::new(),
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            checked: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: String::from("1"),
            min: String::new(),
            max: String::new(),
            options: Vec::new(),
            thumbnails: Vec::new(),
            slug_sources: Vec::new(),
            is_hide: false,
            other_attrs: String::new(),
            css_classes: String::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            common_msg: String::new(),
        }
    }
}

/// For transporting of Widgets map to implementation of methods.
/// Hint: <field name, Widget>
#[derive(Deserialize)]
pub struct TransMapWidgets {
    pub map_widgets: HashMap<String, Widget>,
}

// Enums for the HTML-controls module.
// *************************************************************************************************
/// The HTTP method to submit the form with. Possible (case insensitive) values: GET and POST.
/// Default -> HttpMethod::GET
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl HttpMethod {
    pub fn value(&self) -> String {
        match self {
            HttpMethod::GET => String::from("get"),
            HttpMethod::POST => String::from("post"),
        }
    }
}

/// If the value of the method attribute is post, enctype is the MIME type of the form submission.
/// Possible values: application/x-www-form-urlencoded | multipart/form-data | text/plain.
/// Default -> Enctype::Application
#[derive(Debug)]
pub enum Enctype {
    Application,
    Multipart,
    Text,
}

impl Default for Enctype {
    fn default() -> Self {
        Enctype::Application
    }
}

impl Enctype {
    pub fn value(&self) -> String {
        match self {
            Enctype::Application => String::from("application/x-www-form-urlencoded"),
            Enctype::Multipart => String::from("multipart/form-data"),
            Enctype::Text => String::from("text/plain"),
        }
    }
}
