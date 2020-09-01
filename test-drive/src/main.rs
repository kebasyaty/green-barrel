use mango_orm::forms::Form;
use mango_orm::widgets::{DataType, FieldType, Widget};

pub struct TestModel {
    //
}

fn main() {
    let field_name = Widget {
        label: "Choose a car:",
        field_type: FieldType::Text,
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
