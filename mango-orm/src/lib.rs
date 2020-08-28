//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

// FIELDS ==========================================================================================
pub mod fields {

    /// Boolean type field
    #[derive(Default, Debug)]
    pub struct BooleanField {
        pub label: String,
        pub default: bool, // true or false
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Color type field
    #[derive(Default, Debug)]
    pub struct ColorField {
        pub label: String,
        pub default: String, // example: "#ffffff" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Date type field
    #[derive(Default, Debug)]
    pub struct DateField {
        pub label: String,
        pub default: String, // Date in UNIX format "0000-00-00" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Email type field
    #[derive(Default, Debug)]
    pub struct EmailField {
        pub label: String,
        pub default: String, // email address or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// File type field
    #[derive(Default, Debug)]
    pub struct FileField {
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Float type field
    #[derive(Default, Debug)]
    pub struct FloatField {
        pub label: String,
        pub default: f64, // number 0.0
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub choices: Vec<(String, f64)>,
    }
    /// Image type field
    #[derive(Default, Debug)]
    pub struct ImageField {
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Integer type field
    #[derive(Default, Debug)]
    pub struct IntegerField {
        pub label: String,
        pub default: i64, // number 0
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub choices: Vec<(String, i64)>,
    }
    /// IPAddress type field
    #[derive(Default, Debug)]
    pub struct IPAddressField {
        pub label: String,
        pub default: String, // IP or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Positive Integer type field
    #[derive(Default, Debug)]
    pub struct PositiveIntegerField {
        pub label: String,
        pub default: u64, // number 0
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub choices: Vec<(String, u64)>,
    }
    /// Slug type field
    #[derive(Default, Debug)]
    pub struct SlugField {
        pub label: String,
        pub default: String, // slug-line or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Text type field
    #[derive(Default, Debug)]
    pub struct TextField {
        pub label: String,
        pub default: String, // some text line or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
        pub choices: Vec<(String, String)>,
    }
    /// TextArea type field
    #[derive(Default, Debug)]
    pub struct TextAreaField {
        pub label: String,
        pub default: String, // some text or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Time type field
    #[derive(Default, Debug)]
    pub struct TimeField {
        pub label: String,
        pub default: String, // date in UNIX format "00:00:00" or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// URL type field
    #[derive(Default, Debug)]
    pub struct URLField {
        pub label: String,
        pub default: String, // URL or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Password type field
    #[derive(Default, Debug)]
    pub struct PasswordField {
        pub label: String,
        pub default: String, // password text line or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Phone type field
    #[derive(Default, Debug)]
    pub struct PhoneField {
        pub label: String,
        pub default: String, //  phone number text line or blank line
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// ForeignKey type field
    #[derive(Default, Debug)]
    pub struct ForeignKeyField {
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// ManyToMany type field
    #[derive(Default, Debug)]
    pub struct ManyToManyField {
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// OneToOne type field
    #[derive(Default, Debug)]
    pub struct OneToOneField {
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
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
