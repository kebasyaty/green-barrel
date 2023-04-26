//! Helper methods to validate data before saving or updating to the database.

use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    Client, Collection,
};
use regex::Regex;
use serde_json::value::Value;
use std::{collections::HashMap, error::Error};

/// Helper methods to validate data before saving or updating to the database.
// *************************************************************************************************
#[async_trait(?Send)]
pub trait Validation {
    /// Validation of `minlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(format!("Minimum {} characters.", minlength))?
        }
        Ok(())
    }

    /// Validation of `maxlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(format!("Maximum {} characters.", maxlength))?
        }
        Ok(())
    }

    /// Accumulation of errors.
    // ---------------------------------------------------------------------------------------------
    fn accumula_err(field: &Value, err: &str) -> String {
        let mut msg = field.get("error").unwrap().as_str().unwrap().to_string();
        if !msg.contains(err) {
            if !msg.is_empty() {
                msg = format!("{}<\\br>{}", msg, err)
            } else {
                msg = err.to_string()
            }
        }
        msg
    }

    /// Validation in regular expression (email, password, etc...).
    // ---------------------------------------------------------------------------------------------
    fn regex_validation(field_type: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match field_type {
            "Email" => {
                if !validator::validate_email(value) {
                    Err("Invalid email address.")?
                }
            }
            "Url" => {
                if !validator::validate_url(value) {
                    Err("Invalid Url.")?
                }
            }
            "IP" => {
                if !validator::validate_ip(value) {
                    Err("Invalid IP address.")?
                }
            }
            "IPv4" => {
                if !validator::validate_ip_v4(value) {
                    Err("Invalid IPv4 address.")?
                }
            }
            "IPv6" => {
                if !validator::validate_ip_v6(value) {
                    Err("Invalid IPv6 address.")?
                }
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    /// Validation of `unique`.
    // ---------------------------------------------------------------------------------------------
    async fn check_unique(
        hash: &str,
        field_name: &str,
        field_value_bson: &Bson,
        coll: &Collection<Document>,
    ) -> Result<(), Box<dyn Error>> {
        //
        let object_id = ObjectId::parse_str(hash);
        let mut filter = doc! { field_name: field_value_bson };
        if let Ok(id) = object_id {
            // If the document is will updated.
            filter = doc! {
                "$and": [
                    { "_id": { "$ne": id } },
                    filter
                ]
            };
        }
        let count = coll.count_documents(filter, None).await?;
        if count > 0 {
            Err("Is not unique.")?
        }
        Ok(())
    }

    /// Field attribute check - pattern.
    // ----------------------------------------------------------------------------------------------
    fn regex_pattern_validation(
        field_value: &str,
        regex_str: &str,
    ) -> Result<(), Box<dyn Error>> {
        let pattern = Regex::new(regex_str)?;
        if !field_value.is_empty() && !pattern.is_match(field_value) {
            Err("")?
        }
        Ok(())
    }
}

/// Methods for additional validation.
/// Hint: For custom use, add the Model/Form attribute `is_use_add_valid = true`.
/// Hint (for models): Remember to use for validate of ignored fields.
// *************************************************************************************************
///
/// # Example:
///
/// ```
/// use async_trait::async_trait;
///
/// #[Model(
///     is_use_add_valid = true,
/// )]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     Your fields ...
/// }
///
/// #[async_trait(?Send)]
/// impl AdditionalValidation for ModelName {
///     async fn add_validation(
///         &self,
///     ) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
///         // Hint: error_map.insert("field_name", "Error message.")
///         let mut error_map: std::collections::HashMap<String, String> =
///             std::collections::HashMap::new();
///
///         // Get clean data
///         let hash = self.hash.clone().unwrap_or_default();
///         let password = self.password.clone().unwrap_or_default();
///         let confirm_password = self.confirm_password.clone().unwrap_or_default();
///         let username = self.username.clone().unwrap_or_default();
///
///         // Fields validation
///         if hash.is_empty() && password != confirm_password {
///             error_map.insert("confirm_password".into(), "Password confirmation does not match.".into());
///         }
///         if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
///             .case_insensitive(true)
///             .build()
///             .unwrap()
///             .is_match(username.as_str())
///         {
///             error_map.insert(
///                 "username".into(),
///                 "Invalid characters present.<br>\
///                  Valid characters: a-z A-Z 0-9 _ @ + .".into(),
///             );
///         }
///
///         Ok(error_map)
///     }
/// }
/// ```
///
#[async_trait(?Send)]
pub trait AdditionalValidation {
    // Default implementation as a stub.
    async fn add_validation(
        &self,
        _client: &Client,
    ) -> Result<HashMap<String, String>, Box<dyn Error>> {
        // error_map.insert("field_name", "Error message.")
        let error_map = HashMap::<String, String>::new();
        Ok(error_map)
    }
}
