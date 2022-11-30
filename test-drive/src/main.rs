mod app_state;
mod models;
mod settings;

use green_barrel::*;
//use mongodb::bson::doc;
use mongodb::sync::Client;
use regex::Regex;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, error::Error};

// Migration
fn run_migration(
    meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
    client: &Client,
    validators: &HashMap<String, Regex>,
    media_dir: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    // Caching metadata.
    models::User::caching(meta_store, client)?;
    models::City::caching(meta_store, client)?;

    // Monitor initialization.
    let monitor = Monitor {
        project_name: settings::PROJECT_NAME,
        unique_project_key: settings::UNIQUE_PROJECT_KEY,
        // For register models.
        model_key_list: vec![models::User::key()?, models::City::key()?],
    };
    monitor.migrat(meta_store, client)?;

    // Run fixtures
    models::City::run_fixture("cities", meta_store, client, validators, media_dir)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // This is required for all projects.
    // #############################################################################################
    let app_state = app_state::get_app_state()?;
    let media_dir = app_state::get_media_dir(app_state);
    let meta_store = get_meta_store();
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let validators = get_validators()?;
    run_migration(&meta_store, &client, &validators, &media_dir)?;

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
            models::User::find_one_to_instance(doc! {"username": "user_1"}, &meta_store, &client, None)?
        );
    */

    // Create model instance.
    // *********************************************************************************************
    let mut user = models::User::new(&meta_store)?;
    user.username.set("user_1");
    user.email.set("user_1_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.value = Some("12345678".to_string()); // Example without the set() method
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check Model.
    // *********************************************************************************************
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(&meta_store, &client, &validators, &media_dir, None)?;
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
    let output_data = user.save(&meta_store, &client, &validators, &media_dir, None, None)?;
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

        let output_data = user.save(&meta_store, &client, &validators, &media_dir, None, None)?;
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
    let output_data = user.delete(&meta_store, &client, None)?;
    if !output_data.is_valid() {
        output_data.print_err();
        // or
        println!("ERROR: {}", output_data.err_msg());
    }

    Ok(())
}
