mod migration;
mod models;
mod settings;

use green_barrel::*;
use mongodb::Client;
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

    // Get Cities
    // *********************************************************************************************
    if let Some(city_list) = models::City::find_many_to_doc_list(&client, None, None).await? {
        println!("{:#?}", city_list);
    } else {
        panic!("Documents is missing!");
    }

    Ok(())
}
