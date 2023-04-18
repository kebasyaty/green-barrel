use async_trait::async_trait;
use green_barrel::*;
use metamorphose::Model;
use mongodb::Client;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

// Get settings of service/sub-application.
use crate::settings::{
    service_name::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model(
    is_use_add_valid = true,
    is_use_hooks = true,
    ignore_fields = "confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    pub slug: Slug,
    pub first_name: InputText,
    pub last_name: InputText,
    pub email: InputEmail,
    pub phone: InputPhone,
    pub password: InputPassword,
    pub confirm_password: InputPassword,
    pub is_staff: CheckBox,
    pub is_active: CheckBox,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: InputText {
                label: "Username".into(),
                placeholder: "Enter your username".into(),
                maxlength: 150,
                required: true,
                unique: true,
                hint: "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150".into(),
                ..Default::default()
            },
            slug: Slug {
                label: "Slug".into(),
                hint: "To create a human readable url".into(),
                slug_sources: vec!["hash".into(), "username".into()],
                ..Default::default()
            },
            first_name: InputText {
                label: "First name".into(),
                placeholder: "Enter your First name".into(),
                maxlength: 150,
                ..Default::default()
            },
            last_name: InputText {
                label: "Last name".into(),
                placeholder: "Enter your Last name".into(),
                maxlength: 150,
                ..Default::default()
            },
            email: InputEmail {
                label: "E-mail".into(),
                placeholder: "Please enter your email".into(),
                required: true,
                unique: true,
                maxlength: 320,
                hint: "Your actual E-mail".into(),
                ..Default::default()
            },
            phone: InputPhone {
                label: "Phone number".into(),
                placeholder: "Please enter your phone number".into(),
                unique: true,
                maxlength: 30,
                hint: "Your actual phone number".into(),
                ..Default::default()
            },
            password: InputPassword {
                label: "Password".into(),
                placeholder: "Enter your password".into(),
                required: true,
                minlength: 8,
                hint: "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
                    .into(),
                ..Default::default()
            },
            confirm_password: InputPassword {
                label: "Confirm password".into(),
                placeholder: "Repeat your password".into(),
                required: true,
                minlength: 8,
                ..Default::default()
            },
            is_staff: CheckBox {
                label: "is staff?".into(),
                checked: Some(true),
                hint: "User can access the admin site?".into(),
                ..Default::default()
            },
            is_active: CheckBox {
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
        let hash = self.hash.get().unwrap_or_default();
        let password = self.password.get().unwrap_or_default();
        let confirm_password = self.confirm_password.get().unwrap_or_default();
        let username = self.username.get().unwrap_or_default();

        // Fields validation
        if hash.is_empty() && password != confirm_password {
            error_map.insert(
                "confirm_password".into(),
                "Password confirmation does not match.".into(),
            );
        }
        if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
            .case_insensitive(true)
            .build()
            .unwrap()
            .is_match(username.as_str())
        {
            error_map.insert(
                "username".into(),
                "Invalid characters present.<br>\
                 Valid characters: a-z A-Z 0-9 _ @ + ."
                    .into(),
            );
        }

        Ok(error_map)
    }
}

#[async_trait(?Send)]
impl Hooks for User {
    async fn pre_create(&self, _client: &Client) {
        println!("!!!Pre Create!!!");
    }
    //
    async fn post_create(&self, _client: &Client) {
        println!("!!!Post Create!!!");
    }
    //
    async fn pre_update(&self, _client: &Client) {
        println!("!!!Pre Update!!!");
    }
    //
    async fn post_update(&self, _client: &Client) {
        println!("!!!Post Update!!!");
    }
    //
    async fn pre_delete(&self, _client: &Client) {
        println!("!!!Pre Delet!!!");
    }
    //
    async fn post_delete(&self, _client: &Client) {
        println!("!!!Post Delet!!!");
    }
}
