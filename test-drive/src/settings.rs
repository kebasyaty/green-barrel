//! # Global service/sub-application settings.
//!

// Project name.
// Valid characters: _ a-z A-Z 0-9
// Max size: 22
pub const PROJECT_NAME: &str = "project_name";

// Unique project key.
// Hint: UNIQUE_PROJECT_KEY it is recommended not to change.
// Valid characters: a-z A-Z 0-9
// Size: 8-16
// Example: "7rzgacfqQB3B7q7T"
pub const UNIQUE_PROJECT_KEY: &str = "7rzgacfqQB3B7q7T";

// For a test service/sub-application.
pub mod service_name {
    pub const SERVICE_NAME: &str = "service_name"; // Max size: 31 characters
    pub const DATABASE_NAME: &str = "database_name"; // Max size: 22 characters
    pub const DB_CLIENT_NAME: &str = "default";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}
