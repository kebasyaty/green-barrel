//use argon2::{self, Config};
//use chrono::{DateTime, NaiveDateTime, Utc};
use futures::stream::StreamExt;
use mango_orm::{
    forms::{Form, OutputData, OutputType},
    model,
    models::{Meta, Model},
    store::{
        FormCache, FORM_CACHE, REGEX_IS_COLOR_CODE, REGEX_IS_DATE, REGEX_IS_DATETIME,
        REGEX_IS_PASSWORD,
    },
    widgets::{DataType, FieldType, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, oid::ObjectId, ser::to_document, Bson},
    options::UpdateModifications,
    results, Client, Collection, Cursor, Database,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use validator::{
    validate_email, validate_ip, validate_ip_v4, validate_ip_v6, validate_range, validate_url,
    Validator,
};

const _SERVICE_NAME: &str = "account"; // _SERVICE_NAME or _APP_NAME or _PROJECT_NAME etc...
const _DATABASE_NAME: &str = "test_drive"; // _SERVICE_NAME or _APP_NAME or _PROJECT_NAME etc...

model! {
    struct User {
        hash: String, // Required field
        username: String,
        email: String,
        password: String,
        password_confirm: String,
        datetime: String,
        date: String
    }

    impl Model for User {
        // Example:
        // Metadata (database name, collection name, etc)
        fn meta<'a>() -> Result<Meta<'a>, Box<dyn Error>> {
            Ok(Meta {
                service: _SERVICE_NAME.to_string(),
                database: _DATABASE_NAME.to_string(),
                // List of field names that will not be saved to the database
                ignore_fields: vec!["password_confirm"],
                ..Default::default()
            })
        }

        // Example:
        // Custom validation of model fields
        // (Don't forget to check for ignored fields -> `ignore_fields()`)
        fn custom_check<'a>(&self) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
            // .insert("field_name", "Error message")
            let mut error_map = HashMap::new();

            if self.password != self.password_confirm {
                error_map.insert("password_confirm", "Password confirmation does not match.");
            }
            Ok(error_map)
        }
    }

    impl Form for User {
        // Example:
        // Customizing widgets by model fields
        // (For `hash` field, Widget is added automatically)
        fn widgets<'a>() -> Result<HashMap<&'a str, Widget>, Box<dyn Error>> {
            let mut map = HashMap::new();
            // Username
            map.insert(
                "username",
                Widget {
                    label: "Your Name".to_string(),
                    value: FieldType::InputText("Rust".to_string()),
                    maxlength: 40,
                    min: DataType::U32(3),
                    max: DataType::U32(40),
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
            // Date and Time
            map.insert(
                "datetime",
                Widget {
                    label: "Date and Time".to_string(),
                    value: FieldType::InputDateTime(String::new()),
                    required: true,
                    hint: "Enter date and time.".to_string(),
                    unique: true,
                    min: DataType::Text(String::new()),
                    max: DataType::Text(String::new()),
                    other_attrs: format!("placeholder=\"{}\"", "Enter date and time"),
                    ..Default::default()
                },
            );
            // Date
                map.insert(
                "date",
                Widget {
                    label: "Date".to_string(),
                    value: FieldType::InputDate(String::new()),
                    required: true,
                    hint: "Enter date.".to_string(),
                    unique: true,
                    min: DataType::Text(String::new()),
                    max: DataType::Text(String::new()),
                    other_attrs: format!("placeholder=\"{}\"", "Enter date"),
                    ..Default::default()
                },
            );
            //
            Ok(map)
        }
    }
}
