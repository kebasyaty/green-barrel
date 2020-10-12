//! # Forms
//!
//! `Form` - Define form settings for models (widgets, html).

use crate::widgets::{Transport, Widget};
use mongodb::bson::document::Document;
use std::collections::HashMap;
use std::error::Error;

// FORM
// #################################################################################################
// Define form settings for models.
// *************************************************************************************************
pub trait Form {
    // Customizing widgets by model fields
    // *********************************************************************************************
    fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>>;

    // List of field names that will not be saved to the database
    // *********************************************************************************************
    fn ignore_fields() -> Vec<&'static str> {
        vec![]
    }

    // Customizing HTML form  (If necessary) for page templates
    // *********************************************************************************************
    // Call the method as Struct::form_html()
    fn html(
        attrs: HashMap<String, Transport>,
        model_name: &str,
        build_controls: bool,
    ) -> Result<String, Box<dyn Error>> {
        // Controls of Form
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        if build_controls {
            for (_, trans) in attrs {
                let id_field = format!("{}--{}", model_name, trans.id);
                let label = if trans.field_type != "hidden" {
                    format!(
                        "<p><label for=\"{}\">{}:</label><br>",
                        id_field, trans.label
                    )
                } else {
                    String::new()
                };
                match trans.field_type.as_str() {
                    "text" | "url" | "tel" | "password" | "email" | "color" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.maxlength > 0 { format!(" maxlength=\"{}\" ", trans.maxlength) } else { String::new() },
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 { format!(" class=\"{}\" ", trans.some_classes) } else { String::new() },
                            if trans.other_attrs.len() > 0 { format!(" {}", trans.other_attrs) } else { String::new()},
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "checkbox" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.checked { " checked " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "radio" => {
                        let mut tags = String::new();
                        for item in trans.select {
                            tags = format!(
                                "{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}",
                                label,
                                id_field,
                                trans.field_type,
                                trans.name,
                                item.1,
                                if trans.checked { " checked " } else { "" },
                                if trans.some_classes.len() > 0 {
                                    format!(" class=\"{}\" ", trans.some_classes)
                                } else {
                                    String::new()
                                },
                                if trans.other_attrs.len() > 0 {
                                    format!(" {}", trans.other_attrs)
                                } else {
                                    String::new()
                                },
                                if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                                if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                            );
                        }
                        controls = format!("{}{}</p>", controls, tags);
                    }
                    "date" | "datetime" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "file" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 {
                                format!("<br><small class=\"hint\">{}</small>", trans.hint)
                            } else {
                                String::new()
                            },
                            if trans.error.len() > 0 {
                                format!("<br><small class=\"error\">{}</small>", trans.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    "image" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 {
                                format!("<br><small class=\"hint\">{}</small>", trans.hint)
                            } else {
                                String::new()
                            },
                            if trans.error.len() > 0 {
                                format!("<br><small class=\"error\">{}</small>", trans.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    "number" => {
                        controls =
                            format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.required { " required " } else { "" },
                            if trans.step != "0" { format!(" step=\"{}\" ", trans.step) } else { String::new() },
                            if trans.min != "0" { format!(" min=\"{}\" ", trans.step) } else { String::new() },
                            if trans.max != "0" { format!(" max=\"{}\" ", trans.step) } else { String::new() },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "range" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.required { " required " } else { "" },
                            if trans.step != "0" { format!(" step=\"{}\" ", trans.step) } else { String::new() },
                            if trans.min != "0" { format!(" min=\"{}\" ", trans.step) } else { String::new() },
                            if trans.max != "0" { format!(" max=\"{}\" ", trans.step) } else { String::new() },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "textarea" => {
                        controls = format!(
                            "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\"{}{}{}>{}</textarea>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.name,
                            trans.maxlength,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 { format!(" class=\"{}\" ", trans.some_classes) } else { String::new() },
                            if trans.other_attrs.len() > 0 { format!(" {}", trans.other_attrs) } else { String::new()},
                            trans.value,
                            if trans.hint.len() > 0 { format!("<br><small class=\"hint\">{}</small>", trans.hint) } else { String::new() },
                            if trans.error.len() > 0 { format!("<br><small class=\"error\">{}</small>", trans.error) } else { String::new() }
                        );
                    }
                    "select" => {
                        let mut options = String::new();
                        for item in trans.select {
                            options = format!(
                                "{}<option{}value=\"{}\">{}</option>",
                                options,
                                if trans.value == item.1 {
                                    " selected "
                                } else {
                                    " "
                                },
                                item.1,
                                item.0
                            );
                        }
                        controls = format!(
                            "{}{}<select id=\"{}\" name=\"{}\"{}{}{}>{}</select>{}{}</p>",
                            controls,
                            label,
                            id_field,
                            trans.name,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            },
                            options,
                            if trans.hint.len() > 0 {
                                format!("<br><small class=\"hint\">{}</small>", trans.hint)
                            } else {
                                String::new()
                            },
                            if trans.error.len() > 0 {
                                format!("<br><small class=\"error\">{}</small>", trans.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    "hidden" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 {
                                format!(" class=\"{}\" ", trans.some_classes)
                            } else {
                                String::new()
                            },
                            if trans.other_attrs.len() > 0 {
                                format!(" {}", trans.other_attrs)
                            } else {
                                String::new()
                            }
                        );
                    }
                    _ => panic!("Invalid input type."),
                }
            }
        }
        // Add buttons
        // -----------------------------------------------------------------------------------------
        controls = format!("{}<input type=\"submit\" value=\"Save\">", controls);
        //
        Ok(controls)
    }
}

// OUTPUT TYPES FOR THE `SAVE()` METHOD
// #################################################################################################
// Output type
pub enum OutputType {
    Hash,
    Map,
    Json,
    Html,
}

// Output data
#[derive(Debug)]
pub enum OutputData {
    Hash((String, bool, Document)),
    Map((HashMap<String, Transport>, bool, Document)),
    Json((String, bool, Document)),
    Html((String, bool, Document)),
}

impl OutputData {
    // Get Hash-line
    pub fn hash(&self) -> &str {
        match self {
            Self::Hash(data) => &data.0,
            _ => panic!("`hash()` - Doesn't match the output type."),
        }
    }
    // Get Attribute Map
    pub fn map(&self) -> HashMap<String, Transport> {
        match self {
            Self::Map(data) => data.0.clone(),
            _ => panic!("`map()` - Doesn't match the output type."),
        }
    }
    // Get Json-line
    pub fn json(&self) -> &str {
        match self {
            Self::Json(data) => &data.0,
            _ => panic!("`json()` - Doesn't match the output type."),
        }
    }
    // Get Html-line
    pub fn html(&self) -> &str {
        match self {
            Self::Html(data) => &data.0,
            _ => panic!("`html()` - Doesn't match the output type."),
        }
    }
    // Get Boolean
    pub fn bool(&self) -> bool {
        match self {
            Self::Hash(data) => data.1,
            Self::Map(data) => data.1,
            Self::Json(data) => data.1,
            Self::Html(data) => data.1,
        }
    }
    // Get Document
    pub fn doc(&self) -> Document {
        match self {
            Self::Hash(data) => data.2.clone(),
            Self::Map(data) => data.2.clone(),
            Self::Json(data) => data.2.clone(),
            Self::Html(data) => data.2.clone(),
        }
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
