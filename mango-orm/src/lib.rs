//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

// WIDGET ==========================================================================================
pub mod widgets {
    /// Boolean widgets
    #[derive(Debug)]
    pub enum BooleanWidget {
        CheckboxInput,
        RadioInput,
    }
    impl Default for BooleanWidget {
        fn default() -> Self {
            BooleanWidget::CheckboxInput
        }
    }
    /// Color widgets
    #[derive(Debug)]
    pub enum ColorWidget {
        ColorInput,
    }
    impl Default for ColorWidget {
        fn default() -> Self {
            ColorWidget::ColorInput
        }
    }
    /// Date widgets
    #[derive(Debug)]
    pub enum DateWidget {
        DateInput,
    }
    impl Default for DateWidget {
        fn default() -> Self {
            DateWidget::DateInput
        }
    }
    /// Email widgets
    #[derive(Debug)]
    pub enum EmailWidget {
        EmailInput,
    }
    impl Default for EmailWidget {
        fn default() -> Self {
            EmailWidget::EmailInput
        }
    }
    /// File widgets
    #[derive(Debug)]
    pub enum FileWidget {
        FileInput,
    }
    impl Default for FileWidget {
        fn default() -> Self {
            FileWidget::FileInput
        }
    }
    /// Float widgets
    #[derive(Debug)]
    pub enum FloatWidget {
        NumberInput,
        RangeInput,
        Select,
    }
    impl Default for FloatWidget {
        fn default() -> Self {
            FloatWidget::NumberInput
        }
    }
    /// Image widgets
    #[derive(Debug)]
    pub enum ImageWidget {
        FileInput,
    }
    impl Default for ImageWidget {
        fn default() -> Self {
            ImageWidget::FileInput
        }
    }
    /// Integer widgets
    #[derive(Debug)]
    pub enum IntegerWidget {
        NumberInput,
        RangeInput,
        Select,
    }
    impl Default for IntegerWidget {
        fn default() -> Self {
            IntegerWidget::NumberInput
        }
    }
    /// IPAddress widgets
    #[derive(Debug)]
    pub enum IPAddressWidget {
        TextInput,
    }
    impl Default for IPAddressWidget {
        fn default() -> Self {
            IPAddressWidget::TextInput
        }
    }
    /// Positive Integer widgets
    #[derive(Debug)]
    pub enum PositiveIntegerWidget {
        NumberInput,
        RangeInput,
        Select,
    }
    impl Default for PositiveIntegerWidget {
        fn default() -> Self {
            PositiveIntegerWidget::NumberInput
        }
    }
    /// Slug widgets
    #[derive(Debug)]
    pub enum SlugWidget {
        TextInput,
    }
    impl Default for SlugWidget {
        fn default() -> Self {
            SlugWidget::TextInput
        }
    }
    /// Text widgets
    #[derive(Debug)]
    pub enum TextWidget {
        TextInput,
        Select,
    }
    impl Default for TextWidget {
        fn default() -> Self {
            TextWidget::TextInput
        }
    }
    /// TextArea widgets
    #[derive(Debug)]
    pub enum TextAreaWidget {
        TextArea,
    }
    impl Default for TextAreaWidget {
        fn default() -> Self {
            TextAreaWidget::TextArea
        }
    }
    /// Time widgets
    #[derive(Debug)]
    pub enum TimeWidget {
        TimeInput,
    }
    impl Default for TimeWidget {
        fn default() -> Self {
            TimeWidget::TimeInput
        }
    }
    /// URL widgets
    #[derive(Debug)]
    pub enum URLWidget {
        UrlInput,
    }
    impl Default for URLWidget {
        fn default() -> Self {
            URLWidget::UrlInput
        }
    }
    /// Password widgets
    #[derive(Debug)]
    pub enum PasswordWidget {
        PasswordInput,
    }
    impl Default for PasswordWidget {
        fn default() -> Self {
            PasswordWidget::PasswordInput
        }
    }
    /// Phone widgets
    #[derive(Debug)]
    pub enum PhoneWidget {
        TelInput,
    }
    impl Default for PhoneWidget {
        fn default() -> Self {
            PhoneWidget::TelInput
        }
    }
    /// ForeignKey widgets
    #[derive(Debug)]
    pub enum ForeignKeyWidget {
        Select,
    }
    impl Default for ForeignKeyWidget {
        fn default() -> Self {
            ForeignKeyWidget::Select
        }
    }
    /// ManyToMany widgets
    #[derive(Debug)]
    pub enum ManyToManyWidget {
        Select,
    }
    impl Default for ManyToManyWidget {
        fn default() -> Self {
            ManyToManyWidget::Select
        }
    }
    /// OneToOne widgets
    #[derive(Debug)]
    pub enum OneToOneWidget {
        Select,
    }
    impl Default for OneToOneWidget {
        fn default() -> Self {
            OneToOneWidget::Select
        }
    }
}

// FIELDS ==========================================================================================
pub mod fields {
    use super::widgets::{
        BooleanWidget, ColorWidget, DateWidget, EmailWidget, FileWidget, FloatWidget,
        ForeignKeyWidget, IPAddressWidget, ImageWidget, IntegerWidget, ManyToManyWidget,
        OneToOneWidget, PasswordWidget, PhoneWidget, PositiveIntegerWidget, SlugWidget,
        TextAreaWidget, TextWidget, TimeWidget, URLWidget,
    };

    /// Boolean type field
    #[derive(Default, Debug)]
    pub struct BooleanField {
        pub widget: BooleanWidget,
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
        pub widget: ColorWidget,
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
        pub widget: DateWidget,
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
        pub widget: EmailWidget,
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
        pub widget: FileWidget,
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
        pub widget: FloatWidget,
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
        pub widget: ImageWidget,
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
        pub widget: IntegerWidget,
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
        pub widget: IPAddressWidget,
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
        pub widget: PositiveIntegerWidget,
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
        pub widget: SlugWidget,
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
        pub widget: TextWidget,
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
        pub widget: TextAreaWidget,
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
        pub widget: TimeWidget,
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
        pub widget: URLWidget,
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
        pub widget: PasswordWidget,
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
        pub widget: PhoneWidget,
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
        pub widget: ForeignKeyWidget,
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// ManyToMany type field
    #[derive(Default, Debug)]
    pub struct ManyToManyField {
        pub widget: ManyToManyWidget,
        pub label: String,
        pub readonly: bool,
        pub required: bool,
        pub hint: String,
        pub hidden: bool,
    }
    /// OneToOne type field
    #[derive(Default, Debug)]
    pub struct OneToOneField {
        pub widget: OneToOneWidget,
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
