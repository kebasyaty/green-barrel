use mango_orm::forms::Form;
use mango_orm::models::{Model, ModelsFieldType};
use mango_orm::widgets::Widget;
use std::collections::HashMap;

pub struct TestModel {
    pub title: ModelsFieldType,
}

impl Form for TestModel {
    fn form() -> HashMap<&'static str, Widget> {
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
    //
}
