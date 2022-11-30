//! General settings for Models.

// Project name.
// Valid characters: _ a-z A-Z 0-9
// Hint: PROJECT_NAM it is recommended not to change.
// Max size: 20
// First character: a-z A-Z
pub const PROJECT_NAME: &str = "project_name";

// Unique project key.
// Hint: UNIQUE_PROJECT_KEY it is recommended not to change.
// Valid characters: a-z A-Z 0-9
// Size: 16
// Example: "7rzgacfqQB3B7q7T"
// To generate a key (This is not an advertisement): https://randompasswordgen.com/
pub const UNIQUE_PROJECT_KEY: &str = "q25347r611K6pJ03";

// Default settings.
pub mod default {
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 30
    // First character: a-z A-Z
    pub const SERVICE_NAME: &str = "service_name";
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 20
    // First character: a-z A-Z
    pub const DATABASE_NAME: &str = "service_name";
    //
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}
