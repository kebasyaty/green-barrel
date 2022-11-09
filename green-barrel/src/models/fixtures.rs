//! To populate the database with pre-created data.

use std::error::Error;

/// To populate the database with pre-created data.
/// fixture_name - Name of the fixture file in the ./fixtures directory, no extension (.json).
/// unique_field - The name of any unique field in the Model.
///
/// # Example:
///
/// ```
/// ```
///
pub trait Fixtures {
    fn run_fixture(_fixture_name: &str, _unique_field: &str) -> Result<(), Box<dyn Error>> {
        //
        Ok(())
    }
}
