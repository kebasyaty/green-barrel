//! # Forms
//!
//! `ToForm` - Define form settings for models (widgets, html).
//! `Widget` - Form controls parameters.
//! `OutputData` - Output data for the `check()` and `save()` methods.
//! `TransMapWidgetType` - For transporting of Widget types map to implementation of methods.
//! `TransMapWidgets` - For transporting of Widgets map to implementation of methods.
//! `HtmlControls` - Rendering HTML-controls code for Form.
//! ( If necessary, customize the code generation yourself using html and css from Bootstrap, Material Design, etc. )
//!

pub mod html_controls;
pub mod output_data;

// FORMS
// #################################################################################################
// Data structures for `inputFile` and `inputImage` widgets
// *************************************************************************************************
#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
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

#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct ImageData {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // in bytes
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub width: u32, // in pixels
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub height: u32, // in pixels
}

// Widget
// ( Form controls parameters )
// *************************************************************************************************
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct Widget {
    pub id: String, // "model-name--field-name" ( The value is determined automatically )
    pub label: String,
    pub widget: String,
    pub input_type: String, // The value is determined automatically
    pub name: String,       // The value is determined automatically
    pub value: String,
    pub placeholder: String,
    pub pattern: String, // Validating a field using a client-side regex
    pub minlength: usize,
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub unique: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub step: String,
    pub min: String,
    pub max: String,
    pub other_attrs: String, // "autofocus multiple size=\"some number\" ..."
    pub css_classes: String, // "class-name class-name ..."
    pub options: Vec<(String, String)>, // <value, Title>
    pub hint: String,
    pub warning: String,    // The value is determined automatically
    pub error: String,      // The value is determined automatically
    pub common_msg: String, // Messages common to the entire Form
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
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            checked: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: String::from("0"),
            min: String::from("0"),
            max: String::from("0"),
            other_attrs: String::new(),
            css_classes: String::new(),
            options: Vec::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            common_msg: String::new(),
        }
    }
}

// For transporting of Widgets map to implementation of methods
// <field name, Widget>
#[derive(serde::Deserialize)]
pub struct TransMapWidgets {
    pub map_widgets: std::collections::HashMap<String, Widget>,
}

// Form settings
// *************************************************************************************************
pub trait ToForm {
    // Get a store key
    // ( key = collection name, used in forms exclusively for store access )
    // ---------------------------------------------------------------------------------------------
    fn key_store() -> Result<String, Box<dyn std::error::Error>>;

    // Get map of widgets for Form fields
    // <field name, Widget>
    fn widgets() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>>;
}
