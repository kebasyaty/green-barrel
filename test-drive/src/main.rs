use mango_orm::models::Model;
use mongodb::{
    options::{ClientOptions, StreamAddress},
    Client,
};
// use tokio::runtime::Runtime;

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
    mango_models::Category::migrat(client.clone()).await;
    mango_models::User::migrat(client.clone()).await;
}

#[tokio::main]
async fn main() {
    // Run migrations
    migration().await;

    let user: mango_models::User = Default::default();

    println!("\n\n{:?}\n", user);
    println!("{:?}\n", mango_models::User::raw_attrs());
    println!("{:?}", mango_models::User::form_attrs());
}
