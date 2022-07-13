use mango_orm::*;
use metamorphose::Model;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};

// Get settings of service/sub-application.
use crate::settings::{
    default::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
    PROJECT_NAME, UNIQUE_PROJECT_KEY,
};

#[Model(
    is_del_docs = false,
    is_use_add_valid = true,
    is_use_hooks = true,
    is_use_custom_html = true,
    ignore_fields = "confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserProfile {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "Username",
        placeholder = "Enter your username",
        unique = true,
        required = true,
        maxlength = 150,
        hint = "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150"
    )]
    pub username: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputSlug",
        label = "Slug",
        unique = true,
        readonly = true,
        is_hide = true,
        hint = "To create a human readable url",
        slug_sources = r#"["hash", "username"]"#
    )]
    pub slug: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "First name",
        placeholder = "Enter your First name",
        maxlength = 150
    )]
    pub first_name: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "Last name",
        placeholder = "Enter your Last name",
        maxlength = 150
    )]
    pub last_name: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputEmail",
        label = "E-mail",
        placeholder = "Please enter your email",
        required = true,
        unique = true,
        maxlength = 320,
        hint = "Your actual E-mail"
    )]
    pub email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputPhone",
        label = "Phone number",
        placeholder = "Please enter your phone number",
        unique = true,
        maxlength = 30,
        hint = "Your actual phone number"
    )]
    pub phone: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputPassword",
        label = "Password",
        placeholder = "Enter your password",
        required = true,
        minlength = 8,
        hint = "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
    )]
    pub password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputPassword",
        label = "Confirm password",
        placeholder = "Repeat your password",
        required = true,
        minlength = 8,
        hint = "Repeat your password"
    )]
    pub confirm_password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is staff?",
        checked = true,
        hint = "User can access the admin site?"
    )]
    pub is_staff: Option<bool>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is active?",
        checked = true,
        hint = "Is this an active account?"
    )]
    pub is_active: Option<bool>,
}

impl AdditionalValidation for UserProfile {
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();

        // Get clean data
        let hash = self.hash.clone().unwrap_or_default();
        let password = self.password.clone().unwrap_or_default();
        let confirm_password = self.confirm_password.clone().unwrap_or_default();
        let username = self.username.clone().unwrap_or_default();

        // Fields validation
        if hash.is_empty() && password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
            .case_insensitive(true)
            .build()
            .unwrap()
            .is_match(username.as_str())
        {
            error_map.insert(
                "username",
                "Invalid characters present.<br>\
                 Valid characters: a-z A-Z 0-9 _ @ + .",
            );
        }

        Ok(error_map)
    }
}

impl Hooks for UserProfile {
    fn pre_create(&self) {
        println!("!!!Pre Create!!!");
    }
    //
    fn post_create(&self) {
        println!("!!!Post Create!!!");
    }
    //
    fn pre_update(&self) {
        println!("!!!Pre Update!!!");
    }
    //
    fn post_update(&self) {
        println!("!!!Post Update!!!");
    }
    //
    fn pre_delete(&self) {
        println!("!!!Pre Delet!!!");
    }
    //
    fn post_delete(&self) {
        println!("!!!Post Delet!!!");
    }
}

impl GenerateHtml for UserProfile {}

#[Model]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dynamic {
    // text
    #[serde(default)]
    #[field_attrs(widget = "selectTextDyn")]
    pub select_text_dyn: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "selectTextMultDyn")]
    pub select_text_mult_dyn: Option<Vec<String>>,
    // i32
    #[serde(default)]
    #[field_attrs(widget = "selectI32Dyn")]
    pub select_i32_dyn: Option<i32>,
    #[serde(default)]
    #[field_attrs(widget = "selectI32MultDyn")]
    pub select_i32_mult_dyn: Option<Vec<i32>>,
    // u32
    #[serde(default)]
    #[field_attrs(widget = "selectU32Dyn")]
    pub select_u32_dyn: Option<u32>,
    #[serde(default)]
    #[field_attrs(widget = "selectU32MultDyn")]
    pub select_u32_mult_dyn: Option<Vec<u32>>,
    // i64
    #[serde(default)]
    #[field_attrs(widget = "selectI64Dyn")]
    pub select_i64_dyn: Option<i64>,
    #[serde(default)]
    #[field_attrs(widget = "selectI64MultDyn")]
    pub select_i64_mult_dyn: Option<Vec<i64>>,
    // f64
    #[serde(default)]
    #[field_attrs(widget = "selectF64Dyn")]
    pub select_f64_dyn: Option<f64>,
    #[serde(default)]
    #[field_attrs(widget = "selectF64MultDyn")]
    pub select_f64_mult_dyn: Option<Vec<f64>>,
}
