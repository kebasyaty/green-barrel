use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: TextField,
    pub slug: SlugField,
    pub first_name: TextField,
    pub last_name: TextField,
    pub email: EmailField,
    pub phone: PhoneField,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            ..Default::default()
        }
    }
}
