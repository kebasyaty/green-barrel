//! Output data for QPaladins.

use crate::widgets::{html_controls::HtmlControls, Widget};
use mongodb::bson::{document::Document, oid::ObjectId};
use std::collections::HashMap;
use std::error::Error;

/// Helper methods for converting output data (use in the paladins.rs module).
#[derive(Debug)]
pub enum OutputDataForm {
    Check((bool, Vec<String>, HashMap<String, Widget>, Document)),
    Save((bool, Vec<String>, HashMap<String, Widget>, String)),
    Delete((bool, String)),
    Stub,
}

impl HtmlControls for OutputDataForm {
    /// Get Html-line
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// println!("{}", output_data.to_html());
    /// ```
    ///
    fn to_html(&self) -> String {
        match self {
            Self::Check(data) => Self::generate_html(&data.1, data.2.clone()),
            Self::Save(data) => Self::generate_html(&data.1, data.2.clone()),
            _ => panic!("Invalid output type."),
        }
    }
}

impl OutputDataForm {
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
    /// println!("{}", output_data.hash());
    /// ```
    ///
    pub fn hash(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Self::Check(data) => Ok(Self::get_hash(&data.2)?),
            Self::Save(data) => Ok(Self::get_hash(&data.2)?),
            _ => panic!("Invalid output type."),
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
    /// println!("{}", output_data.print_err());
    /// ```
    ///
    pub fn print_err(&self) {
        match self {
            Self::Check(data) => Self::print_to_console(&data.2),
            Self::Save(data) => Self::print_to_console(&data.2),
            _ => panic!("Invalid output type."),
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
            _ => panic!("Invalid output type."),
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
    /// println!("{:?}", output_data.to_wig());
    /// ```
    ///
    pub fn to_wig(&self) -> HashMap<String, Widget> {
        match self {
            Self::Check(data) => data.2.clone(),
            Self::Save(data) => data.2.clone(),
            _ => panic!("Invalid output type."),
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
            _ => panic!("Invalid output type."),
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
            _ => panic!("Invalid output type."),
        };
        let map_widgets = data.2.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        let hash = map_widgets.get("hash").unwrap().clone().value;
        // Get a list of widgets in the order of the model fields.
        for field_name in data.1.iter() {
            let mut widget = map_widgets.get(field_name).unwrap().clone();
            if field_name.contains("password") && !hash.is_empty() {
                widget.widget = "hiddenText".to_string();
                widget.input_type = "hidden".to_string();
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get validation status (boolean)
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// let output_data = user_profile.check()?;
    /// let output_data = user_profile.save(None, None)?;
    /// let output_data = user_profile.delete()?;
    /// assert!(result.is_valid());
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Check(data) => data.0,
            Self::Save(data) => data.0,
            Self::Delete(data) => data.0,
            _ => panic!("Invalid output type."),
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
    /// println!("{:?}", user_profile.to_doc());
    /// ```
    ///
    pub fn to_doc(&self) -> Document {
        match self {
            Self::Check(data) => data.3.clone(),
            _ => panic!("Invalid output type."),
        }
    }

    /// A description of the error if the document was not deleted.
    // ---------------------------------------------------------------------------------------------
    /// (Main use for admin panel.)
    ///
    /// # Example:
    ///
    /// ```
    /// let user_profile = UserProfile {...};
    /// user_profile.save(None, None)?;
    /// let output_data = user_profile.delete()?;
    /// println!("{}", output_data.err_msg());
    /// ```
    ///
    pub fn err_msg(&self) -> String {
        match self {
            Self::Delete(data) => data.1.clone(),
            _ => panic!("Invalid output type."),
        }
    }
}
