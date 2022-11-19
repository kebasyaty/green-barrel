mod models;
mod settings;

use green_barrel::*;
//use mongodb::bson::doc;
use std::error::Error;

// Migration
fn run_migration() -> Result<(), Box<dyn Error>> {
    // Caching MongoDB clients.
    // ---------------------------------------------------------------------------------------------
    {
        let mut client_store = MONGODB_CLIENT_STORE.write()?;
        client_store.insert(
            "default".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        client_store.insert(
            "default_2".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
    }

    // Monitor initialization.
    // ---------------------------------------------------------------------------------------------
    let monitor = Monitor {
        project_name: settings::PROJECT_NAME,
        unique_project_key: settings::UNIQUE_PROJECT_KEY,
        // Register models.
        metadata_list: vec![models::User::meta()?, models::City::meta()?],
    };
    monitor.migrat()?;

    // Run fixtures
    // ---------------------------------------------------------------------------------------------
    models::City::run_fixture("cities")?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Run migration.
    run_migration()?;

    // Convert Model
    // *********************************************************************************************
    // println!("Convert Model:\n");
    //println!("{}", models::User::json()?);
    //
    /*
        println!(
            "Model instance:\n{:?}",
            models::User::find_one_to_instance(doc! {"username": "user_1"}, None)?
        );
    */

    // Create model instance.
    // *********************************************************************************************
    let mut user = models::User::new()?;
    user.username.set("user_12");
    user.email.set("user_12_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.value = Some("12345678".to_string()); // Example without the set() method
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check Model.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(None)?;
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
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCreate document in database:\n");
    let output_data = user.save(None, None)?;
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
    // ---------------------------------------------------------------------------------------------
    println!("\n\nUpdate document in database:\n");
    if output_data.is_valid() {
        user.username.set("new_user_12");

        let output_data = user.save(None, None)?;
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
    println!("\n\nDelete document in database:\n");
    let output_data = user.delete(None)?;
    if !output_data.is_valid() {
        println!("{}", output_data.err_msg());
    }

    Ok(())
}
