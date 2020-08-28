//! # Fields
//!
//! Field types for models.

/// Boolean type field
/// Use for:
/// <input type="checkbox">
/// <input type="radio">
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
/// Use for:
/// <input type="color">
/// <input type="text">
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
/// Use for:
/// <input type="date">
/// <input type="text">
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
/// Use for:
/// <input type="email">
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
/// Use for:
/// <input type="file">
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
/// Use for:
/// <input type="number">
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
/// Use for:
/// <input type="file">
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
/// Use for:
/// <input type="number">
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
/// Use for:
/// 1. <input type="text">
/// 2. <input type="text" size="16"
/// pattern="^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])$">
/// 3. https://stackoverflow.com/questions/49306970/correct-input-type-for-ip-address
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
/// Use for:
/// <input type="number">
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
/// Use for:
/// <input type="text">
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
/// Use for:
/// <input type="text">
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
/// Use for:
/// <textarea></textarea>
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
/// Use for:
/// <input type="time">
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
/// Use for:
/// <input type="url">
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
/// Use for:
/// <input type="password">
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
/// Use for:
/// <input type="tel">
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
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct ForeignKeyField {
    pub label: String,
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub hidden: bool,
}
/// ManyToMany type field
/// Use for:
/// <select multiple></select>
#[derive(Default, Debug)]
pub struct ManyToManyField {
    pub label: String,
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub hidden: bool,
}
/// OneToOne type field
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct OneToOneField {
    pub label: String,
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub hidden: bool,
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
