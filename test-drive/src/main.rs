use mango_orm::widgets::Widget;

fn main() {
    let field = Widget {
        label: "Planets".to_string(),
        required: true,
        ..Default::default()
    };

    println!("{:#?}", field);
}
