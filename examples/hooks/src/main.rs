mod migration;
mod models;
mod settings;

use chrono::Local;
use green_barrel::*;
use mongodb::Client;
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
    user.email.set("user_1_@noreply.net");

    // Check User
    // *********************************************************************************************
    println!("\n\nCheck User:\n");
    let output_data = user.check(&client, &tz, None).await?;
    user = output_data.update()?;
    //
    if output_data.is_valid() {
        println!("Hash: {:?}", user.hash.get());
        // or
        println!("Hash: {}", output_data.hash());

        println!("Created at: {:?}", user.created_at.get());
        println!("Updated at: {:?}", user.updated_at.get());
        // or
        println!("Created at: {:?}", output_data.created_at());
        println!("Updated at: {:?}", output_data.updated_at());

        println!("Object Id: {:?}", user.hash.obj_id()?);
        // or
        println!("Object Id: {:?}", output_data.obj_id()?);

        //println!("Json:\n{}", output_data.json()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Save User
    // *********************************************************************************************
    println!("\n\nSave User:\n");
    let output_data = user.save(&client, &tz, None, None).await?;
    user = output_data.update()?;

    if output_data.is_valid() {
        println!("Hash: {}", user.hash.get().unwrap());
        // or
        println!("Hash: {}", output_data.hash());

        println!("Created at: {}", user.created_at.get().unwrap());
        println!("Updated at: {}", user.updated_at.get().unwrap());
        // or
        println!("Created at: {}", output_data.created_at().unwrap());
        println!("Updated at: {}", output_data.updated_at().unwrap());

        println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
        // or
        println!("Object Id: {:?}", output_data.obj_id()?.unwrap());

        println!("Slug: {}", user.slug.get().unwrap());

        //println!("Json:\n{}", output_data.json()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Update User
    // *********************************************************************************************
    println!("\n\nUpdate User:\n");
    if output_data.is_valid() {
        user.username.set("new_user_1");

        let output_data = user.save(&client, &tz, None, None).await?;
        user = output_data.update()?;

        if output_data.is_valid() {
            println!("Hash: {}", user.hash.get().unwrap());
            // or
            println!("Hash: {}", output_data.hash());

            println!("Created at: {}", user.created_at.get().unwrap());
            println!("Updated at: {}", user.updated_at.get().unwrap());
            // or
            println!("Created at: {}", output_data.created_at().unwrap());
            println!("Updated at: {}", output_data.updated_at().unwrap());

            println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
            // or
            println!("Object Id: {:?}", output_data.obj_id()?.unwrap());

            println!("Slug: {}", user.slug.get().unwrap())

            //println!("Json:\n{}", output_data.json()?);
        } else {
            // Printing errors to the console ( for development ).
            output_data.print_err();
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Delete User
    // *********************************************************************************************
    println!("\n\nDelete User:\n");
    let output_data = user.delete(&client, None).await?;
    if !output_data.is_valid() {
        output_data.print_err();
        // or
        //println!("ERROR: {}", output_data.err_msg());
    }

    Ok(())
}
