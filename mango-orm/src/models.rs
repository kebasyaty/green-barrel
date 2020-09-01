//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

// MODELS ==========================================================================================
/// Field Types for Models
#[derive(Debug, Clone)]
pub enum ModelsFieldType {
    CheckBox,
    Color,
    Date,
    Email,
    File,
    Hidden,
    Image,
    Number,
    Password,
    Radio,
    Range,
    Tel,
    Text,
    Time,
    Url,
    TextArea,
    Select,
    ForeignKey,
    ManyToMany,
    OneToOne,
}
impl Default for ModelsFieldType {
    fn default() -> Self {
        ModelsFieldType::Text
    }
}
impl ModelsFieldType {
    pub fn get_type(&self) -> String {
        match self {
            Self::CheckBox => "checkbox".to_string(),
            Self::Color => "color".to_string(),
            Self::Date => "date".to_string(),
            Self::Email => "email".to_string(),
            Self::File => "file".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::Image => "image".to_string(),
            Self::Number => "number".to_string(),
            Self::Password => "password".to_string(),
            Self::Radio => "radio".to_string(),
            Self::Range => "range".to_string(),
            Self::Tel => "tel".to_string(),
            Self::Text => "text".to_string(),
            Self::Time => "time".to_string(),
            Self::Url => "url".to_string(),
            Self::TextArea => "textarea".to_string(),
            Self::Select => "select".to_string(),
            Self::ForeignKey => "m2o".to_string(),
            Self::ManyToMany => "m2m".to_string(),
            Self::OneToOne => "o2o".to_string(),
        }
    }
}

/// Abstract Model ---------------------------------------------------------------------------------
pub trait Moel {
    //
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
