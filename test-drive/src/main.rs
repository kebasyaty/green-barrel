use mango_orm::forms::Form;
use mango_orm::models::Model;
use mango_orm::widgets::{DataType, Widget, WidgetFieldType};

pub struct TestModel {
    //
}

impl Form for TestModel {
    //
}

impl Model for TestModel {
    //
}

fn main() {
    let field_name = Widget {
        label: "Choose a car:",
        field_type: WidgetFieldType::Text,
        value: DataType::Text("mercedes"),
        required: true,
        select: vec![
            ("Volvo", DataType::Text("volvo")),
            ("Saab", DataType::Text("saab")),
            ("Mercedes", DataType::Text("mercedes")),
            ("Audi", DataType::Text("audi")),
        ],
        ..Default::default()
    };

    println!("{:#?}", field_name);
    println!("{:#?}", field_name.get_attrs("field_name"));
}
