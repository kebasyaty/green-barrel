//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.
//!

pub mod migration;
pub mod models;
pub mod store;
pub mod test_tool;
pub mod widgets;

pub use crate::{
    migration::Monitor,
    models::{
        caching::CachingModel,
        converters::Converters,
        db_query_api::{commons::QCommons, paladins::QPaladins},
        hooks::Hooks,
        validation::{AdditionalValidation, ValidationModel},
        Meta, ToModel,
    },
    store::{FormCache, FORM_STORE, MONGODB_CLIENT_STORE},
    widgets::{html_controls::HtmlControls, output_data::OutputDataForm, TransMapWidgets, Widget},
};

pub use crate::test_tool::del_test_db;
