//! # Green Barrel
//! **ORM-like API MongoDB for Rust.**
//!
//! ## Usage:
//!
//! [Basic Example](https://github.com/kebasyaty/green-barrel/tree/master/examples/basic "Basic Example")
//!
//! ## Model parameters
//!
//! **_( all parameters are optional )_**
//!
//! | Parameter:          | Default:     | Description:                                                                                         |
//! | :------------------ | :----------- | :--------------------------------------------------------------------------------------------------- |
//! | db_query_docs_limit | 1000         | limiting query results.                                                                              |
//! | is_add_doc          | true         | Create documents in the database. **false** - Alternatively, use it to validate data from web forms. |
//! | is_up_doc           | true         | Update documents in the database.                                                                    |
//! | is_del_doc          | true         | Delete documents from the database.                                                                  |
//! | ignore_fields       | empty string | Fields that are not included in the database (separated by commas).                                  |
//! | is_use_add_valid    | false        | Allows additional validation - **impl AdditionalValidation for ModelName**.                          |
//! | is_use_hooks        | false        | Allows hooks methods - **impl Hooks for ModelName**.                                                 |
//!

#[macro_use]
extern crate rust_i18n;

i18n!("locales");

pub mod fields;
pub mod migration;
pub mod models;
pub mod store;
pub mod test_tool;

pub use crate::{
    fields::*,
    migration::Monitor,
    models::{
        caching::Caching,
        control::Control,
        converters::Converters,
        db_query_api::{commons::QCommons, paladins::QPaladins},
        fixtures::Fixtures,
        helpers::{ControlArr, FileData, ImageData, Meta},
        hooks::Hooks,
        output_data::{OutputData, OutputData2},
        validation::{AdditionalValidation, Validation},
        Main,
    },
    store::METADATA,
    test_tool::del_test_db,
};
