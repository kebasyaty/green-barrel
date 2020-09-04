use mango_orm::forms::Form;
use mango_orm::models::Model;
use mango_orm::widgets::{DataType, FieldType, Widget};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub email: String,
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
                value: DataType::Text("Rust"),
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
        raw_attrs
    }
}

fn main() {
    let test_model = User {
        username: "Some text".to_string(),
        email: "some@some.net".to_string(),
    };

    println!("{:?}", test_model);
    println!("{}", test_model.username);
    println!("{:?}", test_model.form_attrs());
}
