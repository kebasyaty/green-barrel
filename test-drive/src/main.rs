use mango_orm::forms::Form;
use mango_orm::models::{field_types, Model};
use mango_orm::widgets::Widget;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TestModel {
    pub title: field_types::TextLine,
}

impl Form for TestModel {
    fn form(&self) -> HashMap<&'static str, Widget> {
        let mut map = HashMap::new();
        map.insert(
            "title",
            Widget {
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
        title: field_types::TextLine::Data(Some("Some text")),
    };

    println!("{:?}", test_model);
    println!("{}", test_model.title.get_data().unwrap_or(""));
    println!("{:?}", test_model.form());
}
