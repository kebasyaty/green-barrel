use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: Text,
    pub slug: Slug,
    pub first_name: Text,
    pub last_name: Text,
    pub email: Email,
    pub phone: Phone,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            ..Default::default()
        }
    }
}
