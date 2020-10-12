use argon2::{self, Config};
use futures::stream::StreamExt;
use mango_orm::{
    forms::{Form, OutputData, OutputType},
    model,
    models::{FormCache, Meta, FORM_CACHE},
    widgets::{FieldType, StepMinMax, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, oid::ObjectId, ser::to_document, Bson},
    options::UpdateModifications,
    results, Client, Collection, Cursor, Database,
};
use rand::Rng;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use validator::{
    validate_email, validate_ip, validate_ip_v4, validate_ip_v6, validate_range, validate_url,
    Validator,
};

const _SERVICE_NAME: &'static str = "account"; // _SERVICE_NAME or _APP_NAME or _PROJECT_NAME etc...
const _DATABASE_NAME: &'static str = "test_drive"; // _SERVICE_NAME or _APP_NAME or _PROJECT_NAME etc...

model! {
    _SERVICE_NAME,
    _DATABASE_NAME,

    struct Category {
        hash: String, // Required field
        title: String
    }

    impl Form for Category {
        // Example:
        // Customizing widgets by model fields
        // (For `hash` field, Widget is added automatically)
        fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>> {
            let mut map = HashMap::new();
            // Title
            map.insert(
                "title",
                Widget {
                    label: "Category Name".to_string(),
                    value: FieldType::InputText(String::new()),
                    maxlength: 40,
                    required: true,
                    hint: "Please enter Category name.".to_string(),
                    other_attrs: format!("placeholder=\"{}\"", "Category Name"),
                    ..Default::default()
                },
            );

            Ok(map)
        }
    }
}

model! {
    _SERVICE_NAME,
    _DATABASE_NAME,

    struct User {
        hash: String, // Required field
        username: String,
        email: String,
        password: String,
        password_confirm: String
    }

    impl Form for User {
        // Example:
        // List of field names that will not be saved to the database
        fn ignore_fields() -> Result<Vec<&'static str>, Box<dyn Error>> {
            let field_list = vec!["password_confirm"];
            Ok(field_list)
        }

        // Example:
        // Custom validation of model fields
        // (Don't forget to check for ignored fields -> `ignore_fields()`)
        fn custom_check(&self) -> Result<HashMap<&'static str, &'static str>, Box<dyn Error>> {
            // .insert("field_name", "Error message")
            let mut error_map = HashMap::new();

            if self.password_confirm.len() == 0 {
                error_map.insert("password_confirm", "Required field.");
            } else if self.password != self.password_confirm {
                error_map.insert("password_confirm", "Password confirmation does not match.");
            }
            Ok(error_map)
        }

        // Example:
        // Customizing widgets by model fields
        // (For `hash` field, Widget is added automatically)
        fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>> {
            let mut map = HashMap::new();
            // Username
            map.insert(
                "username",
                Widget {
                    label: "Your Name".to_string(),
                    value: FieldType::InputText("Rust".to_string()),
                    maxlength: 40,
                    min: StepMinMax::U32(3),
                    max: StepMinMax::U32(40),
                    required: true,
                    hint: "Please enter your real name.".to_string(),
                    other_attrs: format!("placeholder=\"{}\"", "Your Name"),
                    ..Default::default()
                },
            );
            // Email
            map.insert(
                "email",
                Widget {
                    label: "Your Email".to_string(),
                    value: FieldType::InputEmail(String::new()),
                    maxlength: 78,
                    required: true,
                    hint: "Enter your work email.".to_string(),
                    unique: true,
                    other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                    ..Default::default()
                },
            );
            // Password
            map.insert(
                "password",
                Widget {
                    label: "Your password".to_string(),
                    value: FieldType::InputPassword(String::new()),
                    maxlength: 32,
                    required: true,
                    hint: "Enter your password.".to_string(),
                    other_attrs: format!("placeholder=\"{}\"", "Your password"),
                    ..Default::default()
                },
            );
            // Confirm password
            map.insert(
                "password_confirm",
                Widget {
                    label: "Confirm password".to_string(),
                    value: FieldType::InputPassword(String::new()),
                    maxlength: 32,
                    required: true,
                    hint: "Confirm your password.".to_string(),
                    other_attrs: format!("placeholder=\"{}\"", "Confirm password"),
                    ..Default::default()
                },
            );

            Ok(map)
        }
    }
}
