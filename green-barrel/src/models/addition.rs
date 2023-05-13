//! Addition - Methods for additional actions and additional validation.

use async_trait::async_trait;
use mongodb::{bson::doc, Client};
use std::{collections::HashMap, error::Error};

/// Methods for additional actions and additional validation.
/// Hint: Add the Model parameter is_use_addition = true.
// *************************************************************************************************
///
/// # Example:
///
/// ```
/// use async_trait::async_trait;
/// use mongodb::{bson::doc, Client};
/// use std::{collections::HashMap, error::Error};
///
/// #[Model(
///     username: Text,
///     is_use_addition = true,
///     ignore_fields = "confirm_password"
/// )]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     password: Password,
///     confirm_password: Password,
/// }
///
/// #[async_trait(?Send)]
/// impl Addition for ModelName {
///     async fn add_actions(
///         &self,
///         _client: &Client,
///     ) -> Result<HashMap<String, String>, Box<dyn Error>> {
///         // Get clean data.
///         let username = self.username.get().unwrap_or_default();
///         self.username.set(username.to_uppercase());
///         Ok(())
///     }
///
///     async fn add_validation(
///         &self,
///         _client: &Client,
///     ) -> Result<HashMap<String, String>, Box<dyn Error>> {
///         // Hint: error_map.insert("field_name", "Error message.")
///         let mut error_map = HashMap::<String, String>::new();
///
///         // Get clean data.
///         let password = self.password.get().unwrap_or_default();
///         let confirm_password = self.confirm_password.get().unwrap_or_default();
///
///         // Fields validation.
///         if (!password.is_empty() && !confirm_password.is_empty()) && password != confirm_password {
///             error_map.insert("confirm_password".into(), t!("password_mismatch"));
///         }
///
///         Ok(error_map)
///     }
/// }
/// ```
///
#[async_trait(?Send)]
pub trait Addition {
    /// It is intended for additional actions with fields.
    /// Hint: This method is execute first.
    async fn add_actions(&mut self, _client: &Client) -> Result<(), Box<dyn Error>> {
        // your code...
        Ok(())
    }
    /// It is supposed to be use to additional validation of fields.
    /// Hint: This method is execute second.
    async fn add_validation(
        &self,
        _client: &Client,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        // your code...
        Ok(HashMap::new())
    }
}
