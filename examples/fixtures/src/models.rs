use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    cities::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct City {
    pub city_name: TextField,
    pub descriptione: TextField, // multiline
}

impl Control for City {
    fn custom() -> Self {
        Self {
            descriptione: TextField {
                multiline: true, // for <textarea><textarea/>
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
