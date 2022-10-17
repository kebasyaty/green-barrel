//! Output data for QPaladins.

use mongodb::bson::{document::Document, oid::ObjectId};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::error::Error;

use crate::models::converters::Converters;

/// Output data for delete(), update_password(), delete_many(), delete_one, drop() methods.
// =================================================================================================
#[derive(Debug)]
pub enum OutputData {
    Delete(
        (
            bool,   // result_bool
            String, // err_msg
            i64,    // deleted_count
        ),
    ),
    UpdatePassword(
        (
            bool,   // result_bool
            String, // err_msg
        ),
    ),
}

impl OutputData {
    /// Get validation status (boolean).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.delete()?;
    /// // or
    /// let output_data = model_name.update_password()?;
    ///
    /// assert!(output_data.is_valid());
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Delete(data) => data.0,
            Self::UpdatePassword(data) => data.0,
        }
    }

    /// Description of the error if the document was not deleted or the password was not updated.
    // ---------------------------------------------------------------------------------------------
    /// (Main use for admin panel.)
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.delete()?;
    /// // or
    /// let output_data = model_name.update_password()?;
    ///
    /// if output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    pub fn err_msg(&self) -> String {
        match self {
            Self::Delete(data) => data.1.clone(),
            Self::UpdatePassword(data) => data.1.clone(),
        }
    }

    /// Printing errors to the console ( for development ).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.delete()?;
    /// // or
    /// let output_data = model_name.update_password()?;
    ///
    /// if !output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    pub fn print_err(&self) {
        let errors = self.err_msg();
        if !errors.is_empty() {
            println!("\nERRORS:{}\n", errors);
        }
    }

    /// Get deleted count.
    // ---------------------------------------------------------------------------------------------
    pub fn deleted_count(&self) -> Result<i64, Box<dyn Error>> {
        match self {
            Self::Delete(data) => Ok(data.2),
            _ => Err("Invalid output type.")?,
        }
    }
}

/// Output data for check() and save() methods.
// =================================================================================================
#[derive(Debug)]
pub struct OutputData2 {
    pub is_valid: bool,
    pub final_doc: Option<Document>,
    pub final_model_json: Value,
    pub fields_name: Vec<String>,
}

impl Converters for OutputData2 {}

impl OutputData2 {
    /// Get validation status (boolean).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// assert!(output_data.is_valid());
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// If there are AutoSlug fields, do an update. Use only for save() method.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.save(None, None)?;
    ///
    /// if output_data.is_valid() {
    ///     model_name = output_data.update()?;
    /// }
    /// ```
    ///
    pub fn update<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned + Sized,
    {
        serde_json::from_value::<T>(self.final_model_json.clone())
    }

    /// Get/Set Hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check(None)?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.hash());
    /// println!("{}", output_data.set_hash(hash_line));
    /// ```
    ///
    pub fn hash(&self) -> String {
        let value = self
            .final_model_json
            .get("hash")
            .unwrap()
            .get("value")
            .unwrap();
        if value.is_null() {
            return String::new();
        }
        value.as_str().unwrap().to_string()
    }
    pub fn set_hash(&mut self, hash: String) {
        *self
            .final_model_json
            .get_mut("hash")
            .unwrap()
            .get_mut("value")
            .unwrap() = json!(hash);
    }

    /// Get MongoDB ID from hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{:?}", output_data.obj_id()?);
    /// ```
    ///
    pub fn obj_id(&self) -> Result<Option<ObjectId>, Box<dyn Error>> {
        let hash = self.hash();
        if let Ok(obj_id) = ObjectId::with_string(hash.as_str()) {
            return Ok(Some(obj_id));
        }
        Ok(None)
    }

    /// Get Model instance in Json-line format.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.json()?);
    /// ```
    ///
    pub fn json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(&self.final_model_json).unwrap())
    }

    /// Get the creation date of the document.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.created_at());
    /// ```
    ///
    pub fn created_at(&self) -> Option<&str> {
        self.final_model_json
            .get("created_at")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
    }

    /// Get the date the document was updated.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.updated_at());
    /// ```
    ///
    pub fn updated_at(&self) -> Option<&str> {
        self.final_model_json
            .get("updated_at")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
    }

    /// Get errors message ( for user side ).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// if output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    pub fn err_msg(&self) -> String {
        let mut errors = String::new();
        for field_name in self.fields_name.iter() {
            let tmp = errors.clone();
            let field_type = self.final_model_json.get(field_name).unwrap();
            let error = field_type.get("error").unwrap().as_str().unwrap();
            if !error.is_empty() {
                errors = format!("{}\nField: `{}` => {}", tmp, field_name, error);
            }
        }
        if !errors.is_empty() {
            errors = errors.replace("<br>", " | ");
        }
        errors
    }

    /// Printing errors to the console ( for development ).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// if output_data.is_valid() {
    ///     output_data.print_err();
    /// }
    /// ```
    ///
    pub fn print_err(&self) {
        let errors = self.err_msg();
        if !errors.is_empty() {
            println!("\nERRORS:{}\n", errors);
        }
    }

    // Methods for internal needs.
    // *********************************************************************************************
    /// Get field type list in json-line format for admin panel.
    // ---------------------------------------------------------------------------------------------
    /// ( converts a field type map to a list, in the order of the Model fields )
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.json_for_admin()?);
    /// ```
    ///
    pub fn json_for_admin(&self) -> Result<Option<Value>, Box<dyn Error>> {
        let mut field_list: Vec<Value> = Vec::new();
        let hash: &str = self
            .final_model_json
            .get("hash")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
            .unwrap_or_default();
        // Get a list of fields type in the order of the model fields.
        for field_name in self.fields_name.iter() {
            let mut field = self.final_model_json.get(field_name).unwrap().clone();
            if field_name == "created_at" || field_name == "updated_at" {
                *field.get_mut("input_type").unwrap() = json!("datetime");
                *field.get_mut("is_hide").unwrap() = json!(false);
            }
            if field_name.contains("password") && !hash.is_empty() {
                *field.get_mut("input_type").unwrap() = json!("hidden");
                *field.get_mut("is_hide").unwrap() = json!(true);
                *field.get_mut("value").unwrap() = json!("");
            }
            field_list.push(field);
        }
        //
        Ok(Some(json!(field_list)))
    }

    /// Get/Set final document
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{:?}", output_data.get_doc());
    /// println!("{:?}", output_data.set_doc(Some(new_doc)));
    /// ```
    ///
    pub fn get_doc(&self) -> Option<Document> {
        self.final_doc.clone()
    }
    pub fn set_doc(&mut self, new_doc: Option<Document>) {
        self.final_doc = new_doc;
    }

    /// Get Model instance in serde_json::Value format.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let mut model_name = ModelName::new()?
    /// ;
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{:?}", output_data.model_json());
    /// ```
    ///
    pub fn model_json(&self) -> Value {
        self.final_model_json.clone()
    }
}
