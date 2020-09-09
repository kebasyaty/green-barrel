use mango_orm::models::Model;

mod mango_models;

fn migration() {
    mango_models::Category::migrat();
    mango_models::User::migrat();
}

fn main() {
    // Run migrations
    migration();

    let user = mango_models::User {
        username: "Some text".to_string(),
        email: "some@some.net".to_string(),
        categories: vec![
            "id-1".to_string(),
            "id-2".to_string(),
            "id-3".to_string(),
            "id-4".to_string(),
        ],
    };

    println!("\n\n{:?}\n", user);
    println!("{:?}\n", mango_models::User::raw_attrs());
    println!("{:?}", mango_models::User::form_attrs());
}
