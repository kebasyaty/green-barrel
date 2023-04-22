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
    pub email: Email,
    pub slug: Slug,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: Text {
                label: "Username".into(),
                unique: true,
                required: true,
                maxlength: 150,
                hint: "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150".into(),
                ..Default::default()
            },
            email: Email {
                label: "E-mail".into(),
                required: true,
                unique: true,
                maxlength: 320,
                ..Default::default()
            },
            slug: Slug {
                label: "Slug".into(),
                is_hide: true,
                hint: "To create a human readable url".into(),
                slug_sources: vec!["hash".into(), "username".into()],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
