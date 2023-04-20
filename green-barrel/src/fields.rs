//! For control of fields on the server and client side.

// bool
pub mod check_box;
// hidden
pub mod hash;
pub mod hidden_date_time;
// text
pub mod color;
pub mod date;
pub mod date_time;
pub mod image;
pub mod input_password;
pub mod input_phone;
pub mod input_text;
pub mod input_url;
pub mod ip;
pub mod slug;
pub mod text_area;
// file
pub mod email;
pub mod file;
// number
pub mod number_f64;
pub mod number_i32;
pub mod number_i64;
pub mod number_u32;
pub mod radio_f64;
pub mod radio_i32;
pub mod radio_i64;
pub mod radio_text;
pub mod radio_u32;
pub mod range_f64;
pub mod range_i32;
pub mod range_i64;
pub mod range_u32;
// select
pub mod select_f64;
pub mod select_f64_dyn;
pub mod select_f64_mult;
pub mod select_f64_mult_dyn;
pub mod select_i32;
pub mod select_i32_dyn;
pub mod select_i32_mult;
pub mod select_i32_mult_dyn;
pub mod select_i64;
pub mod select_i64_dyn;
pub mod select_i64_mult;
pub mod select_i64_mult_dyn;
pub mod select_text;
pub mod select_text_dyn;
pub mod select_text_mult;
pub mod select_text_mult_dyn;
pub mod select_u32;
pub mod select_u32_dyn;
pub mod select_u32_mult;
pub mod select_u32_mult_dyn;

pub use {
    crate::fields::image::Image, crate::fields::slug::Slug, check_box::CheckBox, color::Color,
    date::Date, date_time::DateTime, email::Email, file::File, hash::Hash,
    hidden_date_time::HiddenDateTime, input_password::InputPassword, input_phone::InputPhone,
    input_text::InputText, input_url::InputUrl, ip::IP, number_f64::F64, number_i32::I32,
    number_i64::I64, number_u32::U32, radio_f64::RadioF64, radio_i32::RadioI32,
    radio_i64::RadioI64, radio_text::RadioText, radio_u32::RadioU32, range_f64::RangeF64,
    range_i32::RangeI32, range_i64::RangeI64, range_u32::RangeU32, select_f64::SelectF64,
    select_f64_dyn::SelectF64Dyn, select_f64_mult::SelectF64Mult,
    select_f64_mult_dyn::SelectF64MultDyn, select_i32::SelectI32, select_i32_dyn::SelectI32Dyn,
    select_i32_mult::SelectI32Mult, select_i32_mult_dyn::SelectI32MultDyn, select_i64::SelectI64,
    select_i64_dyn::SelectI64Dyn, select_i64_mult::SelectI64Mult,
    select_i64_mult_dyn::SelectI64MultDyn, select_text::SelectText, select_text_dyn::SelectTextDyn,
    select_text_mult::SelectTextMult, select_text_mult_dyn::SelectTextMultDyn,
    select_u32::SelectU32, select_u32_dyn::SelectU32Dyn, select_u32_mult::SelectU32Mult,
    select_u32_mult_dyn::SelectU32MultDyn, text_area::TextArea,
};
