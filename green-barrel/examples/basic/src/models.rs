use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

// Get settings of service/sub-application.
use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    pub slug: AutoSlug,
    pub first_name: InputText,
    pub last_name: InputText,
    pub email: InputEmail,
    pub phone: InputPhone,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            ..Default::default()
        }
    }
}
