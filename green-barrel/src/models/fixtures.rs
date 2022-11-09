//! To populate the database with pre-created data.

use std::error::Error;
use std::path::Path;

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
    fn run_fixture(fixture_name: &str, _unique_field: &str) -> Result<(), Box<dyn Error>> {
        // Create path
        let file_path = format!("./fixtures/{fixture_name}.json");
        let fixture_path = Path::new(&file_path);
        // Validation of file.
        if !fixture_path.is_file() {
            Err(format!(
                "Model: `{model_name} ; Method: \
                    run_fixture()` => File is missing - {file_path}"
            ))?
        }

        Ok(())
    }
}
