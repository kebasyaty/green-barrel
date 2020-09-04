use mango_orm::forms::Form;
use mango_orm::models::Model;
use mango_orm::widgets::{DataType, FieldType, Widget};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct User {
    pub username: String,
    pub email: String,
    pub all_attrs: f64,
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
                required: true,
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
                required: true,
                hint: "Enter your work email.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                ..Default::default()
            },
        );
        raw_attrs.insert(
            "all_attrs",
            Widget {
                label: "Your Email".to_string(),
                field_type: FieldType::Select,
                value: DataType::F64(0.0),
                maxlength: 20,
                required: true,
                readonly: true,
                disabled: true,
                checked: true,
                hidden: true,
                hint: "Test all attrs.".to_string(),
                unique: true,
                other_attrs: format!(
                    "multiple placeholder=\"{}\" step=\"{}\"",
                    "Test all attrs", 0.01
                ),
                some_classes: "class-name class-name-2".to_string(),
                select: vec![
                    ("Mercury".to_string(), DataType::F64(3.302)),
                    ("Venus".to_string(), DataType::F64(4.869)),
                    ("Earth".to_string(), DataType::F64(5.974)),
                    ("Mars".to_string(), DataType::F64(6.419)),
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
        all_attrs: 10.02,
    };

    println!("{:?}", test_model);
    println!("{}", test_model.username);
    println!("{:?}", test_model.form_attrs());
}
