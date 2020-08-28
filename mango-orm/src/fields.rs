//! # Fields
//!
//! Field types for models.

// FIELDS FOR CHOICES ITEMS ========================================================================
/// Select string type field
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct SelectStringField {
    pub label: String,
    pub default: String, // some text
    pub disabled: bool,
    pub multiple: bool,
    pub required: bool,
    pub hint: String,
    pub select: Vec<(String, String)>,
}

/// Select i64 type field
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct SelectIntegerField {
    pub label: String,
    pub default: i64, // number 0_i64
    pub disabled: bool,
    pub multiple: bool,
    pub required: bool,
    pub hint: String,
    pub select: Vec<(String, i64)>,
}

/// Select u64 type field
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct SelectPositiveIntegerField {
    pub label: String,
    pub default: u64, // number 0_u64
    pub disabled: bool,
    pub multiple: bool,
    pub required: bool,
    pub hint: String,
    pub select: Vec<(String, u64)>,
}

/// Select f64 type field
/// Use for:
/// <select></select>
#[derive(Default, Debug)]
pub struct SelectFloatField {
    pub label: String,
    pub default: f64, // number 0.0_f64
    pub disabled: bool,
    pub multiple: bool,
    pub required: bool,
    pub hint: String,
    pub select: Vec<(String, f64)>,
}

// STANDARD FIELDS =================================================================================
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
    pub default: f64, // number 0.0_f64
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
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
    pub default: i64, // number 0_i64
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
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
    pub default: u64, // number 0_u64
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
}
/// Slug type field
/// Use for:
/// <input type="text">
#[derive(Default, Debug)]
pub struct SlugField {
    pub label: String,
    pub default: String, // slug-text or blank line
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
    pub default: String, // some text or blank line
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
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
    pub default: String, // password text or blank line
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
    pub default: String, //  phone number or blank line
    pub readonly: bool,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
}

// RELATIONSHIP FIELDS =============================================================================
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
    use super::*;

    #[test]
    fn test_boolean_field() {
        let field: BooleanField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, false);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_color_field() {
        let field: ColorField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_date_field() {
        let field: DateField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_email_field() {
        let field: EmailField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_file_field() {
        let field: FileField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_float_field() {
        let field: FloatField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0.0_f64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_image_field() {
        let field: ImageField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_integer_field() {
        let field: IntegerField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_i64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_ip_address_field() {
        let field: IPAddressField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_positive_integer_field() {
        let field: PositiveIntegerField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, 0_u64);
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_slug_field() {
        let field: SlugField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_text_field() {
        let field: TextField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_text_area_field() {
        let field: TextAreaField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_time_field() {
        let field: TimeField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_url_field() {
        let field: URLField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_password_field() {
        let field: PasswordField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_phone_field() {
        let field: PhoneField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.default, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.unique, false);
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_foreign_key_field() {
        let field: ForeignKeyField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_many_to_many_field() {
        let field: ManyToManyField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }

    #[test]
    fn test_one_to_one_field() {
        let field: OneToOneField = Default::default();
        assert_eq!(field.label, "".to_string());
        assert_eq!(field.readonly, false);
        assert_eq!(field.required, false);
        assert_eq!(field.hint, "".to_string());
        assert_eq!(field.hidden, false);
    }
}
