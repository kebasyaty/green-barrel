use mango_orm::forms::Form;
use mango_orm::models::{field_types, Model};
use mango_orm::widgets::{DataType, FieldType, Widget};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TestModel {
    pub username: field_types::TextLine,
}

impl Form for TestModel {
    fn form(&self) -> HashMap<&'static str, Widget> {
        // Map of matching fields and widgets.
        let mut map = HashMap::new();
        map.insert(
            "username",
            Widget {
                id: "username",
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
    println!("{:?}", test_model.form());
}
