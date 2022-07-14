//! Output data for QPaladins.

use mongodb::bson::{document::Document, oid::ObjectId};
use std::collections::HashMap;
use std::error::Error;

use crate::widgets::{generate_html::GenerateHtml, Enctype, HttpMethod, Widget};

/// Helper methods for converting output data (use in the paladins.rs module).
#[derive(Debug)]
pub enum OutputData {
    Check(
        (
            bool,
            Vec<String>,
            HashMap<String, Widget>,
            Document,
            String,
            String,
        ),
    ),
    Save((bool, Vec<String>, HashMap<String, Widget>, String, String)),
    Delete((bool, String)),
    UpdatePassword((bool, String)),
    Stub,
}

impl GenerateHtml for OutputData {}

impl OutputData {
    //
    /// Get Hash-line
    // ---------------------------------------------------------------------------------------------
    fn get_hash(map_widgets: &HashMap<String, Widget>) -> Result<String, Box<dyn Error>> {
        Ok(map_widgets.get("hash").unwrap().value.clone())
    }

    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{}", output_data.hash()?);
    /// ```
    ///
    pub fn hash(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(Self::get_hash(&data.2)?),
            Self::Save(data) => Ok(Self::get_hash(&data.2)?),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Get MongoDB ID from hash-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{:?}", output_data.object_id()?);
    /// ```
    ///
    pub fn object_id(&self) -> Result<ObjectId, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(ObjectId::with_string(Self::get_hash(&data.2)?.as_str())?),
            Self::Save(data) => Ok(ObjectId::with_string(Self::get_hash(&data.2)?.as_str())?),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Get Map of Widgets
    // ---------------------------------------------------------------------------------------------
    /// ( Wig - Widgets )
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{:?}", output_data.to_wig()?);
    /// ```
    ///
    pub fn to_wig(&self) -> Result<HashMap<String, Widget>, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(data.2.clone()),
            Self::Save(data) => Ok(data.2.clone()),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Get Json-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{}", output_data.to_json()?);
    /// ```
    ///
    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(serde_json::to_string(&data.2)?),
            Self::Save(data) => Ok(serde_json::to_string(&data.2)?),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Json-line for admin panel.
    // ---------------------------------------------------------------------------------------------
    /// ( converts a widget map to a list, in the order of the Model fields )
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{}", output_data.to_json_for_admin()?);
    /// ```
    ///
    pub fn to_json_for_admin(&self) -> Result<String, Box<dyn Error>> {
        let data = match self {
            Self::Save(data) => data,
            _ => Err("Invalid output type.")?,
        };
        let map_widgets = data.2.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        let hash = map_widgets.get("hash").unwrap().clone().value;
        // Get a list of widgets in the order of the model fields.
        for field_name in data.1.iter() {
            let mut widget = map_widgets.get(field_name).unwrap().clone();
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
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
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
        match self {
            Self::Check(data) => Self::generate_html(
                action,
                method,
                enctype,
                data.4.as_str(),
                data.5.as_str(),
                &data.1,
                &data.2,
            ),
            Self::Save(data) => Self::generate_html(
                action,
                method,
                enctype,
                data.3.as_str(),
                data.4.as_str(),
                &data.1,
                &data.2,
            ),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Get validation status (boolean).
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// let output_data = user_profile.delete()?;
    /// let output_data = user_profile.update_password()?;
    /// assert!(result.is_valid()?);
    /// ```
    ///
    pub fn is_valid(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(data.0),
            Self::Save(data) => Ok(data.0),
            Self::Delete(data) => Ok(data.0),
            Self::UpdatePassword(data) => Ok(data.0),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Get Document
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// println!("{:?}", user_profile.to_doc()?);
    /// ```
    ///
    pub fn to_doc(&self) -> Result<Document, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(data.3.clone()),
            _ => Err("Invalid output type.")?,
        }
    }

    /// Printing errors to the console ( for development ).
    // ---------------------------------------------------------------------------------------------
    fn print_to_console(map_widgets: &HashMap<String, Widget>) {
        let mut errors = String::new();
        for (field_name, widget) in map_widgets {
            let tmp = errors.clone();
            if !widget.error.is_empty() {
                errors = format!("{}\nField: `{}` -> {}", tmp, field_name, widget.error);
            }
        }
        if !errors.is_empty() {
            errors = errors.replace("<br>", " | ");
            println!("\nErrors:{}\n", errors);
        }
    }

    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{}", output_data.print_err()?);
    /// ```
    ///
    pub fn print_err(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Check(data) => {
                Self::print_to_console(&data.2);
                Ok(())
            }
            Self::Save(data) => {
                Self::print_to_console(&data.2);
                Ok(())
            }
            _ => Err("Invalid output type.")?,
        }
    }

    /// Description of the error if the document was not deleted or the password was not updated.
    // ---------------------------------------------------------------------------------------------
    /// (Main use for admin panel.)
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// user_profile.save(None, None)?;
    /// let output_data = user_profile.delete()?;
    /// println!("{}", output_data.err_msg()?);
    /// ```
    ///
    pub fn err_msg(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Delete(data) => Ok(data.1.clone()),
            Self::UpdatePassword(data) => Ok(data.1.clone()),
            _ => Err("Invalid output type.")?,
        }
    }
}
