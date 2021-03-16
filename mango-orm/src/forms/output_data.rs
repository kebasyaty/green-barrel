//! # Output data types for Forms
//!
//! `OutputDataForm` - To return results after processing Forms.
//!

use crate::forms::{html_controls::HtmlControls, Widget};

// Output data type
#[derive(Debug)]
pub enum OutputDataForm {
    CheckForm(
        (
            bool,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            serde_json::value::Value,
        ),
    ),
    CheckModel(
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
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            String,
            serde_json::value::Value,
        ),
    ),
    Delete((bool, String)),
}

impl HtmlControls for OutputDataForm {
    // Get Html-line
    fn html(&self) -> String {
        match self {
            Self::CheckForm(data) => Self::to_html(&data.1, data.2.clone()),
            Self::CheckModel(data) => Self::to_html(&data.1, data.2.clone()),
            Self::Save(data) => Self::to_html(&data.1, data.2.clone()),
            _ => panic!("Invalid output type."),
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
            Self::CheckModel(data) => Ok(Self::to_hash(&data.2)?),
            Self::Save(data) => Ok(Self::to_hash(&data.2)?),
            _ => panic!("Invalid output type."),
        }
    }

    // Get MongoDB ID from hash-line
    pub fn id(&self) -> Result<mongodb::bson::oid::ObjectId, Box<dyn std::error::Error>> {
        match self {
            Self::CheckModel(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.2)?.as_str(),
            )?),
            Self::Save(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.2)?.as_str(),
            )?),
            _ => panic!("Invalid output type."),
        }
    }

    // Get Map of Widgets
    // ( Wig - Widgets )
    // ---------------------------------------------------------------------------------------------
    pub fn wig(&self) -> std::collections::HashMap<String, Widget> {
        match self {
            Self::CheckForm(data) => data.2.clone(),
            Self::CheckModel(data) => data.2.clone(),
            Self::Save(data) => data.2.clone(),
            _ => panic!("Invalid output type."),
        }
    }

    // Get Json-line
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::CheckForm(data) => Ok(serde_json::to_string(&data.2)?),
            Self::CheckModel(data) => Ok(serde_json::to_string(&data.2)?),
            Self::Save(data) => Ok(serde_json::to_string(&data.2)?),
            _ => panic!("Invalid output type."),
        }
    }

    // Json-line for admin panel.
    // ( converts a widget map to a list, in the order of the Model fields )
    pub fn json_for_admin(&self) -> Result<String, Box<dyn std::error::Error>> {
        let data = match self {
            Self::Save(data) => data,
            _ => panic!("Invalid output type."),
        };
        let fields_name = data.1.clone();
        let map_widgets = data.2.clone();
        let model_json = data.4.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        // Get a list of widgets in the order of the model fields.
        for field_name in fields_name {
            let mut widget = map_widgets.get(field_name.as_str()).unwrap().clone();
            let field_json = model_json[field_name].clone();
            if field_json.is_string() {
                widget.value = field_json.as_str().unwrap().to_string();
            } else if field_json.is_i64() {
                widget.value = field_json.as_i64().unwrap().to_string();
            } else if field_json.is_u64() {
                widget.value = field_json.as_u64().unwrap().to_string();
            } else if field_json.is_f64() {
                widget.value = field_json.as_f64().unwrap().to_string();
            } else if field_json.is_boolean() {
                widget.checked = field_json.as_bool().unwrap();
            } else if field_json.is_null() {
                widget.value = String::new();
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    // Get Boolean
    // ---------------------------------------------------------------------------------------------
    pub fn bool(&self) -> bool {
        match self {
            Self::CheckForm(data) => data.0,
            Self::CheckModel(data) => data.0,
            Self::Save(data) => data.0,
            Self::Delete(data) => data.0,
        }
    }

    // Get Document
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::CheckModel(data) => data.3.clone(),
            _ => panic!("Invalid output type."),
        }
    }

    // Get Form instance.
    // (It is convenient if the form passes (after validation) the value of the fields to Models.)
    // ---------------------------------------------------------------------------------------------
    pub fn form<T>(&self) -> Result<T, serde_json::error::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Self::CheckForm(data) => serde_json::from_value::<T>(data.3.clone()),
            _ => panic!("Invalid output type."),
        }
    }

    // A description of the error if the document was not deleted.
    // (Main use for admin panel.)
    // ---------------------------------------------------------------------------------------------
    pub fn err_msg(&self) -> String {
        match self {
            Self::Delete(data) => data.1.clone(),
            _ => panic!("Invalid output type."),
        }
    }
}
