use mango_orm::models::Monitor;
use mongodb::Client;

mod mango_models;

async fn mango_migration() {
    static KEYWORD: &'static str = "7rzg_cfqQB3B7q7T";
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let monitor = Monitor {
        keyword: KEYWORD,
        client: &client,
    };
    // Refresh models state
    monitor.refresh().await;
    // Register models
    mango_models::User::migrat(KEYWORD, &client).await;
    mango_models::Category::migrat(KEYWORD, &client).await;
    // Reorganize databases state
    // (full removal of irrelevant databases and collections)
    monitor.napalm().await;
}

#[tokio::main]
async fn main() {
    // Run migration
    mango_migration().await;
}
