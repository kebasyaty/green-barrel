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
        caching::Caching,
        converters::Converters,
        db_query_api::{
            administrator::{Administrator, OutputDataAdmin},
            commons::QCommons,
            paladins::QPaladins,
        },
        hooks::Hooks,
        validation::{AdditionalValidation, Validation},
        Main, Meta,
    },
    store::{FormCache, FORM_STORE, MONGODB_CLIENT_STORE},
    test_tool::del_test_db,
    widgets::{
        generate_html::GenerateHtml, output_data::OutputData, Enctype, HttpMethod, TransMapWidgets,
        Widget,
    },
};
