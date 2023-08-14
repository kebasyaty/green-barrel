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
    crate::fields::bool::BoolField, crate::fields::image::ImageField,
    crate::fields::slug::SlugField, choice_f64::ChoiceF64Field, choice_f64_dyn::ChoiceF64DynField,
    choice_f64_mult::ChoiceF64MultField, choice_f64_mult_dyn::ChoiceF64MultDynField,
    choice_i32::ChoiceI32Field, choice_i32_dyn::ChoiceI32DynField,
    choice_i32_mult::ChoiceI32MultField, choice_i32_mult_dyn::ChoiceI32MultDynField,
    choice_i64::ChoiceI64Field, choice_i64_dyn::ChoiceI64DynField,
    choice_i64_mult::ChoiceI64MultField, choice_i64_mult_dyn::ChoiceI64MultDynField,
    choice_text::ChoiceTextField, choice_text_dyn::ChoiceTextDynField,
    choice_text_mult::ChoiceTextMultField, choice_text_mult_dyn::ChoiceTextMultDynField,
    choice_u32::ChoiceU32Field, choice_u32_dyn::ChoiceU32DynField,
    choice_u32_mult::ChoiceU32MultField, choice_u32_mult_dyn::ChoiceU32MultDynField,
    color::ColorField, date::DateField, date_time::DateTimeField, email::EmailField,
    file::FileField, hash::HashField, hidden_date_time::HiddenDateTimeField, ip::IPField,
    number_f64::F64Field, number_i32::I32Field, number_i64::I64Field, number_u32::U32Field,
    password::PasswordField, phone::PhoneField, text::TextField, url::URLField,
};
