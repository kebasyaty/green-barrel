//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

// FIELDS ==========================================================================================
pub mod fields {
    /// Fields (field types)
    pub enum Fields {
        //
    }
}

// MODELS ==========================================================================================
pub mod models {
    /// Models (abstract methods)
    pub trait Model {
        //
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use mongodb::{
        options::{ClientOptions, StreamAddress},
        Client,
    };

    // Testing of Client
    // cargo test test_client -- --nocapture
    #[tokio::test]
    async fn test_client() {
        let client_options = ClientOptions::builder()
            .hosts(vec![StreamAddress {
                hostname: "localhost".into(),
                port: Some(27017),
            }])
            .build();

        let client = Client::with_options(client_options).unwrap();

        for db_name in client.list_database_names(None, None).await.unwrap() {
            println!("{}", db_name);
        }
    }
}
