use green_barrel::*;
use metamorphose::Model;
use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

// Get settings of service/sub-application.
use crate::settings::{
    default::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
    PROJECT_NAME, UNIQUE_PROJECT_KEY,
};

#[Model(
    is_use_add_valid = true,
    is_use_hooks = true,
    ignore_fields = "confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    pub slug: AutoSlug,
    pub first_name: InputText,
    pub last_name: InputText,
    pub email: InputEmail,
    pub phone: InputPhone,
    pub password: InputPassword,
    pub confirm_password: InputPassword,
    pub is_staff: CheckBox,
    pub is_active: CheckBox,
    //
    pub file: InputFile,
    pub image: InputImage,
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
            slug: AutoSlug {
                label: "Slug".into(),
                unique: true,
                readonly: true,
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
            //
            file: InputFile {
                default: Some(FileData {
                    path: "./media/hello_world.odt".into(),
                    url: "/media/hello_world.odt".into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            image: InputImage {
                default: Some(ImageData {
                    path: "./media/no-image-found.png".into(),
                    url: "/media/no-image-found.png".into(),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl AdditionalValidation for User {
    fn add_validation<'a>(&self) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map = HashMap::<&'a str, &'a str>::new();

        // Get clean data
        let hash = self.hash.value.clone().unwrap_or_default();
        let password = self.password.value.clone().unwrap_or_default();
        let confirm_password = self.confirm_password.value.clone().unwrap_or_default();
        let username = self.username.value.clone().unwrap_or_default();

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

impl Hooks for User {
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
