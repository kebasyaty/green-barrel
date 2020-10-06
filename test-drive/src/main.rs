use mango_orm::migration::Monitor;
use mongodb::Client;

mod mango_models;

async fn mango_migration() {
    // KEYWORD - It is recommended not to change within the boundaries of one project
    // (Valid characters: _|a-z|A-Z|0-9 ; Size: 8-16.)
    static KEYWORD: &'static str = "7rzg_cfqQB3B7q7T";
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
    mango_models::User::migrat(KEYWORD, &client).await;
    mango_models::Category::migrat(KEYWORD, &client).await;
    // Reorganize databases state
    // (full delete of orphaned collections and databases)
    monitor.napalm().await;
}

#[tokio::main]
async fn main() {
    // Run migration
    mango_migration().await;

    // println!("{:?}", mango_models::User::form_map_attrs().unwrap());
    // println!("{}", mango_models::User::form_json_attrs().unwrap());
    // println!("{}", mango_models::User::form_html("/", None, None).unwrap());
    // println!("{}", mango_models::User::form_html("/", Some(Method::Post), Some(Enctype::Multipart)).unwrap());

    let mut user = mango_models::User {
        username: "Rust".to_string(),
        email: "x15@x.xx".to_string(),
        ..Default::default()
    };
    let client: Client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    println!("{}", user.save(&client).await.unwrap().to_hash().unwrap());
    println!("{}", user.save(&client).await.unwrap().to_hash().unwrap());
}
