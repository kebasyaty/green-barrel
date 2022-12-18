//! Indexes.

use green_barrel::QCommons;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use std::error::Error;

use crate::models;

// Create indexes
pub async fn run_indexion(client: &Client) -> Result<(), Box<dyn Error>> {
    // Create index for User
    let options = IndexOptions::builder()
        .unique(true)
        .name("usernameIdx".to_string())
        .build();
    let index = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    models::User::create_index(client, index, None).await?;

    Ok(())
}
