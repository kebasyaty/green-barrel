use mango_orm::models::Monitor;
use mongodb::Client;

mod mango_models;

async fn migration() {
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let monitor = Monitor {
        password: "7rzg_cfqQB3B7q7T",
        _client: &client,
    };
    // Refresh models state
    monitor.refresh().await;
    // Register models
    mango_models::User::migrat(&client).await;
    mango_models::Category::migrat(&client).await;
    // Reorganize databases state
    monitor.run().await;
}

#[tokio::main]
async fn main() {
    // Run migration
    migration().await;
}
