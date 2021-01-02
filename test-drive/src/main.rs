use mango_orm::*;
//use mongodb::bson::doc;

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
    // Valid characters: _ a-z A-Z 0-9 ; Size: 6-48
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

    // Test Model.
    // *********************************************************************************************
    // println!("{:?}\n\n", mango_models::UserProfile::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserProfile::form_html().unwrap());

    //mango_models::UserProfile::find_one_and_delete(doc! {"username": "Rust"}, None)?;

    /*
    // Get boolean.
    println!(
        "Documents availability: {}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.bool()
    );
    // Get raw document.
    println!(
        "Raw document:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.raw_doc()
    );
    // Get prepared document.
    println!(
        "Prepared document:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.doc()?
    );
    // Get json-line.
    println!(
        "Json-linw:\n{}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?.json()?
    );
    // Get Model instance.
    println!(
        "Model instance:\n{:?}\n\n",
        mango_models::UserProfile::find_one(Some(doc! {"username": "Rust"}), None)?
            .model::<mango_models::UserProfile>()?
    );

    // Get boolean.
    println!(
        "Documents availability: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.bool()?
    );
    // Get the number of documents.
    println!(
        "The number of documents: {}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.count()?
    );
    // Get raw documents.
    println!(
        "Raw document:\n{:?}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.raw_docs()?
    );
    // Get prepared document.
    println!(
        "Prepared document:\n{:?}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.docs()?
    );
    // Get json-line.
    println!(
        "Json-linw:\n{}\n\n",
        mango_models::UserProfile::find(Some(doc! {"username": "Rust"}), None)?.json()?
    );
    */

    /*
    // Test Model.
    let mut user = mango_models::UserProfile {
        username: Some("Rust".to_string()),
        email: Some("test_1_@test.test".to_string()),
        confirm_email: Some("test_1_@test.test".to_string()),
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
    println!("Boolean: {}", result.bool());
    println!("Hash: {}", result.hash()?);
    //println!("ID: {:?}", result.id()?);
    //println!("\n\nWidget map:\n{:?}", result.wig());
    //println!("\n\nJson:\n{}", result.json()?);
    //println!("\n\nHtml:\n{}", result.html());

    // Update doc.
    user.username = Some(String::new());
    let result = user.save(None, None)?;
    println!("\n\n\nBoolean: {}", result.bool());
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
    */

    // Test Form.
    // *********************************************************************************************
    // println!("{:?}\n\n", mango_models::UserForm::form_wig().unwrap());
    // println!("{}\n\n", mango_models::UserForm::form_json().unwrap());
    // println!("{}\n\n", mango_models::UserForm::form_html().unwrap());

    println!("Form name: {}\n", mango_models::UserForm::form_name());
    println!(
        "Fields name:\n{:?}\n",
        mango_models::UserForm::fields_name()?
    );

    // Test Form.
    let user_form = mango_models::UserForm {
        username: Some("Rust".to_string()),
        email: Some("test_1_@test.test".to_string()),
        confirm_email: Some("test_1_@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
    };

    let result = user_form.check()?;

    println!("Boolean: {}\n\n", result.bool());
    //println!("Widget map:\n{:?}\n\n", result.wig());
    //println!("Json:\n{}\n\n", result.json()?);
    //println!("Html:\n{}\n", result.html());
    println!(
        "Form instance:\n{:?}\n",
        result.form::<mango_models::UserForm>()
    );

    Ok(())
}
