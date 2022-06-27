mod mango_models;
mod settings;

use mango_orm::*;
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
        models: vec![
            mango_models::Dynamic::meta()?,
            mango_models::UserProfile::meta()?,
        ],
    };
    monitor.migrat()?;

    // Add metadata and widgects map to cache.
    // Hint: Optional. It is required to add to work with the admin panel.
    // Admin panel: https://github.com/kebasyaty/mango-panel
    // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
    /*
    mango_models::Dynamic::to_cache()?;
    mango_models::User::to_cache()?;
    mango_models::UserProfile::to_cache()?;
    */
    //
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run migration.
    mango_migration()?;

    // Test model.
    // *********************************************************************************************
    // println!("{:?}\n\n", mango_models::UserProfile::to_wig()?);
    // println!("{}\n\n", mango_models::UserProfile::to_json()?);
    // println!("{}\n\n", mango_models::UserProfile::to_html()?);
    // println!("{}\n\n", mango_models::UserProfile::to_json_for_admin()?);
    //
    /*
    println!(
        "Result:\n{:?}\n\n",
        mango_models::UserProfile::find_one_to_wig(doc! {"username": "user_34"}, None)?
    );

    println!(
        "Result:\n{:?}\n\n",
        mango_models::UserProfile::find_one_to_model_instance::<mango_models::UserProfile>(
            doc! {"username": "user_34"},
            None
        )?
    );
    */

    // Test model instance.
    // *********************************************************************************************
    let mut user = mango_models::UserProfile {
        username: Some("user_38".to_string()),
        email: Some("user_38_@noreply.net".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
        is_staff: Some(false),
        ..Default::default() // or initialize the `hash` field - { hash: Some(String::new()) }
    };

    // Check.
    /*
    let result = user.check()?;
    println!("Boolean: {}", result.is_valid());
    println!("\n\nbson::Document:\n{:?}", result.to_doc());
    */
    //println!("Object Id: {:?}", result.object_id()?);
    //println!("\n\nWidget map:\n{:?}", result.to_wig());
    //println!("\n\nJson:\n{}", result.to_json()?);
    //println!("\n\nHtml:\n{}", result.to_html());

    // Create document in database.
    let result = user.save(None, None)?;
    println!("Boolean: {}", result.is_valid());
    println!("Hash: {}", result.hash()?);
    println!(
        "Created at: {}",
        user.created_at.clone().unwrap_or_default()
    );
    println!(
        "Updated at: {}",
        user.updated_at.clone().unwrap_or_default()
    );
    // Printing errors to the console ( for development ).
    if !result.is_valid() {
        result.print_err();
    }
    //
    //println!("\nObject Id:\n{:?}\n", result.object_id()?);
    // println!("\n\nWidget map:\n{:?}", result.to_wig());
    //println!("\n\nSlug:\n{}", result.to_wig().get("slug").unwrap().value);
    //println!("\n\nJson:\n{}", result.to_json()?);
    //println!("\n\nHtml:\n{}\n", result.to_html());
    //println!("\nJson for admin:\n{}\n", result.to_json_for_admin()?);
    /*
    println!(
        "Verify password (false): {}",
        user.verify_password("123456789", None)?
    );
    println!(
        "Verify password (true): {}",
        user.verify_password("12345678", None)?
    );

    // Get Model instance.
    println!(
        "Model instance:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?
            .model::<mango_models::UserProfile>()?
    );
    */

    // Update document in database.
    if result.is_valid() {
        // user.username = Some("user_x".to_string());
        //user.file = Some(r#"{"path":"","url":"","is_delete":true}"#.to_string());
        //user.image = Some(r#"{"path":"","url":"","is_delete":true}"#.to_string());
        let result = user.save(None, None)?;
        println!("\n\nBoolean: {}", result.is_valid());
        println!("Hash: {}", result.hash()?);
        println!(
            "Created at: {}",
            user.created_at.clone().unwrap_or_default()
        );
        println!(
            "Updated at: {}",
            user.updated_at.clone().unwrap_or_default()
        );
        // Printing errors to the console ( for development ).
        if !result.is_valid() {
            result.print_err();
        }
        //println!("Remove document: {:?}", user.delete(None)?);
        //println!("\nObject Id:\n{:?}\n", result.object_id()?);
        // println!("\n\nWidget map:\n{:?}", result.to_wig());
        //println!("\n\nSlug:\n{}", result.to_wig().get("slug").unwrap().value);
        //println!("\n\nJson:\n{}", result.to_json()?);
        //println!("\n\nHtml:\n{}", result.to_html());
        //println!("/nJson for admin: {}/n", result.to_json_for_admin()?);
        /*
        println!(
            "Update password (false): {}",
            user.update_password("123456789", "123456789", None, None)?
        );
        println!(
            "Update password (true): {}",
            user.update_password("12345678", "123456789", None, None)?
        );
        println!(
            "Verify password (true): {}",
            user.verify_password("123456789", None)?
        );
        println!(
            "Verify password (false): {}",
            user.verify_password("12345678", None)?
        );
        */
    }

    /*
    // Remove document.
    println!("Remove document: {:?}", user.delete(None)?);

    // Get count of documents.
    println!(
        "Estimated count of documents: {}",
        mango_models::UserProfile::estimated_document_count(None)?
    );
    */

    Ok(())
}
