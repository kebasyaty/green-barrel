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
    widgets::{generate_html::GenerateHtml, Widget},
};
