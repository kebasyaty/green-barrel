//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

// MODELS ==========================================================================================
/// Field Types for Models
/// CheckBox
#[derive(Debug, Clone)]
pub enum CheckBox {
    Data(Option<bool>),
}
impl Default for CheckBox {
    fn default() -> Self {
        CheckBox::Data(Option::None)
    }
}
impl CheckBox {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Color
#[derive(Debug, Clone)]
pub enum Color {
    Data(Option<&'static str>),
}
impl Default for Color {
    fn default() -> Self {
        Color::Data(Option::None)
    }
}
impl Color {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Date
#[derive(Debug, Clone)]
pub enum Date {
    Data(Option<&'static str>),
}
impl Default for Date {
    fn default() -> Self {
        Date::Data(Option::None)
    }
}
impl Date {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Email
#[derive(Debug, Clone)]
pub enum Email {
    Data(Option<&'static str>),
}
impl Default for Email {
    fn default() -> Self {
        Email::Data(Option::None)
    }
}
impl Email {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// File
#[derive(Debug, Clone)]
pub enum File {
    Data(Option<&'static str>),
}
impl Default for File {
    fn default() -> Self {
        File::Data(Option::None)
    }
}
impl File {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Image
#[derive(Debug, Clone)]
pub enum Image {
    Data(Option<&'static str>),
}
impl Default for Image {
    fn default() -> Self {
        Image::Data(Option::None)
    }
}
impl Image {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberI64
#[derive(Debug, Clone)]
pub enum NumberI64 {
    Data(Option<i64>),
}
impl Default for NumberI64 {
    fn default() -> Self {
        NumberI64::Data(Option::None)
    }
}
impl NumberI64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberU64
#[derive(Debug, Clone)]
pub enum NumberU64 {
    Data(Option<i64>),
}
impl Default for NumberU64 {
    fn default() -> Self {
        NumberU64::Data(Option::None)
    }
}
impl NumberU64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// NumberF64
#[derive(Debug, Clone)]
pub enum NumberF64 {
    Data(Option<f64>),
}
impl Default for NumberF64 {
    fn default() -> Self {
        NumberF64::Data(Option::None)
    }
}
impl NumberF64 {
    pub fn get_data(&self) -> Option<f64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Password
#[derive(Debug, Clone)]
pub enum Password {
    Data(Option<&'static str>),
}
impl Default for Password {
    fn default() -> Self {
        Password::Data(Option::None)
    }
}
impl Password {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Radio
#[derive(Debug, Clone)]
pub enum Radio {
    Data(Option<bool>),
}
impl Default for Radio {
    fn default() -> Self {
        Radio::Data(Option::None)
    }
}
impl Radio {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
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
