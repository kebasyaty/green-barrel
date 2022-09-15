mod models;
mod settings;

use green_barrel::*;
//use mongodb::bson::doc;

// Migration Service `Mango`.
fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
    // Caching MongoDB clients.
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
    let monitor = Monitor {
        project_name: settings::PROJECT_NAME,
        unique_project_key: settings::UNIQUE_PROJECT_KEY,
        // Register models.
        // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        models: vec![models::User::meta()?],
    };
    monitor.migrat()?;
    //
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run migration.
    mango_migration()?;

    // Convert Model
    // *********************************************************************************************
    // println!("Convert Model:\n");
    //println!("{}", models::User::json()?);
    //println!("{}", models::User::model_json_for_admin()?);
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
    user.username.set("user_3");
    user.email.set("user_3_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.set("12345678");
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check Model.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCheck Modell:\n");
    let output_data = user.check(None)?;
    if output_data.is_valid() {
        println!("Hash: {}", output_data.hash());

        println!("Created at: {:?}", user.created_at.value.clone());
        println!("Updated at: {:?}", user.updated_at.value.clone());
        println!("Created at: {:?}", output_data.created_at());
        println!("Updated at: {:?}", output_data.updated_at());

        if let Ok(obj_id) = output_data.obj_id() {
            println!("Object Id: {:?}", obj_id);
        }
    } else {
        // Printing errors to the console ( for development ).
        output_data.print_err();
    }

    //println!("Json:\n{}", output_data.json()?);
    //println!("Json for admin:\n{}", output_data.json_for_admin()?);

    // Create document in database.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nCreate document in database:\n");
    let output_data = user.save(None, None)?;
    if output_data.is_valid() {
        println!("Hash: {}", output_data.hash());

        println!("Created at: {}", user.created_at.value.clone().unwrap());
        println!("Updated at: {}", user.updated_at.value.clone().unwrap());
        println!("Created at: {}", output_data.created_at().unwrap());
        println!("Updated at: {}", output_data.updated_at().unwrap());

        if let Ok(obj_id) = output_data.obj_id() {
            println!("Object Id: {:?}", obj_id);
        }

        // If there are AutoSlug fields, do an update.
        user = output_data.update()?;
        println!("Slug: {}", user.slug.value.clone().unwrap())

        //println!("Json:\n{}", output_data.json()?);
        //println!("Json for admin:\n{}", output_data.json_for_admin()?);
    } else {
        output_data.print_err();
    }

    // Update document in database.
    // ---------------------------------------------------------------------------------------------
    println!("\n\nUpdate document in database:\n");
    if output_data.is_valid() {
        user.username.set("user_3_update");
        let output_data = user.save(None, None)?;
        println!("Hash: {}", output_data.hash());

        println!("Created at: {}", user.created_at.value.clone().unwrap());
        println!("Updated at: {}", user.updated_at.value.clone().unwrap());
        println!("Created at: {}", output_data.created_at().unwrap());
        println!("Updated at: {}", output_data.updated_at().unwrap());

        if let Ok(obj_id) = output_data.obj_id() {
            println!("Object Id: {:?}", obj_id);
        }

        // If there are AutoSlug fields, do an update.
        user = output_data.update()?;
        println!("Slug: {}", user.slug.value.clone().unwrap());

        //println!("Json:\n{}", output_data.json()?);
        //println!("Json for admin:\n{}", output_data.json_for_admin()?);
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
