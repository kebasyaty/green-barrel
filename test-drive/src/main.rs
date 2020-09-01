use mango_orm::widgets::{DataType, Widget};

fn main() {
    let field = Widget {
        label: "Planets",
        value: DataType::F64(10.3),
        required: true,
        ..Default::default()
    };

    println!("{:#?}", field);
}
