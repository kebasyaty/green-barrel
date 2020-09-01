//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

// MODELS ==========================================================================================
/// Field Types for Models
#[derive(Debug, Clone)]
pub enum ModelsFieldType {
    CheckBox(Option<bool>),
    Color(Option<String>),
    Date(Option<String>),
    Email(Option<String>),
    File(Option<String>),
    Image(Option<String>),
    NumberI64(Option<i64>),
    NumberU64(Option<u64>),
    NumberF64(Option<f64>),
    Password(Option<String>),
    Radio(Option<bool>),
    Tel(Option<String>),
    Text(Option<String>),
    Time(Option<String>),
    Url(Option<String>),
    TextArea(Option<String>),
    SelectText(Option<String>),
    SelectI64(Option<i64>),
    SelectU64(Option<u64>),
    SelectF64(Option<f64>),
    ForeignKey(Option<String>),
    ManyToMany(Option<String>),
    OneToOne(Option<String>),
}
impl Default for ModelsFieldType {
    fn default() -> Self {
        ModelsFieldType::Text(Option::None)
    }
}
impl ModelsFieldType {
    pub fn get_type(&self) -> String {
        match self {
            Self::CheckBox(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Color(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Date(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Email(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::File(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Image(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::NumberI64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::NumberU64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::NumberF64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Password(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Radio(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Tel(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Text(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Time(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::Url(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::TextArea(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::SelectText(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::SelectI64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::SelectU64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::SelectF64(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::ForeignKey(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::ManyToMany(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
            Self::OneToOne(data) => match data {
                Some(data) => data.to_string(),
                None => "Option::None".to_string(),
            },
        }
    }
}

/// Abstract Model ---------------------------------------------------------------------------------
pub trait Model {
    //
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
