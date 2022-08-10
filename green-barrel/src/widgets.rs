//! For control of fields on the server and client side.

pub mod check_box;
pub mod hidden_i32;
pub mod hidden_text;
pub mod hidden_u32;
pub mod input_color;
pub mod input_date;
pub mod input_date_time;
pub mod input_email;
pub mod input_file;
pub mod input_image;
pub mod input_ip;
pub mod input_ipv4;
pub mod input_ipv6;
pub mod input_password;
pub mod input_phone;
pub mod input_slug;
pub mod input_text;
pub mod input_url;
pub mod number_f64;
pub mod number_i32;
pub mod number_i64;
pub mod number_u32;
pub mod radio_f64;
pub mod radio_i32;
pub mod radio_i64;
pub mod radio_text;
pub mod radio_u32;
pub mod range_f64;
pub mod range_i32;
pub mod range_i64;
pub mod range_u32;
pub mod select_f64;
pub mod select_f64_dyn;
pub mod select_f64_mult;
pub mod select_f64_mult_dyn;
pub mod select_i32;
pub mod select_i32_dyn;
pub mod select_i32_mult;
pub mod select_i32_mult_dyn;
pub mod select_i64;
pub mod select_i64_dyn;
pub mod select_i64_mult;
pub mod select_i64_mult_dyn;
pub mod select_text;
pub mod select_text_dyn;
pub mod select_text_mult;
pub mod select_text_mult_dyn;
pub mod select_u32;
pub mod select_u32_dyn;
pub mod select_u32_mult;
pub mod select_u32_mult_dyn;
pub mod text_area;

use core::fmt::Debug;
use serde::{Deserialize, Serialize};

// CONTROLS FOR MODEL FIELDS.
// #################################################################################################
//
/// Field attributes.
// *************************************************************************************************
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Widget {
    pub id: String, // The value is determined automatically. Format: "model-name--field-name".
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
    pub options: Vec<(String, String)>, // <option value="value1">Title 1</option> - Example: r#"[[1,"Volvo"], [2,"Saab"]]"#.
    pub thumbnails: Vec<(String, u32)>, // From one to four inclusive. Example: r#"[["xs",150],["sm",300],["md",600],["lg",1200]]"#. Hint: An Intel i7-4770 processor or better is recommended.
    pub slug_sources: Vec<String>, // Example: r#"["title"]"# or r#"["hash", "username"]"# or r#"["email", "first_name", "last_name"]"#.
    pub is_hide: bool,             // Hide field from user.
    pub other_attrs: String, // Example: r# "autofocus tabindex="some number" size="some number""#.
    pub css_classes: String, // Example: "class-name-1 class-name-2".
    pub hint: String,        // Additional explanation for the user.
    pub warning: String,     // The value is determined automatically.
    pub error: String,       // The value is determined automatically.
    pub alert: String, // Alert message for the entire web form. The value is determined automatically.
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
            alert: String::new(),
        }
    }
}
