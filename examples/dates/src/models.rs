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
    pub date: DateField,
    pub datetime: DateTimeField,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            username: TextField {
                maxlength: 150,
                required: true,
                unique: true,
                ..Default::default()
            },
            date: DateField {
                required: true,
                default: Some("1970-02-28".into()), // optional
                max: "1970-03-01".into(),           // optional
                min: "1970-01-01".into(),           // optional
                ..Default::default()
            },
            datetime: DateTimeField {
                required: true,
                default: Some("1970-02-28T00:00".into()), // optional
                max: "1970-03-01T00:00".into(),           // optional
                min: "1970-01-01T00:00".into(),           // optional
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
