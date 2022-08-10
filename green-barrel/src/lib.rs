//! # Green Barrel
//! ORM-like API MongoDB for Rust.

pub mod helpers;
pub mod migration;
pub mod models;
pub mod store;
pub mod test_tool;
pub mod widgets;

pub use crate::{
    helpers::{
        administrator::{Administrator, OutputDataAdmin},
        enumerations::{ControlArr, Enctype, HttpMethod},
        generate_html::GenerateHtml,
        structures::{FileData, ImageData, Meta, TransMapWidgets},
    },
    migration::Monitor,
    models::{
        caching::Caching,
        converters::Converters,
        db_query_api::{commons::QCommons, paladins::QPaladins},
        hooks::Hooks,
        output_data::{OutputData, OutputDataCheck},
        validation::{AdditionalValidation, Validation},
        Main,
    },
    store::{ModelCache, MODEL_STORE, MONGODB_CLIENT_STORE},
    test_tool::del_test_db,
    widgets::{
        check_box::CheckBox, input_color::InputColor, input_date::InputDate,
        input_date_time::InputDateTime, input_email::InputEmail, input_file::InputFile,
        input_image::InputImage, input_ip::InputIP, input_ipv4::InputIPv4, input_ipv6::InputIPv6,
        input_password::InputPassword, input_phone::InputPhone, input_slug::InputSlug,
        input_text::InputText, input_url::InputUrl, number_f64::NumberF64, number_i32::NumberI32,
        number_i64::NumberI64, number_u32::NumberU32, radio_f64::RadioF64, radio_i32::RadioI32,
        radio_i64::RadioI64, radio_text::RadioText, radio_u32::RadioU32, range_f64::RangeF64,
        range_i32::RangeI32, range_i64::RangeI64, range_u32::RangeU32, select_f64::SelectF64,
        select_f64_dyn::SelectF64Dyn, select_i32::SelectI32, select_i32_dyn::SelectI32Dyn,
        select_i32_mult::SelectI32Mult, select_i32_mult_dyn::SelectI32MultDyn,
        select_i64::SelectI64, select_i64_dyn::SelectI64Dyn, select_i64_mult::SelectI64Mult,
        select_i64_mult_dyn::SelectI64MultDyn, select_text::SelectText,
        select_text_dyn::SelectTextDyn, select_text_mult::SelectTextMult,
        select_text_mult_dyn::SelectTextMultDyn, select_u32::SelectU32,
        select_u32_dyn::SelectU32Dyn, select_u32_mult::SelectU32Mult,
        select_u32_mult_dyn::SelectU32MultDyn, text_area::TextArea, Widget,
    },
};
