mod migration;
mod models;
mod settings;

use green_barrel::*;
use mongodb::{bson::doc, Client};
use std::error::Error;

#[macro_use]
extern crate rust_i18n;

i18n!("locales");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Change globally current locale
    // defaule = en-us
    rust_i18n::set_locale("ru");
    // Init Client
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // Create User
    // *********************************************************************************************
    let mut user = models::User::new().await?;
    user.username.set("user_1");
    user.date.set("1970-02-28");
    user.datetime.set("1970-02-28T00:00");

    // Save User
    // *********************************************************************************************
    let output_data = user.save(&client, None, None).await?;
    //user = output_data.update()?;

    if output_data.is_valid() {
        // Get doc
        let filter = doc! {"username": "user_1"};
        if let Some(user) = models::User::find_one_to_instance(&client, filter, None).await? {
            println!("Date: {}", user.date.get().unwrap());
            println!("Date and Time: {}", user.datetime.get().unwrap());
            println!("Created at: {}", user.created_at.get().unwrap());
            println!("Updated at: {}", user.updated_at.get().unwrap());
        } else {
            panic!("Document is missing!");
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    Ok(())
}
