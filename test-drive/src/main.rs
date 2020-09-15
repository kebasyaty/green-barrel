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
    // Register models
    mango_models::User::migrat(&client).await;
    mango_models::Category::migrat(&client).await;
}

#[tokio::main]
async fn main() {
    // Run migration
    migration().await;
}
