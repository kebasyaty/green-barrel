use mango_orm::forms::Form;
use mango_orm::models::Model;
use mango_orm::widgets::{DataType, Transport, Widget, WidgetFields};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
}

impl Model for User {
    //
}

impl Form for User {
    // Get raw attributes for further processing
    fn raw_attrs(&self) -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your name".to_string(),
                field_type: WidgetFields::InputText,
                value: DataType::Text("Rust"),
                maxlength: 30,
                required: true,
                hint: "Please enter your real name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Your name"),

                ..Default::default()
            },
        );
        raw_attrs
    }
    // Get pure attributes for a page templating engine
    fn form_attrs(&self) -> HashMap<String, Transport> {
        let raw_attrs = self.raw_attrs();
        let mut clean_attrs = HashMap::new();
        for (field, widget) in &raw_attrs {
            clean_attrs.insert(field.to_string(), widget.get_clean_attrs(field));
        }
        clean_attrs
    }
}

fn main() {
    let test_model = User {
        username: "Some text".to_string(),
    };

    println!("{:?}", test_model);
    println!("{}", test_model.username);
    println!("{:?}", test_model.form_attrs());
}
