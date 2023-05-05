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
pub mod choice_f64_dyn;
pub mod choice_f64_mult;
pub mod choice_f64_mult_dyn;
pub mod choice_i32;
pub mod choice_i32_dyn;
pub mod choice_i32_mult;
pub mod choice_i32_mult_dyn;
pub mod choice_i64;
pub mod choice_i64_dyn;
pub mod choice_i64_mult;
pub mod choice_i64_mult_dyn;
pub mod choice_text;
pub mod choice_text_dyn;
pub mod choice_text_mult;
pub mod choice_text_mult_dyn;
pub mod choice_u32;
pub mod choice_u32_dyn;
pub mod choice_u32_mult;
pub mod choice_u32_mult_dyn;

pub use {
    crate::fields::bool::Bool, crate::fields::image::Image, crate::fields::slug::Slug,
    choice_f64::ChoiceF64, choice_f64_dyn::ChoiceF64Dyn, choice_f64_mult::ChoiceF64Mult,
    choice_f64_mult_dyn::ChoiceF64MultDyn, choice_i32::ChoiceI32, choice_i32_dyn::ChoiceI32Dyn,
    choice_i32_mult::ChoiceI32Mult, choice_i32_mult_dyn::ChoiceI32MultDyn, choice_i64::ChoiceI64,
    choice_i64_dyn::ChoiceI64Dyn, choice_i64_mult::ChoiceI64Mult,
    choice_i64_mult_dyn::ChoiceI64MultDyn, choice_text::ChoiceText, choice_text_dyn::ChoiceTextDyn,
    choice_text_mult::ChoiceTextMult, choice_text_mult_dyn::ChoiceTextMultDyn,
    choice_u32::ChoiceU32, choice_u32_dyn::ChoiceU32Dyn, choice_u32_mult::ChoiceU32Mult,
    choice_u32_mult_dyn::ChoiceU32MultDyn, color::Color, date::Date, date_time::DateTime,
    email::Email, file::File, hash::Hash, hidden_date_time::HiddenDateTime, ip::IP,
    number_f64::F64, number_i32::I32, number_i64::I64, number_u32::U32, password::Password,
    phone::Phone, text::Text, url::URL,
};
