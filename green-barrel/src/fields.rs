//! For control of fields on the server and client side.

// bool
pub mod check_box;
// hidden
pub mod hidden_date_time;
pub mod hidden_hash;
// text
pub mod auto_slug;
pub mod input_color;
pub mod input_date;
pub mod input_date_time;
pub mod input_image;
pub mod input_ip;
pub mod input_ipv4;
pub mod input_ipv6;
pub mod input_password;
pub mod input_phone;
pub mod input_text;
pub mod input_url;
pub mod text_area;
// file
pub mod input_email;
pub mod input_file;
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
    auto_slug::AutoSlug, check_box::CheckBox, hidden_date_time::HiddenDateTime,
    hidden_hash::HiddenHash, input_color::InputColor, input_date::InputDate,
    input_date_time::InputDateTime, input_email::InputEmail, input_file::InputFile,
    input_image::InputImage, input_ip::InputIP, input_ipv4::InputIPv4, input_ipv6::InputIPv6,
    input_password::InputPassword, input_phone::InputPhone, input_text::InputText,
    input_url::InputUrl, number_f64::NumberF64, number_i32::NumberI32, number_i64::NumberI64,
    number_u32::NumberU32, radio_f64::RadioF64, radio_i32::RadioI32, radio_i64::RadioI64,
    radio_text::RadioText, radio_u32::RadioU32, range_f64::RangeF64, range_i32::RangeI32,
    range_i64::RangeI64, range_u32::RangeU32, select_f64::SelectF64, select_f64_dyn::SelectF64Dyn,
    select_f64_mult::SelectF64Mult, select_f64_mult_dyn::SelectF64MultDyn, select_i32::SelectI32,
    select_i32_dyn::SelectI32Dyn, select_i32_mult::SelectI32Mult,
    select_i32_mult_dyn::SelectI32MultDyn, select_i64::SelectI64, select_i64_dyn::SelectI64Dyn,
    select_i64_mult::SelectI64Mult, select_i64_mult_dyn::SelectI64MultDyn, select_text::SelectText,
    select_text_dyn::SelectTextDyn, select_text_mult::SelectTextMult,
    select_text_mult_dyn::SelectTextMultDyn, select_u32::SelectU32, select_u32_dyn::SelectU32Dyn,
    select_u32_mult::SelectU32Mult, select_u32_mult_dyn::SelectU32MultDyn, text_area::TextArea,
};