mod migration;
mod models;
mod settings;

use green_barrel::*;
use mongodb::{bson::doc, Client};
use serde_json::json;
use std::error::Error;

#[macro_use]
extern crate rust_i18n;

i18n!("locales");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Change globally current locale
    // defaule = en-us
    rust_i18n::set_locale("ru");
    // Init Client
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // Add items of selection for dynamic field
    // A more convenient use of these types of fields is implemented in the Green Panel project:
    // https://github.com/kebasyaty/green-panel
    // *********************************************************************************************
    let dyn_data = json! ({
         "field_name": "select_text_mult_dyn",
         "value": "windows",
         "title": "Windows",
         "is_delete": false
    });
    models::User::update_dyn_field(&client, dyn_data).await?;
    let dyn_data = json! ({
         "field_name": "select_text_mult_dyn",
         "value": "linux",
         "title": "Linux",
         "is_delete": false
    });
    models::User::update_dyn_field(&client, dyn_data).await?;
    let dyn_data = json! ({
         "field_name": "select_text_mult_dyn",
         "value": "mac os",
         "title": "Mac OS",
         "is_delete": false
    });
    models::User::update_dyn_field(&client, dyn_data).await?;

    // Create User
    // *********************************************************************************************
    let mut user = models::User::new().await?;
    user.username.set("user_1");
    user.select_text_mult_dyn
        .set(vec!["windows", "linux", "mac os"]);

    // Save User
    // *********************************************************************************************
    let output_data = user.save(&client, None, None).await?;
    //user = output_data.update()?;

    if output_data.is_valid() {
        // Get doc
        let filter = doc! {"username": "user_1"};
        if let Some(user) = models::User::find_one_to_doc(&client, filter, None).await? {
            println!("{:#?}", user);
        } else {
            panic!("Document is missing!");
        }
    } else {
        // Printing errors to the console ( for development )
        output_data.print_err();
    }

    Ok(())
}
