//! # Output data types for Forms
//!
//! `OutputDataForm` - To return results after processing Forms.
//!

use crate::forms::{html_controls::HtmlControls, Widget};

// Output data type
#[derive(Debug)]
pub enum OutputDataForm {
    Check(
        (
            bool,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            mongodb::bson::document::Document,
        ),
    ),
    Save(
        (
            bool,
            String,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
        ),
    ),
}

impl HtmlControls for OutputDataForm {
    // Get Html-line
    fn html(&self) -> String {
        match self {
            Self::Check(data) => Self::to_html(&data.1, data.2.clone()),
            Self::Save(data) => Self::to_html(&data.2, data.3.clone()),
        }
    }
}

impl OutputDataForm {
    // Get Hash-line
    // ---------------------------------------------------------------------------------------------
    fn to_hash(
        map_widgets: &std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut errors = String::new();
        for (field_name, widget) in map_widgets {
            let tmp = if !errors.is_empty() {
                format!("{} ; ", errors)
            } else {
                String::new()
            };
            if !widget.error.is_empty() {
                errors = format!("{}Field: `{}` - {}", tmp, field_name, widget.error);
            }
        }
        if !errors.is_empty() {
            Err(errors.replace("<br>", " | "))?
        }
        Ok(map_widgets.get(&"hash".to_owned()).unwrap().value.clone())
    }
    // Get Hash-line
    pub fn hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(Self::to_hash(&data.2)?),
            Self::Save(data) => Ok(Self::to_hash(&data.3)?),
        }
    }
    // Get MongoDB ID from hash-line
    pub fn id(&self) -> Result<mongodb::bson::oid::ObjectId, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.2)?.as_str(),
            )?),
            Self::Save(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.3)?.as_str(),
            )?),
        }
    }

    // Get Map of Widgets
    // ( Wig - Widgets )
    // ---------------------------------------------------------------------------------------------
    pub fn wig(&self) -> std::collections::HashMap<String, Widget> {
        match self {
            Self::Check(data) => data.2.clone(),
            Self::Save(data) => data.3.clone(),
        }
    }

    // Get Json-line
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(serde_json::to_string(&data.2)?),
            Self::Save(data) => Ok(serde_json::to_string(&data.3)?),
        }
    }

    // Get Boolean
    // ---------------------------------------------------------------------------------------------
    pub fn bool(&self) -> bool {
        match self {
            Self::Check(data) => data.0,
            Self::Save(data) => data.0,
        }
    }

    // Get Document
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::Check(data) => data.3.clone(),
            _ => panic!("Invalid output type."),
        }
    }
}
