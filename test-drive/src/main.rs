use mango_orm::fields::IntegerField;

fn main() {
    let field = IntegerField {
        label: "Planets".to_string(),
        required: true,
        choices: vec![
            ("Mercury".to_string(), 1),
            ("Venus".to_string(), 2),
            ("Earth".to_string(), 3),
            ("Mars".to_string(), 4),
        ],
        ..Default::default()
    };

    println!("{:#?}", field);
}
