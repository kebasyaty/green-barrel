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
    is_use_addition = true,
    ignore_fields = "confirm_password" // Example: "field_name, field_name_2"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: TextField,
    pub slug: SlugField,
    pub first_name: TextField,
    pub last_name: TextField,
    pub email: EmailField,
    pub phone: PhoneField,
    pub password: PasswordField,
    pub confirm_password: PasswordField,
    pub is_staff: BoolField,
    pub is_active: BoolField,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            username: TextField {
                label: "Username".into(),
                placeholder: "Enter your username".into(),
                regex: r"^[a-zA-Z\d_@.+]{1,150}$".into(),
                regex_err_msg: t!("allowed_chars", chars = "a-z A-Z 0-9 _ @ . +"),
                minlength: 1,
                maxlength: 150,
                required: true,
                unique: true,
                hint: t!("allowed_chars", chars = "a-z A-Z 0-9 _ @ . +"),
                ..Default::default()
            },
            slug: SlugField {
                label: "Slug".into(),
                unique: true,
                readonly: true,
                hint: "To create a human readable url".into(),
                slug_sources: vec!["hash".into(), "username".into()],
                ..Default::default()
            },
            first_name: TextField {
                label: "First name".into(),
                placeholder: "Enter your First name".into(),
                maxlength: 150,
                ..Default::default()
            },
            last_name: TextField {
                label: "Last name".into(),
                placeholder: "Enter your Last name".into(),
                maxlength: 150,
                ..Default::default()
            },
            email: EmailField {
                label: "E-mail".into(),
                placeholder: "Please enter your email".into(),
                required: true,
                unique: true,
                maxlength: 320,
                hint: "Your actual E-mail".into(),
                ..Default::default()
            },
            phone: PhoneField {
                label: "Phone number".into(),
                placeholder: "Please enter your phone number".into(),
                unique: true,
                hint: "Your actual phone number".into(),
                ..Default::default()
            },
            password: PasswordField {
                label: "Password".into(),
                placeholder: "Enter your password".into(),
                required: true,
                hint: "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (".into(),
                ..Default::default()
            },
            confirm_password: PasswordField {
                label: "Confirm password".into(),
                placeholder: "Repeat your password".into(),
                required: true,
                ..Default::default()
            },
            is_staff: BoolField {
                label: "is staff?".into(),
                default: Some(true),
                hint: "User can access the admin site?".into(),
                ..Default::default()
            },
            is_active: BoolField {
                label: "is active?".into(),
                default: Some(true),
                hint: "Is this an active account?".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[async_trait(?Send)]
impl Addition for User {
    // It is intended for additional actions with fields.
    // Hint: This method is execute first.
    async fn add_actions(&mut self, _client: &Client) -> Result<(), Box<dyn Error>> {
        // Get clean data.
        let username = self.username.get().unwrap_or_default();
        // Field processing.
        self.username.set(&username.to_uppercase());
        Ok(())
    }
    // It is supposed to be use to additional validation of fields.
    // Hint: This method is execute second.
    async fn add_validation(
        &self,
        _client: &Client,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map = HashMap::<String, String>::new();

        // Get clean data.
        let password = self.password.get().unwrap_or_default();
        let confirm_password = self.confirm_password.get().unwrap_or_default();

        // Fields validation.
        if (!password.is_empty() && !confirm_password.is_empty()) && password != confirm_password {
            error_map.insert("confirm_password".into(), t!("password_mismatch"));
        }

        Ok(error_map)
    }
}
