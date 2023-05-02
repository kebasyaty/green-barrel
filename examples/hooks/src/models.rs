use async_trait::async_trait;
use green_barrel::*;
use metamorphose::Model;
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model(is_use_hooks = true)]
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

#[async_trait(?Send)]
impl Hooks for User {
    async fn pre_create(&self, _client: &Client) {
        println!("!!!Pre Create!!!");
    }
    //
    async fn post_create(&self, _client: &Client) {
        println!("!!!Post Create!!!");
    }
    //
    async fn pre_update(&self, _client: &Client) {
        println!("!!!Pre Update!!!");
    }
    //
    async fn post_update(&self, _client: &Client) {
        println!("!!!Post Update!!!");
    }
    //
    async fn pre_delete(&self, _client: &Client) {
        println!("!!!Pre Delet!!!");
    }
    //
    async fn post_delete(&self, _client: &Client) {
        println!("!!!Post Delet!!!");
    }
}
