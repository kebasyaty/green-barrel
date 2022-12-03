//! Global application settings.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, path::Path};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
    pub media_root: String,
    pub media_url: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            media_root: String::from("./media"),
            media_url: String::from("/media"),
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

pub fn get_media_dir() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let app_state = get_app_state()?;
    Ok([
        ("media_root".into(), app_state.media_root),
        ("media_url".into(), app_state.media_url),
    ]
    .iter()
    .cloned()
    .collect())
}
