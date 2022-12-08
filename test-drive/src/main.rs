mod app_state;
mod models;
mod settings;

use async_lock::RwLock;
use green_barrel::*;
use mongodb::Client;
use std::sync::Arc;
use std::{collections::HashMap, error::Error};

// Migration
async fn run_migration(
    meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
    client: &Client,
    media_dir: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    // Caching metadata.
    models::User::caching(meta_store, client).await?;
    models::City::caching(meta_store, client).await?;

    // Monitor initialization.
    let monitor = Monitor {
        project_name: settings::PROJECT_NAME,
        unique_project_key: settings::UNIQUE_PROJECT_KEY,
        // For register models.
        model_key_list: vec![models::User::key()?, models::City::key()?],
    };
    monitor.migrat(meta_store, client).await?;

    // Run fixtures
    models::City::run_fixture("cities", meta_store, client, media_dir).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // THIS IS REQUIRED FOR ALL PROJECTS
    // Hint: This is done to be able to add data to streams.
    // #############################################################################################
    let _app_state = app_state::get_app_state()?;
    let media_dir = app_state::get_media_dir()?;
    let meta_store = Arc::new(get_meta_store());
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    run_migration(&meta_store, &client, &media_dir).await?;

    // YOUR CODE ...
    // #############################################################################################

    // Convert Model
    // *********************************************************************************************
    // println!("Convert Model:\n");
    //println!("{}", models::User::json()?);
    //
    /*
    println!(
        "Model instance:\n{:?}",
        models::User::find_one_to_instance(
            mongodb::bson::doc! {"username": "user_1"},
            &meta_store,
            &client,
            None
        )?
    );
    */

    // Create model instance.
    // *********************************************************************************************
    let mut user = models::User::new(&meta_store).await?;
    user.username.set("user_1");
    user.email.set("user_1_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.value = Some("12345678".to_string()); // Example without the set() method
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check Model.
    // *********************************************************************************************
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(&meta_store, &client, &media_dir, None).await?;
    user = output_data.update()?;

    if output_data.is_valid() {
        println!("Hash: {:?}", user.hash.get());
        println!("Hash: {}", output_data.hash());

        println!("Created at: {:?}", user.created_at.get());
        println!("Updated at: {:?}", user.updated_at.get());
        println!("Created at: {:?}", output_data.created_at());
        println!("Updated at: {:?}", output_data.updated_at());

        println!("Object Id: {:?}", user.hash.obj_id()?);
        println!("Object Id: {:?}", output_data.obj_id()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    //println!("Json:\n{}", output_data.json()?);
    //println!("Json for admin:\n{:?}", output_data.json_for_admin()?);

    // Create document in database.
    // *********************************************************************************************
    println!("\n\nCreate document in database:\n");
    let output_data = user
        .save(&meta_store, &client, &media_dir, None, None)
        .await?;
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
        //println!("Json for admin:\n{:?}", output_data.json_for_admin()?);
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    // Update document in database.
    // *********************************************************************************************
    println!("\n\nUpdate document in database:\n");
    if output_data.is_valid() {
        user.username.set("new_user_1");

        let output_data = user
            .save(&meta_store, &client, &media_dir, None, None)
            .await?;
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
            //println!("Json for admin:\n{:?}", output_data.json_for_admin()?);
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
    let output_data = user.delete(&meta_store, &client, None).await?;
    if !output_data.is_valid() {
        output_data.print_err();
        // or
        println!("ERROR: {}", output_data.err_msg());
    }

    Ok(())
}
