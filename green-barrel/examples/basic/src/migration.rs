use crate::{models, settings};
use green_barrel::{Caching, Monitor};
use mongodb::Client;
use std::error::Error;


async fn run_migration(client: &Client) -> Result<(), Box<dyn Error>> {
    // Caching metadata.
    models::User::caching(client).await?;

    // Monitor initialization.
    let monitor = Monitor {
        app_name: settings::APP_NAME,
        unique_app_key: settings::UNIQUE_APP_KEY,
        // For register models.
        model_key_list: vec![models::User::key()?],
    };
    monitor.migrat(client).await?;

    Ok(())
}
