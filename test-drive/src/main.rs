use mango_orm::models::Model;

pub mod mango_models;

fn main() {
    let test_model = mango_models::User {
        username: "Some text".to_string(),
        email: "some@some.net".to_string(),
        categories: vec![
            "id-1".to_string(),
            "id-2".to_string(),
            "id-3".to_string(),
            "id-4".to_string(),
        ],
    };

    println!("{:?}\n", test_model.raw_attrs());
    println!("{:?}", test_model.form_attrs());
}
