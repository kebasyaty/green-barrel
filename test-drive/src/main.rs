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
        models: vec![models::Dynamic::meta()?, models::UserProfile::meta()?],
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
    //println!("{:?}", models::UserProfile::to_wig()?);
    //println!("{}", models::UserProfile::to_json()?);
    //println!("{}", models::UserProfile::model_to_json_for_admin()?);
    /*
    println!(
        "Html code:\n{}",
        models::UserProfile::to_html(
            Some("/login"),
            Some(HttpMethod::POST),
            Some(Enctype::Multipart)
        )?
    );
    */
    //
    /*
       println!(
           "Widget map:\n{:?}",
           models::UserProfile::find_one_to_wig(doc! {"username": "user_34"}, None)?
       );
    */
    /*
        println!(
            "Model instance:\n{:?}",
            models::UserProfile::find_one_to_model_instance(doc! {"username": "user_38"}, None)?
        );
    */

    // Test model instance.
    // *********************************************************************************************
    let mut user = models::UserProfile::new()?;
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
    println!("Hash: {}", output_data.hash());
    println!("Object Id: {:?}", output_data.object_id());
    println!("Created at: {}", user.get_created_at());
    println!("Updated at: {}", user.get_updated_at());
    // Printing errors to the console ( for development ).
    if !output_data.is_valid() {
        output_data.print_err();
    }
    //
    //println!("Slug: {}\n\n", output_data.to_wig().get("slug").unwrap().value);
    //
    //println!("bson::Document:\n{:?}\n\n", output_data.get_doc());
    //println!("Widget map:\n{:?}\n\n", output_data.to_wig());
    //println!("Json:\n{}\n\n", output_data.to_json()?);
    //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);
    /*
    println!(
        "Html:\n{}\n\n",
        output_data.to_html(
            Some("/login"),
            Some(HttpMethod::POST),
            Some(Enctype::Multipart)
        )?
    );
    */
    */

    // Create document in database.
    // ---------------------------------------------------------------------------------------------
    let output_data = user.save(None, None)?;
    println!("Boolean: {}", output_data.is_valid());
    println!("Hash: {}", output_data.hash());
    println!("Object Id: {:?}", output_data.object_id());
    println!("Created at: {}", user.get_created_at());
    println!("Updated at: {}", user.get_updated_at());
    // Printing errors to the console ( for development ).
    if !output_data.is_valid() {
        output_data.print_err();
    }
    //
    println!(
        "Slug: {}\n\n",
        output_data.to_wig().get("slug").unwrap().value
    );
    //
    //println!("bson::Document:\n{:?}\n\n", output_data.get_doc());
    //println!("Widget map:\n{:?}\n\n", output_data.to_wig());
    //println!("Json:\n{}\n\n", output_data.to_json()?);
    //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);
    /*
    println!(
        "Html:\n{}\n\n",
        output_data.to_html(
            Some("/login"),
            Some(HttpMethod::POST),
            Some(Enctype::Multipart)
        )?
    );
    */

    // Update document in database.
    // ---------------------------------------------------------------------------------------------
    if output_data.is_valid() {
        let output_data = user.save(None, None)?;
        println!("Boolean: {}", output_data.is_valid());
        println!("Hash: {}", output_data.hash());
        println!("Object Id: {:?}", output_data.object_id());
        println!("Created at: {}", user.get_created_at());
        println!("Updated at: {}", user.get_updated_at());
        // Printing errors to the console ( for development ).
        if !output_data.is_valid() {
            output_data.print_err();
        }
        //
        println!(
            "Slug: {}\n\n",
            output_data.to_wig().get("slug").unwrap().value
        );
        //
        //println!("bson::Document:\n{:?}\n\n", output_data.get_doc());
        //println!("Widget map:\n{:?}\n\n", output_data.to_wig());
        //println!("Json:\n{}\n\n", output_data.to_json()?);
        //println!("Json for admin:\n{}\n\n", output_data.to_json_for_admin()?);
        /*
        println!(
            "Html:\n{}\n\n",
            output_data.to_html(
                Some("/login"),
                Some(HttpMethod::POST),
                Some(Enctype::Multipart)
            )?
        );
        */
    }
    //
    Ok(())
}
