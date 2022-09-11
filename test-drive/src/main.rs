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
    //println!("{}", models::UserProfile::to_json()?);
    //println!("{}", models::UserProfile::model_to_json_for_admin()?);
    //
    /*
        println!(
            "Model instance:\n{:?}",
            models::UserProfile::find_one_to_model_instance(doc! {"username": "user_1"}, None)?
        );
    */

    // Create model instance.
    // *********************************************************************************************
    let mut user = models::User::new()?;
    user.username.set("user_1");
    user.email.set("user_1_@noreply.net");
    user.password.set("12345678");
    user.confirm_password.set("12345678");
    user.is_staff.set(true);
    user.is_active.set(true);

    // Check model.
    // ---------------------------------------------------------------------------------------------
    /*
    let output_data = user.check(None)?;
    println!("Boolean: {}", output_data.is_valid());
    println!("Hash: {}", output_data.get_hash());
    if let Ok(obj_id) = output_data.get_obj_id() {
        println!("Object Id: {:?}", obj_id);
    }
    //
    // Printing errors to the console ( for development ).
    if !output_data.is_valid() {
        output_data.print_err();
    }
    */
    //
    //println!("Json:\n{}\n\n", output_data.to_json()?);
    //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);

    // Create document in database.
    // ---------------------------------------------------------------------------------------------
    let output_data = user.save(None, None)?;
    println!("Boolean: {}", output_data.is_valid());
    println!("Hash: {}", output_data.get_hash());
    if let Ok(obj_id) = output_data.get_obj_id() {
        println!("Object Id: {:?}", obj_id);
    }
    println!("Created at: {}", user.get_created_at());
    println!("Updated at: {}", user.get_updated_at());
    //
    // Printing errors to the console ( for development ).
    if !output_data.is_valid() {
        output_data.print_err();
    }
    //
    // If there are AutoSlug fields, do an update.
    if output_data.is_valid() {
        user = output_data.update()?;
        println!("Slug: {}", user.slug.value.clone().unwrap_or_default())
    }
    //
    //println!("Json:\n{}\n\n", output_data.to_json()?);
    //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);

    // Update document in database.
    // ---------------------------------------------------------------------------------------------
    if output_data.is_valid() {
        user.username.set("user_1_update");
        let output_data = user.save(None, None)?;
        println!("Boolean: {}", output_data.is_valid());
        println!("Hash: {}", output_data.get_hash());
        if let Ok(obj_id) = output_data.get_obj_id() {
            println!("Object Id: {:?}", obj_id);
        }
        println!("Created at: {}", user.get_created_at());
        println!("Updated at: {}", user.get_updated_at());
        // Printing errors to the console ( for development ).
        if !output_data.is_valid() {
            output_data.print_err();
        }
        //
        // If there are AutoSlug fields, do an update.
        if output_data.is_valid() {
            user = output_data.update()?;
            println!("Slug: {}", user.slug.value.clone().unwrap_or_default())
        }
        //
        //println!("Json:\n{}\n\n", output_data.to_json()?);
        //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);
    }
    //
    Ok(())
}
