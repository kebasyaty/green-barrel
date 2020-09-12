use async_trait::async_trait;
use mango_orm::model_info;
use mango_orm::models::{Meta, Model};
use mango_orm::widgets::{DataType, FieldType, Widget};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const DATABASE: &str = "dev_db";

model_info! {
    struct Category {
        title: String
    }
}
#[async_trait]
impl Model for Category {
    // Metadata
    fn meta() -> Meta {
        Meta {
            database: DATABASE,
            collection: "category_name",
        }
    }
    //
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "title",
            Widget {
                label: "Category Name".to_string(),
                field_type: FieldType::InputText,
                value: DataType::Text(String::new()),
                maxlength: 40,
                hint: "Please enter Category name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Category Name"),
                ..Default::default()
            },
        );
        raw_attrs
    }
}

model_info! {
    struct User {
        username: String,
        email: String,
        categories: Vec<String>
    }
}
#[async_trait]
impl Model for User {
    // Metadata
    fn meta() -> Meta {
        Meta {
            database: DATABASE,
            collection: "user",
        }
    }
    //
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your Name".to_string(),
                field_type: FieldType::InputText,
                value: DataType::Text("Rust".to_string()),
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
                field_type: FieldType::InputEmail,
                maxlength: 78,
                hint: "Enter your work email.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                ..Default::default()
            },
        );
        raw_attrs.insert(
            "categories",
            Widget {
                label: "Select Categories".to_string(),
                field_type: FieldType::ManyToMany,
                relation_model: Category::meta().collection.to_string(),
                hidden: true,
                hint: "Test all attrs.".to_string(),
                unique: true,
                other_attrs: format!("multiple placeholder=\"{}\"", "Select Categories"),
                some_classes: "class-name class-name-2".to_string(),
                select: vec![
                    ("Doc name 1".to_string(), DataType::Text("id-1".to_string())),
                    ("Doc name 2".to_string(), DataType::Text("id-2".to_string())),
                    ("Doc name 3".to_string(), DataType::Text("id-3".to_string())),
                    ("Doc name 4".to_string(), DataType::Text("id-4".to_string())),
                ],
                ..Default::default()
            },
        );
        raw_attrs
    }
}
