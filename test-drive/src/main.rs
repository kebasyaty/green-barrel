use mango_orm::*;
use mongodb::bson::doc;

mod mango_models;

// Migration Service `Mango`.
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
        // Register models.
        models: vec![
            mango_models::User::meta()?,
            mango_models::UserProfile::meta()?,
        ],
    };
    monitor.migrat();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Run migration.
    mango_migration()?;

    // println!("{:?}\n\n", mango_models::UserProfile::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_html().unwrap());

    // Get the number of documents.
    println!(
        "Number of documents: {:?}",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.count()?
    );
    // Get boolean.
    println!(
        "Documents availability: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.bool()?
    );
    // Get json-line.
    println!(
        "Json-linw: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.json()?
    );

    // Test Model.
    let mut user = mango_models::UserProfile {
        username: Some("Rust".to_string()),
        email: Some("test_38_@test.test".to_string()),
        confirm_email: Some("test_38_@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
        date: Some("2020-12-19".to_string()),
        datetime: Some("2020-12-19T15:57".to_string()),
        num_i32: Some(-32),
        num_u32: Some(32),
        num_i64: Some(-64),
        num_f64: Some(-64.64),
        ..Default::default() // or initialize the `hash` field - { hash: Some(String::new()) }
    };

    // Create doc.
    let result = user.save(None, None)?;
    println!("Bool: {}", result.bool());
    println!("Hash: {}", result.hash()?);
    //println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());

    // Update doc.
    user.username = Some(String::new());
    let result = user.save(None, None)?;
    println!("\n\n\nBool: {}", result.bool());
    println!("Hash: {}", result.hash()?);
    //println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());

    // Remove document.
    println!("Remove document: {:?}", user.delete(None)?);

    // Get count of documents.
    println!(
        "Estimated count of documents: {}",
        mango_models::UserProfile::estimated_document_count(None)?
    );

    Ok(())
}
