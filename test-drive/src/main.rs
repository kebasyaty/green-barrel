use mango_orm::fields::IntegerField;

fn main() {
    let field = IntegerField {
        label: "Planets".to_string(),
        required: true,
        ..Default::default()
    };

    println!("{:#?}", field);
}
