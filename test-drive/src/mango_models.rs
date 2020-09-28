use async_trait::async_trait;
use futures::stream::StreamExt;
use mango_orm::{
    create_model,
    forms::Form,
    models::Meta,
    widgets::{FieldType, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, oid::ObjectId, ser::to_document, Bson},
    options::UpdateModifications,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;

const SERVICE_NAME: &'static str = "account"; // SERVICE_NAME or APP_NAME or PROJECT_NAME etc...
const DATABASE_NAME: &'static str = "test_drive"; // SERVICE_NAME or APP_NAME or PROJECT_NAME etc...

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    #[derive(Serialize, Deserialize, Debug, Default)]
    struct Category {
        title: String
    }
}
#[async_trait]
impl Form for Category {
    // Example:
    // Customizing widgets by model fields
    fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>> {
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
        Ok(raw_attrs)
    }
}

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    #[derive(Serialize, Deserialize, Debug, Default)]
    struct User {
        username: String,
        email: String
    }
}
#[async_trait]
impl Form for User {
    // Example:
    // Customizing widgets by model fields
    fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>> {
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
        Ok(raw_attrs)
    }
}
