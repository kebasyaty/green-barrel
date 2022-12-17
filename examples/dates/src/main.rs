mod migration;
mod models;
mod settings;

use chrono::Local;
use green_barrel::*;
use mongodb::{bson::doc, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // Specify the time zone (optional).
    // ( For convert to Utc )
    let tz = Some(Local::now().format("%z").to_string()); // or None

    // Create User
    // *********************************************************************************************
    let mut user = models::User::new().await?;
    user.username.set("user_1");
    user.date.set("1970-02-28");
    user.datetime.set("1970-02-28T00:00");

    // Save User
    // *********************************************************************************************
    let output_data = user.save(&client, &tz, None, None).await?;
    //user = output_data.update()?;

    if output_data.is_valid() {
        let filter = doc! {"username": "user_1"};
        if let Some(user) = models::User::find_one_to_doc(&client, filter, None).await? {
            println!("{:#?}", user);
        } else {
            panic!("Document is missing!");
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    Ok(())
}
