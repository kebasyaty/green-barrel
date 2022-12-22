mod migration;
mod models;
mod settings;

use green_barrel::*;
use mongodb::{bson::doc, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // Add file and image
    // *********************************************************************************************
    let mut user = models::User::new().await?;
    user.username.set("user_1");
    user.avatar.set("./some_files/avatar.png", false, None);
    user.resume.set("./some_files/resume.pdf", false, None);

    // Save file and image
    // *********************************************************************************************
    println!("\n\nSave file and image:\n");
    let output_data = user.save(&client, None, None).await?;
    //user = output_data.update()?;
    if output_data.is_valid() {
        // Get doc
        let filter = doc! {"username": "user_1"};
        if let Some(user_doc) = models::User::find_one_to_doc(&client, filter, None).await? {
            println!("{:#?}", user_doc);
        } else {
            panic!("The document is missing!");
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Update file and image
    // *********************************************************************************************
    println!("\n\nUpdate file and image:\n");
    user.avatar.set("./some_files/avatar_2.jpg", false, None);
    user.resume.set("./some_files/resume_2.odt", false, None);
    let output_data = user.save(&client, None, None).await?;
    //user = output_data.update()?;
    if output_data.is_valid() {
        // Get doc
        let filter = doc! {"username": "user_1"};
        if let Some(user_doc) = models::User::find_one_to_doc(&client, filter, None).await? {
            println!("{:#?}", user_doc);
        } else {
            panic!("The document is missing!");
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Delete file and image
    // *********************************************************************************************
    println!("\n\nDelete file and image:\n");
    user.avatar.set("", true, None);
    user.resume.set("", true, None);
    let output_data = user.save(&client, None, None).await?;
    //user = output_data.update()?;
    if output_data.is_valid() {
        // Get doc
        let filter = doc! {"username": "user_1"};
        if let Some(user_doc) = models::User::find_one_to_doc(&client, filter, None).await? {
            println!("{:#?}", user_doc);
        } else {
            panic!("The document is missing!");
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    Ok(())
}
