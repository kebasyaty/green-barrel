use mango_orm::*;
use metamorphose::{Form, Model};
use serde::{Deserialize, Serialize};

// Get settings (service) of the application.
use crate::settings::{
    app_name::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
    KEYWORD,
};

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

#[Model]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        default = "Some text",
        required = false,
        minlength = 3,
        maxlength = 40
    )]
    pub username: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, unique = true, maxlength = 74)]
    pub email: Option<String>,
}

#[Model(
    is_use_add_valid = true,
    db_client_name = "default_2",
    ignore_fields = "confirm_email, confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserProfile {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        default = "Some text",
        required = false,
        minlength = 3,
        maxlength = 40
    )]
    pub username: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, unique = true, maxlength = 74)]
    pub email: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, maxlength = 74)]
    pub confirm_email: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub password: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub confirm_password: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputDate")]
    pub date: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputDateTime")]
    pub datetime: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "numberI32")]
    pub num_i32: Option<i32>,
    #[serde(default)]
    #[field_attrs(widget = "numberU32")]
    pub num_u32: Option<u32>,
    #[serde(default)]
    #[field_attrs(widget = "numberI64")]
    pub num_i64: Option<i64>,
    #[serde(default)]
    #[field_attrs(widget = "numberF64")]
    pub num_f64: Option<f64>,
    #[serde(default)]
    #[field_attrs(
        widget = "inputFile",
        default = r#"{
            "path":"./test-drive/media/hello_world.odt",
            "url":"/media/hello_world.odt"
        }"#
    )]
    pub file: Option<String>,
    #[serde(default)]
    #[field_attrs(
        widget = "inputImage",
        default = r#"{
            "path":"./test-drive/media/no-image-found.png",
            "url":"/media/no-image-found.png"
        }"#
    )]
    pub image: Option<String>,
}

// Methods for additional validation.
// Hint: For custom use, add the Model attribute `is_use_add_valid = true`.
// Hint: Remember to use for validate of ignored fields.
impl AdditionalValidation for UserProfile {
    // Example of additional validation for ignored fields
    // ---------------------------------------------------------------------------------------------
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // .insert("field_name", "Error message")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        // Get clean data
        let email = self.email.clone().unwrap();
        let confirm_email = self.confirm_email.clone().unwrap();
        let password = self.password.clone().unwrap();
        let confirm_password = self.confirm_password.clone().unwrap();
        // Validation of fields
        if email != confirm_email {
            error_map.insert(
                "confirm_email",
                "Email address confirmation does not match.",
            );
        }
        if password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        Ok(error_map)
    }
}

#[Form(is_use_add_valid = true)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserForm {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        default = "Some text",
        required = false,
        minlength = 3,
        maxlength = 40
    )]
    pub username: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, unique = true, maxlength = 74)]
    pub email: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, maxlength = 74)]
    pub confirm_email: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub password: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub confirm_password: Option<String>,
}

// Methods for additional validation.
// Hint: For custom use, add the Model attribute `is_use_add_valid = true`.
impl AdditionalValidation for UserForm {
    // Example of additional validation for ignored fields
    // ---------------------------------------------------------------------------------------------
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // .insert("field_name", "Error message")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        // Get clean data
        let email = self.email.clone().unwrap();
        let confirm_email = self.confirm_email.clone().unwrap();
        let password = self.password.clone().unwrap();
        let confirm_password = self.confirm_password.clone().unwrap();
        // Validation of fields
        if email != confirm_email {
            error_map.insert(
                "confirm_email",
                "Email address confirmation does not match.",
            );
        }
        if password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        Ok(error_map)
    }
}
