use mango_orm::models::Model;
use mongodb::{
    options::{ClientOptions, StreamAddress},
    Client,
};

mod mango_models;

async fn migration() {
    let client_options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: "localhost".into(),
            port: Some(27017),
        }])
        .build();
    let client = Client::with_options(client_options).unwrap();
    // Models
    mango_models::User::migrat(&client).await;
}

#[tokio::main]
async fn main() {
    // Run migrations
    migration().await;

    let user = mango_models::User {
        username: "Some text".to_string(),
        email: "some@some.net".to_string(),
    };

    println!("\n\n{:?}\n", user);
    println!("{:?}\n", mango_models::User::meta());
    println!("{:?}\n", mango_models::User::form_attrs());
    println!("{:?}", mango_models::User::form_attrs());
}
