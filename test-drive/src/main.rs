use mongodb::Client;

mod mango_models;

async fn migration() {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    // Register models
    mango_models::User::migrat(&client).await;
    mango_models::Category::migrat(&client).await;
}

#[tokio::main]
async fn main() {
    // Run migration
    migration().await;
}
