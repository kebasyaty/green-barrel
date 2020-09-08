use mango_orm::models::{Meta, Model};
use mango_orm::widgets::{DataType, FieldType, Widget};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod settings;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Category {
    pub title: String,
}
impl Model for Category {
    // Metadata
    fn meta() -> Meta {
        Meta {
            db_name: settings::DB_NAME,
            collection: "category_name",
        }
    }
    //
    fn raw_attrs(&self) -> HashMap<&'static str, Widget> {
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub categories: Vec<String>,
}
impl Model for User {
    // Metadata
    fn meta() -> Meta {
        Meta {
            db_name: settings::DB_NAME,
            collection: "user",
        }
    }
    //
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

    println!("{:?}\n", test_model.raw_attrs());
    println!("{:?}", test_model.form_attrs());
}
