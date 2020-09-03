use mango_orm::forms::Form;
use mango_orm::models::{field_types as model, Model};
use mango_orm::widgets::{DataType, FieldType as widget, Transport, Widget};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TestModel {
    pub username: model::InputText,
}

impl Model for TestModel {
    //
}

impl Form for TestModel {
    // Get raw attributes for further processing
    fn raw_attrs(&self) -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your name".to_string(),
                field_type: widget::InputText,
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
    let test_model = TestModel {
        username: model::InputText::Data(Some("Some text")),
    };

    println!("{:?}", test_model);
    println!("{}", test_model.username.get_data().unwrap_or(""));
    println!("{:?}", test_model.raw_attrs());
    println!("{:?}", test_model.form_attrs());
}
