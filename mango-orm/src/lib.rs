//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

// FIELDS ==========================================================================================
pub mod fields {
    /// Fields (field types)
    pub enum Fields {
        BooleanField(bool),
        DateField(String),
        EmailField(String),
        FileField(String),
        FloatField(f64),
        ImageField(String),
        IntegerField(i64),
        PositiveIntegerField(u64),
        SlugField(String),
        TextField(String),
        TextAreaField(String),
        TimeField(String),
        URLField(String),
        ForeignKeyField(String),
        ManyToManyField(String),
        OneToOneField(String),
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

    // Testing of Client ---------------------------------------------------------------------------
    // cargo test test_client -- --nocapture
    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        let client_options = ClientOptions::builder()
            .hosts(vec![StreamAddress {
                hostname: "localhost".into(),
                port: Some(27017),
            }])
            .build();

        let client = Client::with_options(client_options)?;

        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        Ok(())
    }
}
