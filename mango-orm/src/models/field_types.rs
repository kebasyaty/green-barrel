//! # Field Types
//!
//! Field Types for Models.

// FIELD TYPES =====================================================================================
/// Field Types for Models
/// Input CheckBox
#[derive(Debug, Clone)]
pub enum InputCheckBox {
    Data(Option<bool>),
}
impl Default for InputCheckBox {
    fn default() -> Self {
        InputCheckBox::Data(Option::None)
    }
}
impl InputCheckBox {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Color
#[derive(Debug, Clone)]
pub enum InputColor {
    Data(Option<&'static str>),
}
impl Default for InputColor {
    fn default() -> Self {
        InputColor::Data(Option::None)
    }
}
impl InputColor {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Date
#[derive(Debug, Clone)]
pub enum InputDate {
    Data(Option<&'static str>),
}
impl Default for InputDate {
    fn default() -> Self {
        InputDate::Data(Option::None)
    }
}
impl InputDate {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Email
#[derive(Debug, Clone)]
pub enum InputEmail {
    Data(Option<&'static str>),
}
impl Default for InputEmail {
    fn default() -> Self {
        InputEmail::Data(Option::None)
    }
}
impl InputEmail {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input File
#[derive(Debug, Clone)]
pub enum InputFile {
    Data(Option<&'static str>),
}
impl Default for InputFile {
    fn default() -> Self {
        InputFile::Data(Option::None)
    }
}
impl InputFile {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Image
#[derive(Debug, Clone)]
pub enum InputImage {
    Data(Option<&'static str>),
}
impl Default for InputImage {
    fn default() -> Self {
        InputImage::Data(Option::None)
    }
}
impl InputImage {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input NumberI64
#[derive(Debug, Clone)]
pub enum InputNumberI64 {
    Data(Option<i64>),
}
impl Default for InputNumberI64 {
    fn default() -> Self {
        InputNumberI64::Data(Option::None)
    }
}
impl InputNumberI64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input NumberU64
#[derive(Debug, Clone)]
pub enum InputNumberU64 {
    Data(Option<i64>),
}
impl Default for InputNumberU64 {
    fn default() -> Self {
        InputNumberU64::Data(Option::None)
    }
}
impl InputNumberU64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input NumberF64
#[derive(Debug, Clone)]
pub enum InputNumberF64 {
    Data(Option<f64>),
}
impl Default for InputNumberF64 {
    fn default() -> Self {
        InputNumberF64::Data(Option::None)
    }
}
impl InputNumberF64 {
    pub fn get_data(&self) -> Option<f64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Password
#[derive(Debug, Clone)]
pub enum InputPassword {
    Data(Option<&'static str>),
}
impl Default for InputPassword {
    fn default() -> Self {
        InputPassword::Data(Option::None)
    }
}
impl InputPassword {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Radio
#[derive(Debug, Clone)]
pub enum InputRadio {
    Data(Option<bool>),
}
impl Default for InputRadio {
    fn default() -> Self {
        InputRadio::Data(Option::None)
    }
}
impl InputRadio {
    pub fn get_data(&self) -> Option<bool> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Tel
#[derive(Debug, Clone)]
pub enum InputTel {
    Data(Option<&'static str>),
}
impl Default for InputTel {
    fn default() -> Self {
        InputTel::Data(Option::None)
    }
}
impl InputTel {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Text
#[derive(Debug, Clone)]
pub enum InputText {
    Data(Option<&'static str>),
}
impl Default for InputText {
    fn default() -> Self {
        InputText::Data(Option::None)
    }
}
impl InputText {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Time
#[derive(Debug, Clone)]
pub enum InputTime {
    Data(Option<&'static str>),
}
impl Default for InputTime {
    fn default() -> Self {
        InputTime::Data(Option::None)
    }
}
impl InputTime {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// Input Url
#[derive(Debug, Clone)]
pub enum InputUrl {
    Data(Option<&'static str>),
}
impl Default for InputUrl {
    fn default() -> Self {
        InputUrl::Data(Option::None)
    }
}
impl InputUrl {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// TextArea
#[derive(Debug, Clone)]
pub enum TextArea {
    Data(Option<&'static str>),
}
impl Default for TextArea {
    fn default() -> Self {
        TextArea::Data(Option::None)
    }
}
impl TextArea {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectText
#[derive(Debug, Clone)]
pub enum SelectText {
    Data(Option<&'static str>),
}
impl Default for SelectText {
    fn default() -> Self {
        SelectText::Data(Option::None)
    }
}
impl SelectText {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectI64
#[derive(Debug, Clone)]
pub enum SelectI64 {
    Data(Option<i64>),
}
impl Default for SelectI64 {
    fn default() -> Self {
        SelectI64::Data(Option::None)
    }
}
impl SelectI64 {
    pub fn get_data(&self) -> Option<i64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectU64
#[derive(Debug, Clone)]
pub enum SelectU64 {
    Data(Option<u64>),
}
impl Default for SelectU64 {
    fn default() -> Self {
        SelectU64::Data(Option::None)
    }
}
impl SelectU64 {
    pub fn get_data(&self) -> Option<u64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// SelectF64
#[derive(Debug, Clone)]
pub enum SelectF64 {
    Data(Option<f64>),
}
impl Default for SelectF64 {
    fn default() -> Self {
        SelectF64::Data(Option::None)
    }
}
impl SelectF64 {
    pub fn get_data(&self) -> Option<f64> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// ForeignKey
#[derive(Debug, Clone)]
pub enum ForeignKey {
    Data(Option<&'static str>),
}
impl Default for ForeignKey {
    fn default() -> Self {
        ForeignKey::Data(Option::None)
    }
}
impl ForeignKey {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// ManyToMany
#[derive(Debug, Clone)]
pub enum ManyToMany {
    Data(Option<&'static str>),
}
impl Default for ManyToMany {
    fn default() -> Self {
        ManyToMany::Data(Option::None)
    }
}
impl ManyToMany {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}
/// OneToOne
#[derive(Debug, Clone)]
pub enum OneToOne {
    Data(Option<&'static str>),
}
impl Default for OneToOne {
    fn default() -> Self {
        OneToOne::Data(Option::None)
    }
}
impl OneToOne {
    pub fn get_data(&self) -> Option<&'static str> {
        match self {
            Self::Data(data) => *data,
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
