//! Output data for QPaladins.

use mongodb::bson::{document::Document, oid::ObjectId};
use serde_json::{json, Value};
use std::error::Error;

use crate::models::converters::Converters;

/// Helper methods for converting output data (use in the paladins.rs module).
// *************************************************************************************************
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
    /// let model_name = ModelName::new()?;
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
    /// let model_name = ModelName {...};
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

    /// Get deleted count.
    // ---------------------------------------------------------------------------------------------
    pub fn deleted_count(&self) -> Result<i64, Box<dyn Error>> {
        match self {
            Self::Delete(data) => Ok(data.2),
            _ => Err("Invalid output type.")?,
        }
    }
}

/// Helper methods for converting output data (use in the paladins.rs module).
// *************************************************************************************************
#[derive(Debug)]
pub struct OutputDataCheck {
    is_valid: bool,
    final_doc: Option<Document>,
    final_model_json: Value,
    service_name: String,
    model_name: String,
    fields_name: Vec<String>,
}

impl Converters for OutputDataCheck {}

impl OutputDataCheck {
    /// Output data initialization.
    pub fn from(
        is_valid: bool,
        final_doc: Option<Document>,
        final_model_json: Value,
        service_name: String,
        model_name: String,
        fields_name: Vec<String>,
    ) -> Self {
        Self {
            is_valid,
            final_doc,
            final_model_json,
            service_name,
            model_name,
            fields_name,
        }
    }

    /// Get Hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check(None)?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.hash());
    /// ```
    ///
    pub fn hash(&self) -> String {
        self.final_model_json
            .get("hash")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    /// Get MongoDB ID from hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{:?}", output_data.object_id()?);
    /// ```
    ///
    pub fn object_id(&self) -> Result<ObjectId, Box<dyn Error>> {
        let hash_line = self.hash();
        let object_id = ObjectId::with_string(hash_line.as_str())?;
        Ok(object_id)
    }

    /// Get/Set final document
    // ---------------------------------------------------------------------------------------------
    /// ( Wig - Widgets )
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
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

    /// Get/Set Model instance in serde_json::Value format.
    // ---------------------------------------------------------------------------------------------
    /// ( Wig - Widgets )
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?
    /// ;
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{:?}", output_data.get_model_json());
    /// println!("{:?}", output_data.set_model_json(updated_widget_map));
    /// ```
    ///
    pub fn get_model_json(&self) -> Value {
        self.final_model_json.clone()
    }
    pub fn set_model_json(&mut self, new_model_json: Value) {
        self.final_model_json = new_model_json
    }

    /// Get Model instance in Json-line format.
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.to_json()?);
    /// ```
    ///
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(&self.final_model_json).unwrap())
    }

    /// Get widget list in json-line format for admin panel.
    // ---------------------------------------------------------------------------------------------
    /// ( converts a widget map to a list, in the order of the Model fields )
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
    ///
    /// let output_data = model_name.check()?;
    /// // or
    /// let output_data = model_name.save(None, None)?;
    ///
    /// println!("{}", output_data.to_json_for_admin()?);
    /// ```
    ///
    pub fn to_json_for_admin(&self) -> Result<String, Box<dyn Error>> {
        let mut widget_list: Vec<Value> = Vec::new();
        let hash = self
            .final_model_json
            .get("hash")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
            .unwrap();
        // Get a list of widgets in the order of the model fields.
        for field_name in self.fields_name.iter() {
            let mut widget = self.final_model_json.get(field_name).unwrap().clone();
            if field_name == "created_at" || field_name == "updated_at" {
                *widget.get_mut("is_hide").unwrap() = json!(false);
            }
            if field_name.contains("password") && !hash.is_empty() {
                *widget.get_mut("input_type").unwrap() = json!("hidden");
                *widget.get_mut("value").unwrap() = json!("");
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get validation status (boolean).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
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

    /// Get errors message ( for user side ).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName::new()?;
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
            let widget = self.final_model_json.get(field_name).unwrap();
            let error = widget.get("error").unwrap().as_str().unwrap();
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
    /// let model_name = ModelName::new()?;
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
}
