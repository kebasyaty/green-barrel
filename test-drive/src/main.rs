use mango_orm::forms::Form;
use mango_orm::models::Model;
use mango_orm::widgets::{DataType, FieldType, Widget};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CategoryName {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub categories: Vec<String>,
}

impl Model for User {
    //
}

impl Form for User {
    fn raw_attrs(&self) -> HashMap<&'static str, Widget> {
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
                label: "Your Email".to_string(),
                field_type: FieldType::ManyToMany,
                relation_model_name: "CategoryName",
                hidden: true,
                hint: "Test all attrs.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Test all attrs"),
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

fn main() {
    let test_model = User {
        username: "Some text".to_string(),
        email: "some@some.net".to_string(),
        categories: vec![
            "id-1".to_string(),
            "id-2".to_string(),
            "id-3".to_string(),
            "id-4".to_string(),
        ],
    };

    println!("{:?}", test_model.form_attrs());
}
