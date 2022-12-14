mod app_state;
mod migration;
mod models;
mod settings;

use chrono::Local;
use green_barrel::*;
use mongodb::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // THIS IS REQUIRED FOR ALL PROJECTS
    // #############################################################################################
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    // Hint: Use to add to streams.
    let client = Client::with_uri_str(uri).await?;
    let _app_state = app_state::get_app_state()?;
    //
    migration::run_migration(&client).await?;

    // YOUR CODE ...
    // #############################################################################################
    //
    // Specify the time zone (optional).
    // By default Utc = +0000
    let tz = Some(Local::now().format("%z").to_string());

    // Create model instance.
    // *********************************************************************************************
    let mut user = models::User::new().await?;
    user.username.set("user_1");
    user.email.set("user_1_@noreply.net");

    // Check Model.
    // *********************************************************************************************
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(&client, &tz, None).await?;
    user = output_data.update()?;
    //
    if output_data.is_valid() {
        println!("Hash: {:?}", user.hash.get());
        println!("Hash: {}", output_data.hash());

        println!("Created at: {:?}", user.created_at.get());
        println!("Updated at: {:?}", user.updated_at.get());
        println!("Created at: {:?}", output_data.created_at());
        println!("Updated at: {:?}", output_data.updated_at());

        println!("Object Id: {:?}", user.hash.obj_id()?);
        println!("Object Id: {:?}", output_data.obj_id()?);

        //println!("Json:\n{}", output_data.json()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Create document in database.
    // *********************************************************************************************
    println!("\n\nCreate document in database:\n");
    let output_data = user.save(&client, &tz, None, None).await?;
    user = output_data.update()?;

    if output_data.is_valid() {
        println!("Hash: {}", user.hash.get().unwrap());
        println!("Hash: {}", output_data.hash());

        println!("Created at: {}", user.created_at.get().unwrap());
        println!("Updated at: {}", user.updated_at.get().unwrap());
        println!("Created at: {}", output_data.created_at().unwrap());
        println!("Updated at: {}", output_data.updated_at().unwrap());

        println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
        println!("Object Id: {:?}", output_data.obj_id()?.unwrap());

        println!("Slug: {}", user.slug.get().unwrap());

        //println!("Json:\n{}", output_data.json()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Update document in database.
    // *********************************************************************************************
    println!("\n\nUpdate document in database:\n");
    if output_data.is_valid() {
        user.username.set("new_user_1");

        let output_data = user.save(&client, &tz, None, None).await?;
        user = output_data.update()?;

        if output_data.is_valid() {
            println!("Hash: {}", user.hash.get().unwrap());
            println!("Hash: {}", output_data.hash());

            println!("Created at: {}", user.created_at.get().unwrap());
            println!("Updated at: {}", user.updated_at.get().unwrap());
            println!("Created at: {}", output_data.created_at().unwrap());
            println!("Updated at: {}", output_data.updated_at().unwrap());

            println!("Object Id: {:?}", user.hash.obj_id()?.unwrap());
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

    // Delete document in database.
    // *********************************************************************************************
    println!("\n\nDelete document in database:\n");
    let output_data = user.delete(&client, None).await?;
    if !output_data.is_valid() {
        output_data.print_err();
        // or
        println!("ERROR: {}", output_data.err_msg());
    }

    Ok(())
}
