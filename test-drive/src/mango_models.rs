use async_trait::async_trait;
use futures::stream::StreamExt;
use mango_orm::create_model;
use mango_orm::models::{Meta, Model};
use mango_orm::widgets::{FieldType, Widget};
use mongodb::{
    bson::{doc, document::Document},
    options::UpdateModifications,
    Client, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SERVICE_NAME: &'static str = "account"; // SERVICE_NAME or APP_NAME or PROJECT_NAME
const DATABASE_NAME: &'static str = "test_drive";

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    struct Category {
        title: String
    }
}
#[async_trait]
impl Model for Category {
    // Define attributes for widgets of fields
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "title",
            Widget {
                label: "Category Name".to_string(),
                value: FieldType::InputText(String::new()),
                maxlength: 40,
                hint: "Please enter Category name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Category Name"),
                ..Default::default()
            },
        );
        raw_attrs
    }
}

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    struct User {
        username: String,
        email: String
    }
}
#[async_trait]
impl Model for User {
    // Define attributes for widgets of fields
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your Name".to_string(),
                value: FieldType::InputText("Rust".to_string()),
                maxlength: 40,
                hint: "Please enter your real name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Your Name"),
                ..Default::default()
            },
        );
        raw_attrs.insert(
            "email",
            Widget {
                label: "Your Email".to_string(),
                value: FieldType::InputEmail(String::new()),
                maxlength: 78,
                hint: "Enter your work email.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                ..Default::default()
            },
        );
        raw_attrs
    }
}
