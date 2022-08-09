//! # Green Barrel
//! ORM-like API MongoDB for Rust.

pub mod administrator;
pub mod helpers;
pub mod migration;
pub mod models;
pub mod store;
pub mod test_tool;
pub mod widgets;

pub use crate::{
    administrator::{Administrator, OutputDataAdmin},
    helpers::{ControlArr, Enctype, FileData, HttpMethod, ImageData, Meta, TransMapWidgets},
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
        check_box::CheckBox, generate_html::GenerateHtml, input_color::InputColor,
        input_date::InputDate, input_date_time::InputDateTime, input_file::InputFile,
        input_image::InputImage, input_ip::InputIP, input_ipv4::InputIPv4, input_ipv6::InputIPv6,
        input_mail::InputEmail, input_password::InputPassword, input_phone::InputPhone,
        input_slug::InputSlug, input_text::InputText, input_url::InputUrl, number_f64::NumberF64,
        number_i32::NumberI32, number_i64::NumberI64, number_u32::NumberU32, radio_f64::RadioF64,
        radio_i32::RadioI32, radio_i64::RadioI64, radio_text::RadioText, radio_u32::RadioU32,
        range_f64::RangeF64, range_i32::RangeI32, range_i64::RangeI64, range_u32::RangeU32,
        select_text::SelectText, select_text_dyn::SelectTextDyn, select_text_mult::SelectTextMult,
        text_area::TextArea, Widget,
    },
};
