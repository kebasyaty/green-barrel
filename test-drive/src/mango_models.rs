use async_trait::async_trait;
use futures::stream::StreamExt;
use mango_orm::{
    create_model,
    models::{Meta, Model},
    widgets::{FieldType, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, Bson},
    options::UpdateModifications,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use serde_json;
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
    // Example:
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
    // Example:
    // Define (If necessary) HTML form for page templates
    fn form() -> String {
        let attrs: HashMap<String, Transport> = Self::form_attrs();
        let mut form_text = String::from("<form action=\"/\" method=\"GET\">");
        for (_, trans) in attrs {
            match trans.field_type.as_str() {
                "checkbox" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input type=\"{}\" id=\"{}\" name=\"{}\" {} class={} {}>",
                        form_text,
                        label,
                        trans.field_type,
                        trans.id,
                        trans.name,
                        if trans.checked { "checked" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "color" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input type=\"{}\" id=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.field_type,
                        trans.id,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                _ => panic!("Invalid input type."),
            }
        }
        format!("{}\n</form>", form_text)
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
    // Example:
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
