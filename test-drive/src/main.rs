use mango_orm::widgets::IntegerWidget;

fn main() {
    let field = IntegerWidget {
        label: "Planets".to_string(),
        required: true,
        ..Default::default()
    };

    println!("{:#?}", field);
}
