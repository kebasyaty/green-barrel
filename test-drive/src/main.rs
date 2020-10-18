use mango_orm::{forms::OutputType, migration::Monitor};
use mongodb::Client;

mod mango_models;

// Migration Service `Mango`
async fn mango_migration() {
    // KEYWORD - It is recommended not to change within the boundaries of one project
    // (Valid characters: _|a-z|A-Z|0-9 ; Size: 8-16.)
    static KEYWORD: &str = "7rzg_cfqQB3B7q7T";
    let client: Client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let monitor = Monitor {
        keyword: KEYWORD,
        client: &client,
    };
    // Refresh models state
    monitor.refresh().await;
    // Register models
    mango_models::User::migrat(&client, KEYWORD).await;
    // Reorganize databases state
    // (full delete of orphaned collections and databases)
    monitor.napalm().await;
}

#[tokio::main]
async fn main() {
    // Run migration
    mango_migration().await;

    // println!("{:?}", mango_models::User::form_map().await.unwrap());
    // println!("\n{}", mango_models::User::form_json().await.unwrap());
    // println!("\n{}", mango_models::User::form_html().await.unwrap());

    let mut user = mango_models::User {
        username: "Rust".to_string(),
        email: "test_4_@test.test".to_string(),
        password: "12345678".to_string(),
        password_confirm: "12345678".to_string(),
        datetime: "2020-10-16T12:52".to_string(),
        ..Default::default()
    };
    let client: Client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    let result = user.save(&client, OutputType::Hash).await.unwrap();
    println!("\n{}", result.hash());
    println!("{}", result.bool());

    let result = user.save(&client, OutputType::Hash).await.unwrap();
    println!("\n{}", result.hash());
    println!("{}", result.bool());
}
