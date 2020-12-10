//! #OutputData
//!
//! Output data for the `check()` and `save()` methods.
//!

use crate::forms::{html_controls, Widget};

// Output data
// ( Wig - Widgets )
#[derive(Debug)]
pub enum OutputData {
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

impl html_controls::HtmlControls for OutputData {
    // Get Html-line
    fn html(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(Self::to_html(&data.1, data.2.clone())?),
            Self::Save(data) => Ok(Self::to_html(&data.2, data.3.clone())?),
        }
    }
}

impl OutputData {
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

    // Get Map of Widgets
    // ( Wig - Widgets )
    // ---------------------------------------------------------------------------------------------
    pub fn wig(
        &self,
    ) -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(data.2.clone()),
            Self::Save(data) => Ok(data.3.clone()),
        }
    }

    // Get Json-line
    // ---------------------------------------------------------------------------------------------
    fn to_json(
        map_widgets: &std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut json_text = String::new();
        for (field_name, widget) in map_widgets {
            let widget_json = serde_json::to_string(&widget).unwrap();
            if !json_text.is_empty() {
                json_text = format!("{},\"{}\":{}", json_text, field_name, widget_json);
            } else {
                json_text = format!("\"{}\":{}", field_name, widget_json);
            }
        }
        Ok(format!("{{{}}}", json_text))
    }

    // Get Json-line
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(Self::to_json(&data.2)?),
            Self::Save(data) => Ok(Self::to_json(&data.3)?),
        }
    }

    // Get Boolean
    // ---------------------------------------------------------------------------------------------
    pub fn bool(&self) -> Result<bool, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(data.0),
            Self::Save(data) => Ok(data.0),
        }
    }

    // Get Document
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> Result<mongodb::bson::document::Document, Box<dyn std::error::Error>> {
        match self {
            Self::Check(data) => Ok(data.3.clone()),
            _ => panic!("Invalid output type."),
        }
    }
}
