//! # Forms
//!
//! `Form` - Define form settings for models (widgets, html).

use crate::widgets::{Transport, Widget};
use mongodb::bson::document::Document;
use std::collections::HashMap;
use std::error::Error;

// FORM
// #################################################################################################
// Form settings
// *************************************************************************************************
pub trait Form {
    // Customizing widgets by model fields
    // *********************************************************************************************
    fn widgets<'a>() -> Result<HashMap<&'a str, Widget>, Box<dyn Error>>;

    // Customizing HTML form  (If necessary) for page templates
    // *********************************************************************************************
    // Call the method as Struct::form_html()
    fn html(
        attrs_map: HashMap<String, Transport>,
        model_name: &str,
        build_controls: bool,
    ) -> Result<String, Box<dyn Error>> {
        // Controls of Form
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        if build_controls {
            for (_, attrs) in attrs_map {
                let id_field = format!("{}--{}", model_name, attrs.id);
                let label = if attrs.field_type != "hidden" {
                    format!(
                        "<p><label for=\"{}\">{}:</label><br>",
                        id_field, attrs.label
                    )
                } else {
                    String::new()
                };
                match attrs.field_type.as_str() {
                    "text" | "url" | "tel" | "password" | "email" | "color" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.maxlength > 0 { format!(" maxlength=\"{}\" ", attrs.maxlength) } else { String::new() },
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() { format!(" class=\"{}\" ", attrs.css_classes) } else { String::new() },
                            if !attrs.other_attrs.is_empty() { format!(" {}", attrs.other_attrs) } else { String::new()},
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "checkbox" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.checked { " checked " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "radio" => {
                        let mut tags = String::new();
                        for item in attrs.select {
                            tags = format!(
                                "{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}{}",
                                label,
                                id_field,
                                attrs.field_type,
                                attrs.name,
                                item.1,
                                if attrs.checked { " checked " } else { "" },
                                if !attrs.css_classes.is_empty() {
                                    format!(" class=\"{}\" ", attrs.css_classes)
                                } else {
                                    String::new()
                                },
                                if !attrs.other_attrs.is_empty() {
                                    format!(" {}", attrs.other_attrs)
                                } else {
                                    String::new()
                                },
                                if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                                if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                                if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                            );
                        }
                        controls = format!("{}{}</p>", controls, tags);
                    }
                    "date" | "datetime" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "file" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() {
                                format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                            } else {
                                String::new()
                            },
                            if !attrs.warning.is_empty() {
                                format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                            } else {
                                String::new()
                            },
                            if !attrs.error.is_empty() {
                                format!("<br><small class=\"error\">{}</small>", attrs.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    "image" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() {
                                format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                            } else {
                                String::new()
                            },
                            if !attrs.warning.is_empty() {
                                format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                            } else {
                                String::new()
                            },
                            if !attrs.error.is_empty() {
                                format!("<br><small class=\"error\">{}</small>", attrs.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    "number" => {
                        controls =
                            format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.required { " required " } else { "" },
                            if attrs.step != "0" { format!(" step=\"{}\" ", attrs.step) } else { String::new() },
                            if attrs.min != "0" { format!(" min=\"{}\" ", attrs.step) } else { String::new() },
                            if attrs.max != "0" { format!(" max=\"{}\" ", attrs.step) } else { String::new() },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "range" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.required { " required " } else { "" },
                            if attrs.step != "0" { format!(" step=\"{}\" ", attrs.step) } else { String::new() },
                            if attrs.min != "0" { format!(" min=\"{}\" ", attrs.step) } else { String::new() },
                            if attrs.max != "0" { format!(" max=\"{}\" ", attrs.step) } else { String::new() },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "textarea" => {
                        controls = format!(
                            "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\"{}{}{}>{}</textarea>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.name,
                            attrs.maxlength,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() { format!(" class=\"{}\" ", attrs.css_classes) } else { String::new() },
                            if !attrs.other_attrs.is_empty() { format!(" {}", attrs.other_attrs) } else { String::new()},
                            attrs.value,
                            if !attrs.hint.is_empty() { format!("<br><small class=\"hint\">{}</small>", attrs.hint) } else { String::new() },
                            if !attrs.warning.is_empty() { format!("<br><small class=\"warning\">{}</small>", attrs.warning) } else { String::new() },
                            if !attrs.error.is_empty() { format!("<br><small class=\"error\">{}</small>", attrs.error) } else { String::new() }
                        );
                    }
                    "select" => {
                        let mut options = String::new();
                        for item in attrs.select {
                            options = format!(
                                "{}<option{}value=\"{}\">{}</option>",
                                options,
                                if attrs.value == item.1 {
                                    " selected "
                                } else {
                                    " "
                                },
                                item.1,
                                item.0
                            );
                        }
                        controls = format!(
                            "{}{}<select id=\"{}\" name=\"{}\"{}{}{}>{}</select>{}{}{}</p>",
                            controls,
                            label,
                            id_field,
                            attrs.name,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            options,
                            if !attrs.hint.is_empty() {
                                format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                            } else {
                                String::new()
                            },
                            if !attrs.warning.is_empty() {
                                format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                            } else {
                                String::new()
                            },
                            if !attrs.error.is_empty() {
                                format!("<br><small class=\"error\">{}</small>", attrs.error)
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
                            attrs.field_type,
                            attrs.name,
                            attrs.value,
                            if attrs.required { " required " } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\" ", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
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
