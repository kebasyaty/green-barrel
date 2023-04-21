//! For control of fields on the server and client side.

// bool
pub mod bool;
// hidden
pub mod hash;
pub mod hidden_date_time;
// text
pub mod color;
pub mod date;
pub mod date_time;
pub mod image;
pub mod ip;
pub mod password;
pub mod phone;
pub mod slug;
pub mod text;
pub mod url;
// file
pub mod email;
pub mod file;
// number
pub mod number_f64;
pub mod number_i32;
pub mod number_i64;
pub mod number_u32;
// choice
pub mod choice_f64;
pub mod choice_i32;
pub mod choice_i64;
pub mod choice_text;
pub mod choice_u32;
pub mod select_f64_dyn;
pub mod select_f64_mult;
pub mod select_f64_mult_dyn;
pub mod select_i32_dyn;
pub mod select_i32_mult;
pub mod select_i32_mult_dyn;
pub mod select_i64_dyn;
pub mod select_i64_mult;
pub mod select_i64_mult_dyn;
pub mod select_text_dyn;
pub mod select_text_mult;
pub mod select_text_mult_dyn;
pub mod select_u32_dyn;
pub mod select_u32_mult;
pub mod select_u32_mult_dyn;

pub use {
    crate::fields::bool::Bool, crate::fields::image::Image, crate::fields::slug::Slug,
    choice_f64::ChoiceF64, choice_i32::ChoiceI32, choice_i64::ChoiceI64, choice_text::ChoiceText,
    choice_u32::ChoiceU32, color::Color, date::Date, date_time::DateTime, email::Email, file::File,
    hash::Hash, hidden_date_time::HiddenDateTime, ip::IP, number_f64::F64, number_i32::I32,
    number_i64::I64, number_u32::U32, password::Password, phone::Phone,
    select_f64_dyn::SelectF64Dyn, select_f64_mult::SelectF64Mult,
    select_f64_mult_dyn::SelectF64MultDyn, select_i32_dyn::SelectI32Dyn,
    select_i32_mult::SelectI32Mult, select_i32_mult_dyn::SelectI32MultDyn,
    select_i64_dyn::SelectI64Dyn, select_i64_mult::SelectI64Mult,
    select_i64_mult_dyn::SelectI64MultDyn, select_text_dyn::SelectTextDyn,
    select_text_mult::SelectTextMult, select_text_mult_dyn::SelectTextMultDyn,
    select_u32_dyn::SelectU32Dyn, select_u32_mult::SelectU32Mult,
    select_u32_mult_dyn::SelectU32MultDyn, text::Text, url::Url,
};
