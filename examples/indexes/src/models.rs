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
    pub email: EmailField,
    pub slug: SlugField,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            username: TextField {
                label: "Username".into(),
                placeholder: "Enter your username".into(),
                regex: r"^[a-zA-Z\d_@.+]{1,150}$".into(),
                regex_err_msg: "Allowed chars: a-z A-Z 0-9 _ @ . +".into(),
                minlength: 1,
                maxlength: 150,
                required: true,
                unique: true,
                hint: "Allowed chars: a-z A-Z 0-9 _ @ . +".into(),
                ..Default::default()
            },
            email: EmailField {
                label: "E-mail".into(),
                required: true,
                unique: true,
                maxlength: 320,
                ..Default::default()
            },
            slug: SlugField {
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
