//! Validating Model fields for save and update.

use mongodb::{
    bson::{doc, oid::ObjectId, Bson},
    sync::Collection,
};
use regex::Regex;
use serde_json::value::Value;
use std::{collections::HashMap, error::Error};

use crate::store::{REGEX_IS_COLOR_CODE, REGEX_IS_DATE, REGEX_IS_DATETIME, REGEX_IS_PASSWORD};

/// Validating Model fields for save and update.
// *************************************************************************************************
pub trait Validation {
    /// Validation of `minlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(format!("Exceeds limit, minlength={}.", minlength))?
        }
        Ok(())
    }

    /// Validation of `maxlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(format!("Exceeds limit, maxlength={}.", maxlength))?
        }
        Ok(())
    }

    /// Accumulation of errors.
    // ---------------------------------------------------------------------------------------------
    fn accumula_err(field: &Value, err: &str) -> Result<String, Box<dyn Error>> {
        let mut tmp = field.get("error").unwrap().as_str().unwrap().to_string();
        tmp = if !tmp.is_empty() {
            format!("{}<br>", tmp)
        } else {
            String::new()
        };
        Ok(format!("{}{}", tmp, err))
    }

    /// Validation in regular expression (email, password, etc...).
    // ---------------------------------------------------------------------------------------------
    fn regex_validation(field_type: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match field_type {
            "InputEmail" => {
                if !validator::validate_email(value) {
                    Err("Invalid email address.")?
                }
            }
            "InputColor" => {
                if !REGEX_IS_COLOR_CODE.is_match(value) {
                    Err("Invalid Color code.")?
                }
            }
            "InputUrl" => {
                if !validator::validate_url(value) {
                    Err("Invalid Url.")?
                }
            }
            "InputIP" => {
                if !validator::validate_ip(value) {
                    Err("Invalid IP address.")?
                }
            }
            "InputIPv4" => {
                if !validator::validate_ip_v4(value) {
                    Err("Invalid IPv4 address.")?
                }
            }
            "InputIPv6" => {
                if !validator::validate_ip_v6(value) {
                    Err("Invalid IPv6 address.")?
                }
            }
            "InputPassword" => {
                if !REGEX_IS_PASSWORD.is_match(value) {
                    Err("Size 8-256 chars.<br>\
                        Allowed chars: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (")?
                }
            }
            "InputDate" => {
                if !REGEX_IS_DATE.is_match(value) {
                    Err("Incorrect date format.<br>\
                         Example: 1970-02-28")?
                }
            }
            "InputDateTime" => {
                if !REGEX_IS_DATETIME.is_match(value) {
                    Err("Incorrect date and time format.<br>\
                         Example: 1970-02-28T00:00")?
                }
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    /// Validation of `unique`.
    // ---------------------------------------------------------------------------------------------
    fn check_unique(
        hash: &str,
        field_name: &str,
        bson_field_value: &Bson,
        coll: &Collection,
    ) -> Result<(), Box<dyn Error>> {
        //
        let object_id = ObjectId::with_string(hash);
        let mut filter = doc! { field_name: bson_field_value };
        if let Ok(id) = object_id {
            // If the document is will updated.
            filter = doc! {
                "$and": [
                    { "_id": { "$ne": id } },
                    filter
                ]
            };
        }
        let count: i64 = coll.count_documents(filter, None)?;
        if count > 0 {
            Err("Is not unique.")?
        }
        Ok(())
    }

    /// Field attribute check - pattern.
    // ----------------------------------------------------------------------------------------------
    fn regex_pattern_validation(
        field_value: &str,
        regex_pattern: &str,
    ) -> Result<(), Box<dyn Error>> {
        let pattern = Regex::new(regex_pattern)?;
        if !pattern.is_match(field_value) {
            Err("Does not match the pattern attribute.")?
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
/// #[Model(
///     is_use_add_valid = true,
/// )]
/// #[derive(Serialize, Deserialize, Default, Debug)]
/// pub struct ModelName {
///     Add your fields ...
/// }
///
/// impl AdditionalValidation for ModelName {
///     fn add_validation<'a>(
///         &self,
///     ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
///         // Hint: error_map.insert("field_name", "Error message.")
///         let mut error_map: std::collections::HashMap<&'a str, &'a str> =
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
///             error_map.insert("confirm_password", "Password confirmation does not match.");
///         }
///         if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
///             .case_insensitive(true)
///             .build()
///             .unwrap()
///             .is_match(username.as_str())
///         {
///             error_map.insert(
///                 "username",
///                 "Invalid characters present.<br>\
///                  Valid characters: a-z A-Z 0-9 _ @ + .",
///             );
///         }
///
///         Ok(error_map)
///     }
/// }
/// ```
///
pub trait AdditionalValidation {
    // Default implementation as a stub.
    fn add_validation<'a>(&self) -> Result<HashMap<&'a str, &'a str>, Box<dyn Error>> {
        // error_map.insert("field_name", "Error message.")
        let error_map: HashMap<&'a str, &'a str> = HashMap::new();
        Ok(error_map)
    }
}
