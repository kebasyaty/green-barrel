//! Output data for QPaladins.

use mongodb::bson::{document::Document, oid::ObjectId};
use std::collections::HashMap;
use std::error::Error;

use crate::{
    helpers::{Enctype, HttpMethod},
    models::converters::Converters,
    widgets::{generate_html::GenerateHtml, Widget},
};

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
    /// let model_name = ModelName {...};
    /// let output_data = model_name.delete()?;
    /// let output_data = model_name.update_password()?;
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
    /// let output_data = model_name.delete()?;
    /// let output_data = model_name.update_password()?;
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
    final_widget_map: HashMap<String, Widget>,
    service_name: String,
    model_name: String,
    fields_name: Vec<String>,
}

impl GenerateHtml for OutputDataCheck {}
impl Converters for OutputDataCheck {}

impl OutputDataCheck {
    /// Output data initialization.
    pub fn from(
        is_valid: bool,
        final_doc: Option<Document>,
        final_widget_map: HashMap<String, Widget>,
        service_name: String,
        model_name: String,
        fields_name: Vec<String>,
    ) -> Self {
        Self {
            is_valid,
            final_doc,
            final_widget_map,
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
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// println!("{}", output_data.hash());
    /// ```
    ///
    pub fn hash(&self) -> String {
        self.final_widget_map.get("hash").unwrap().value.clone()
    }

    /// Get MongoDB ID from hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
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
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
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

    /// Get/Set Map of Widgets
    // ---------------------------------------------------------------------------------------------
    /// ( Wig - Widgets )
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// println!("{:?}", output_data.to_wig());
    /// println!("{:?}", output_data.set_wig(updated_widget_map));
    /// ```
    ///
    pub fn to_wig(&self) -> HashMap<String, Widget> {
        self.final_widget_map.clone()
    }
    pub fn set_wig(&mut self, new_widget_map: HashMap<String, Widget>) {
        self.final_widget_map = new_widget_map
    }

    /// Get Json-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// println!("{}", output_data.to_json()?);
    /// ```
    ///
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        Self::widget_map_to_json(self.final_widget_map.clone())
    }

    /// Json-line for admin panel.
    // ---------------------------------------------------------------------------------------------
    /// ( converts a widget map to a list, in the order of the Model fields )
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// println!("{}", output_data.to_json_for_admin()?);
    /// ```
    ///
    pub fn to_json_for_admin(&self) -> Result<String, Box<dyn Error>> {
        let mut widget_list: Vec<Widget> = Vec::new();
        let hash = self.final_widget_map.get("hash").unwrap().clone().value;
        // Get a list of widgets in the order of the model fields.
        for field_name in self.fields_name.iter() {
            let mut widget = self.final_widget_map.get(field_name).unwrap().clone();
            if field_name == "created_at" || field_name == "updated_at" {
                widget.is_hide = false;
            }
            if field_name.contains("password") && !hash.is_empty() {
                widget.widget = "hiddenText".to_string();
                widget.input_type = "hidden".to_string();
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get Html-code
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// //
    /// println!("{}", output_data.to_html(None, None, None)?);
    /// // OR
    /// println!("{}", output_data.to_html(Some("/login"), Some(HttpMethod::POST), Some(Enctype::Multipart))?);
    /// ```
    ///
    pub fn to_html(
        &self,
        action: Option<&str>,
        method: Option<HttpMethod>,
        enctype: Option<Enctype>,
    ) -> Result<String, Box<dyn Error>> {
        Self::generate_html(
            action,
            method,
            enctype,
            &self.service_name,
            &self.model_name,
            &self.fields_name,
            &self.final_widget_map,
        )
    }

    /// Get validation status (boolean).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
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
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
    /// if output_data.is_valid() {
    ///     println!("{}", output_data.err_msg());
    /// }
    /// ```
    ///
    pub fn err_msg(&self) -> String {
        let mut errors = String::new();
        for (field_name, widget) in self.final_widget_map.iter() {
            let tmp = errors.clone();
            if !widget.error.is_empty() {
                errors = format!("{}\nField: `{}` => {}", tmp, field_name, widget.error);
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
    /// let model_name = ModelName {...};
    /// let output_data = model_name.check()?;
    /// let output_data = model_name.save(None, None)?;
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