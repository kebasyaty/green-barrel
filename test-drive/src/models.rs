use green_barrel::*;
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
    ignore_fields = "confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserProfile {
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
}

impl Creator for UserProfile {
    fn custom_default() -> Self {
        Self {
            username: InputText {
                label: String::from("Username"),
                placeholder: String::from("Enter your username"),
                maxlength: 150,
                required: true,
                unique: true,
                is_hide: String::from("Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150"),
                ..Default::default()
            },
            slug: AutoSlug {
                label: String::from("Slug"),
                unique: true,
                readonly: true,
                hint: String::from("To create a human readable url"),
                slug_sources: vec![String::from("hash"), String::from("username")],
                ..Default::default()
            },
            first_name: InputText {
                label: String::from("First name"),
                placeholder: String::from("Enter your First name"),
                maxlength: 150,
                ..Default::default()
            },
            last_name: InputText {
                label: String::from("Last name"),
                placeholder: String::from("Enter your Last name"),
                maxlength: 150,
                ..Default::default()
            },
            email: InputEmail {
                label: String::from("E-mail"),
                placeholder: String::from("Please enter your email"),
                required: true,
                unique: true,
                maxlength: 320,
                hint: String::from("Your actual E-mail"),
                ..Default::default()
            },
            phone: InputPhone {
                label: String::from("Phone number"),
                placeholder: String::from("Please enter your phone number"),
                unique: true,
                maxlength: 30,
                hint: String::from("Your actual phone number"),
                ..Default::default()
            },
            password: InputPassword {
                label: String::from("Password"),
                placeholder: String::from("Enter your password"),
                required: true,
                minlength: 8,
                hint: String::from(
                    "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8",
                ),
                ..Default::default()
            },
            confirm_password: InputPassword {
                label: String::from("Confirm password"),
                placeholder: String::from("Repeat your password"),
                required: true,
                minlength: 8,
                ..Default::default()
            },
            is_staff: CheckBox {
                label: String::from("is staff?"),
                hint: String::from("User can access the admin site?"),
                ..Default::default()
            },
            is_active: CheckBox {
                label: String::from("is active?"),
                hint: String::from("Is this an active account?"),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl AdditionalValidation for UserProfile {
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();

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
