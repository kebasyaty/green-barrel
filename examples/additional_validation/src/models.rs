use async_trait::async_trait;
use green_barrel::*;
use metamorphose::Model;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model(
    is_use_add_valid = true,
    ignore_fields = "confirm_password" // Example: "field_name, field_name_2"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: Text,
    pub slug: Slug,
    pub first_name: Text,
    pub last_name: Text,
    pub email: Email,
    pub phone: Phone,
    pub password: Password,
    pub confirm_password: Password,
    pub is_staff: Bool,
    pub is_active: Bool,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: Text {
                label: "Username".into(),
                placeholder: "Enter your username".into(),
                regex: r"^[a-zA-Z\d_@.+]{1,150}$".into(),
                regex_err_msg: "Allowed chars: a-z A-Z 0-9 _ @ . +".into(),
                minlength: 1,
                maxlength: 150,
                required: true,
                unique: true,
                hint: "Allowed chars: a-z A-Z 0-9 _ @ . +".into(),
                ..Default::default()
            },
            slug: Slug {
                label: "Slug".into(),
                unique: true,
                readonly: true,
                hint: "To create a human readable url".into(),
                slug_sources: vec!["hash".into(), "username".into()],
                ..Default::default()
            },
            first_name: Text {
                label: "First name".into(),
                placeholder: "Enter your First name".into(),
                maxlength: 150,
                ..Default::default()
            },
            last_name: Text {
                label: "Last name".into(),
                placeholder: "Enter your Last name".into(),
                maxlength: 150,
                ..Default::default()
            },
            email: Email {
                label: "E-mail".into(),
                placeholder: "Please enter your email".into(),
                required: true,
                unique: true,
                maxlength: 320,
                hint: "Your actual E-mail".into(),
                ..Default::default()
            },
            phone: Phone {
                label: "Phone number".into(),
                placeholder: "Please enter your phone number".into(),
                unique: true,
                hint: "Your actual phone number".into(),
                ..Default::default()
            },
            password: Password {
                label: "Password".into(),
                placeholder: "Enter your password".into(),
                required: true,
                hint: "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (".into(),
                ..Default::default()
            },
            confirm_password: Password {
                label: "Confirm password".into(),
                placeholder: "Repeat your password".into(),
                required: true,
                ..Default::default()
            },
            is_staff: Bool {
                label: "is staff?".into(),
                checked: Some(true),
                hint: "User can access the admin site?".into(),
                ..Default::default()
            },
            is_active: Bool {
                label: "is active?".into(),
                checked: Some(true),
                hint: "Is this an active account?".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[async_trait(?Send)]
impl AdditionalValidation for User {
    async fn add_validation(
        &self,
        _client: &Client,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map = HashMap::<String, String>::new();

        // Get clean data
        let password = self.password.get().unwrap_or_default();
        let confirm_password = self.confirm_password.get().unwrap_or_default();

        // Fields validation
        if (!password.is_empty() && !confirm_password.is_empty()) && password != confirm_password {
            error_map.insert(
                "confirm_password".into(),
                "Password confirmation does not match.".into(),
            );
        }

        Ok(error_map)
    }
}
