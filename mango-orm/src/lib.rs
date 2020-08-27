//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

// WIDGET ==========================================================================================
pub mod widgets {
    /// Boolean widgets
    pub enum BooleanWidget {
        CheckboxInput,
        RadioInput,
    }
    /// Color widgets
    pub enum ColorWidget {
        ColorInput,
    }
    /// Date widgets
    pub enum DateWidget {
        DateInput,
    }
    /// Email widgets
    pub enum EmailWidget {
        EmailInput,
    }
    /// File widgets
    pub enum FileWidget {
        FileInput,
    }
    /// Float widgets
    pub enum FloatWidget {
        NumberInput,
    }
    /// Image widgets
    pub enum ImageWidget {
        FileInput,
    }
    /// Integer widgets
    pub enum IntegerWidget {
        NumberInput,
    }
    /// IPAddress widgets
    pub enum IPAddressWidget {
        TextInput,
    }
    /// Positive Integer widgets
    pub enum PositiveIntegerWidget {
        NumberInput,
    }
    /// Slug widgets
    pub enum SlugWidget {
        TextInput,
    }
    /// Text widgets
    pub enum TextWidget {
        TextInput,
    }
    /// TextArea widgets
    pub enum TextAreaWidget {
        TextInput,
    }
    /// Time widgets
    pub enum TimeWidget {
        TimeInput,
    }
    /// URL widgets
    pub enum URLWidget {
        UrlInput,
    }
}

// FIELDS ==========================================================================================
pub mod fields {
    use super::widgets::{
        BooleanWidget, ColorWidget, DateWidget, EmailWidget, FileWidget, FloatWidget,
        IPAddressWidget, ImageWidget, IntegerWidget, PositiveIntegerWidget, SlugWidget,
        TextAreaWidget, TextWidget, TimeWidget, URLWidget,
    };

    /// Boolean type field
    pub struct BooleanField {
        pub widget: BooleanWidget,
        pub label: String,
        pub default: bool, // true or false
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Color type field
    pub struct ColorField {
        pub widget: ColorWidget,
        pub label: String,
        pub default: String, // example: "#ffffff" or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Date type field
    pub struct DateField {
        pub widget: DateWidget,
        pub label: String,
        pub default: String, // Date in UNIX format "0000-00-00" or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Email type field
    pub struct EmailField {
        pub widget: EmailWidget,
        pub label: String,
        pub default: String, // email address or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// File type field
    pub struct FileField {
        pub widget: FileWidget,
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Float type field
    pub struct FloatField {
        pub widget: FloatWidget,
        pub label: String,
        pub default: f64, // number 0.0
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Image type field
    pub struct ImageField {
        pub widget: ImageWidget,
        pub label: String,
        pub default: String, // media_url plus file path or blank line
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// Integer type field
    pub struct IntegerField {
        pub widget: IntegerWidget,
        pub label: String,
        pub default: i64, // number 0
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// IPAddress type field
    pub struct IPAddressField {
        pub widget: IPAddressWidget,
        pub label: String,
        pub default: String, // IP or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Positive Integer type field
    pub struct PositiveIntegerField {
        pub widget: PositiveIntegerWidget,
        pub label: String,
        pub default: u64, // number 0
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Slug type field
    pub struct SlugField {
        pub widget: SlugWidget,
        pub label: String,
        pub default: String, // slug-line or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Text type field
    pub struct TextField {
        pub widget: TextWidget,
        pub label: String,
        pub default: String, // some text line or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// TextArea type field
    pub struct TextAreaField {
        pub widget: TextAreaWidget,
        pub label: String,
        pub default: String, // some text or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// Time type field
    pub struct TimeField {
        pub widget: TimeWidget,
        pub label: String,
        pub default: String, // Date in UNIX format "00:00:00" or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }
    /// URL type field
    pub struct URLField {
        pub widget: URLWidget,
        pub label: String,
        pub default: String, // URL or blank line
        pub required: bool,
        pub hint: String,
        pub unique: bool,
        pub hidden: bool,
    }

    /// Fields (field types)
    pub enum Fields {
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
