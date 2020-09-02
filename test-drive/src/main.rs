use mango_orm::forms::Form;
use mango_orm::models::{field_types, Model};
use mango_orm::widgets::{DataType, FieldType, Transport, Widget};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TestModel {
    pub username: field_types::TextLine,
}

impl Form for TestModel {
    // Get raw attributes for further processing
    fn raw_attrs(&self) -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut map = HashMap::new();
        map.insert(
            "username",
            Widget {
                label: "Your name",
                field_type: FieldType::TextLine,
                value: DataType::Text("Rust"),
                maxlength: 30,
                required: true,
                hint: "Please enter your real name.",
                ..Default::default()
            },
        );
        map
    }
    // Get pure attributes for a page templating engine
    fn form_attrs(&self) -> HashMap<String, Transport> {
        let raw_attrs = self.raw_attrs();
        let mut attrs = HashMap::new();
        for (field, widget) in &raw_attrs {
            attrs.insert(field.to_string(), widget.get_attrs(field));
        }
        attrs
    }
}

impl Model for TestModel {
    //
}

fn main() {
    let test_model = TestModel {
        username: field_types::TextLine::Data(Some("Some text")),
    };

    println!("{:?}", test_model);
    println!("{}", test_model.username.get_data().unwrap_or(""));
    println!("{:?}", test_model.raw_attrs());
    println!("{:?}", test_model.form_attrs());
}
