//! Global application settings.

use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub app_name: String,
    pub media_root: String,
    pub media_url: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            app_name: "App Name".into(),
            media_root: "./resources/media".into(), // the resources directory is recommended to be used as a standard
            media_url: "/media".into(),
        }
    }
}

pub fn get_app_state() -> Result<AppState, Box<dyn Error>> {
    let path = Path::new("./AppState.toml");
    if !path.is_file() {
        fs::File::create(path)?;
        let cfg = AppState::default();
        confy::store_path(path, cfg)?;
    }
    Ok(confy::load_path::<AppState>(path)?)
}
