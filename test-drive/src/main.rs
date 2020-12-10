use mango_orm::*;

mod mango_models;

// Migration Service `Mango`
fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
    // Caching MongoDB clients;
    DB_MAP_CLIENT_NAMES.lock()?.insert(
        "default".to_string(),
        mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
    );
    DB_MAP_CLIENT_NAMES.lock()?.insert(
        "default_2".to_string(),
        mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
    );
    // KEYWORD it is recommended not to change.
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    // Example: "PROJECT_NAME_7rzg_cfqQB3B7q7T"
    static KEYWORD: &str = "PROJECT_NAME_7rzg_cfqQB3B7q7T";
    let monitor = Monitor {
        keyword: KEYWORD,
        // Register models
        models: vec![
            mango_models::User::meta()?,
            mango_models::UserProfile::meta()?,
        ],
    };
    monitor.migrat();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run migration
    mango_migration()?;

    // println!("{:?}\n\n", mango_models::UserProfile::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_html().unwrap());

    let mut user = mango_models::UserProfile {
        username: Some("Rust".to_string()),
        email: Some("test_15_@test.test".to_string()),
        confirm_email: Some("test_15_@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
        ..Default::default()
    };

    // Create doc
    let result = user.save()?;
    println!("Bool: {}", result.bool()?);
    println!("Hash: {:?}", result.hash()?);
    println!("\n\nWidget map:\n{:?}", result.wig()?);
    println!("\n\nJson:\n{:?}", result.json()?);
    println!("\n\nHtml:\n{:?}", result.html()?);

    // Update doc
    user.username = Some(String::new());
    let result = user.save()?;
    println!("\n\n\nBool: {}", result.bool()?);
    println!("Hash: {:?}", result.hash()?);
    println!("\n\nWidget map:\n{:?}", result.wig()?);
    println!("\n\nJson:\n{:?}", result.json()?);
    println!("\n\nHtml:\n{:?}", result.html()?);

    Ok(())
}
