use mango_orm::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

// Simulate forwarding from application settings
const SERVICE_NAME: &str = "service_name";
const DATABASE_NAME: &str = "database_name";
const DB_CLIENT_NAME: &str = "default";

#[Model]
#[derive(Serialize, Deserialize, Default)]
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
#[derive(Serialize, Deserialize, Default)]
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
}

// Methods for additional validation.
// **For custom use, add the Model attribute `is_use_add_valid = true`.
// ( Remember to use for validate of ignored fields )
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
