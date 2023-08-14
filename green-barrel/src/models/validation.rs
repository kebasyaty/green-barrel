//! Helper methods to validate data before saving or updating to the database.

use async_trait::async_trait;
use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    Collection,
};
use regex::{Regex, RegexBuilder};
use serde_json::value::Value;
use std::error::Error;

/// Helper methods to validate data before saving or updating to the database.
// *************************************************************************************************
#[async_trait(?Send)]
pub trait Validation {
    /// Validation of `minlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(t!("min_chars", count = minlength))?
        }
        Ok(())
    }

    /// Validation of `maxlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(t!("max_chars", count = maxlength))?
        }
        Ok(())
    }

    /// Accumulation of errors.
    // ---------------------------------------------------------------------------------------------
    fn accumula_err(field: &mut Value, err: &str) {
        let err_vec = field["errors"].as_array_mut().unwrap();
        let err = serde_json::to_value(err).unwrap();
        if !err_vec.contains(&err) {
            err_vec.push(err);
        }
    }

    /// Validation Email, Url, IP, IPv4, IPv6, Color.
    // ---------------------------------------------------------------------------------------------
    fn validation(field_type: &str, value: &str) -> Result<(), Box<dyn Error>> {
        match field_type {
            "EmailField" => {
                if !validator::validate_email(value) {
                    Err(t!("invalid_email"))?
                }
            }
            "URLField" => {
                if !validator::validate_url(value) {
                    Err(t!("invalid_url"))?
                }
            }
            "IPField" => {
                if !validator::validate_ip(value) {
                    Err(t!("invalid_ip"))?
                }
            }
            "IPv4" => {
                if !validator::validate_ip_v4(value) {
                    Err(t!("invalid_ipv4"))?
                }
            }
            "IPv6" => {
                if !validator::validate_ip_v6(value) {
                    Err(t!("invalid_ipv6"))?
                }
            }
            "ColorField" => {
                if !(RegexBuilder::new(
                    r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6}|[a-f0-9]{8})\b|(?:rgb|hsl)a?\([^\)]*\)$",
                )
                .case_insensitive(true)
                .build()
                .unwrap()
                .is_match(value))
                {
                    Err(t!("invalid_color"))?
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
            Err(t!("not_unique"))?
        }
        Ok(())
    }

    /// Validation field attribute `regex`.
    // ----------------------------------------------------------------------------------------------
    fn regex_validation(field_value: &str, regex_str: &str) -> Result<(), Box<dyn Error>> {
        let pattern = Regex::new(regex_str)?;
        if !field_value.is_empty() && !pattern.is_match(field_value) {
            Err("")?
        }
        Ok(())
    }
}
