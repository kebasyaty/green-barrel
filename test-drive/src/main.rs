use mango_orm::widgets::StandardWidget;

fn main() {
    let field = StandardWidget {
        label: "Planets".to_string(),
        required: true,
        ..Default::default()
    };

    println!("{:#?}", field);
}
