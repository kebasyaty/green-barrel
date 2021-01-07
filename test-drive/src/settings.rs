//! # Global application settings.
//!

// KEYWORD it is recommended not to change.
// Valid characters: _ a-z A-Z 0-9
// Size: 6-52
// Example: "PROJECT_NAME_7rzg_cfqQB3B7q7T"
pub static KEYWORD: &str = "PROJECT_NAME_7rzg_cfqQB3B7q7T";

// For a test application.
pub mod app_name {
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}
