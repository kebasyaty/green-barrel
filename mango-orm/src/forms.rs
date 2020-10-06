//! # Forms
//!
//! `Form` - Define form settings for models (widgets, html).

use crate::widgets::{Transport, Widget};
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

    // Customizing HTML form  (If necessary) for page templates
    // *********************************************************************************************
    // Call the method as Struct::form_html()
    fn html(
        attrs: HashMap<String, Transport>,
        model_name: &str,
        action: &str,
        method: String,
        enctype: String,
        build_controls: bool,
    ) -> Result<(String, String, String), Box<dyn Error>> {
        // Tag <form>
        // -----------------------------------------------------------------------------------------
        let form = format!(
            "<form id\"{}-form\" action=\"{}\" method=\"{}\" enctype=\"{}\">",
            model_name, action, method, enctype
        );

        // Controls of Form
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        if build_controls {
            for (_, trans) in attrs {
                let id_field = format!("{}--{}", model_name, trans.id);
                let label = format!(
                    "<p><label for=\"{}\">{}:</label><br>",
                    id_field, trans.label
                );
                match trans.field_type.as_str() {
                    "text" | "url" | "tel" | "password" | "email" | "color" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}></p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            trans.value,
                            if trans.maxlength > 0 { format!(" maxlength=\"{}\" ", trans.maxlength) } else { String::new() },
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 { format!(" class=\"{}\" ", trans.some_classes) } else { String::new() },
                            if trans.other_attrs.len() > 0 { format!(" {}", trans.other_attrs) } else { String::new()}
                        );
                    }
                    "checkbox" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}></p>",
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
                            }
                        );
                    }
                    "radio" => {
                        let mut tags = String::new();
                        for item in trans.select {
                            tags = format!(
                                "{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}>",
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
                                }
                            );
                        }
                        controls = format!("{}{}</p>", controls, tags);
                    }
                    "date" | "datetime" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}></p>",
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
                    "file" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}></p>",
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
                            }
                        );
                    }
                    "image" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\"{}{}{}></p>",
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
                            }
                        );
                    }
                    "number" => {
                        controls =
                            format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}></p>",
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
                            }
                        );
                    }
                    "range" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}{}{}{}></p>",
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
                            }
                        );
                    }
                    "textarea" => {
                        controls = format!(
                            "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\"{}{}{}>{}</textarea></p>",
                            controls,
                            label,
                            id_field,
                            trans.name,
                            trans.maxlength,
                            if trans.required { " required " } else { "" },
                            if trans.some_classes.len() > 0 { format!(" class=\"{}\" ", trans.some_classes) } else { String::new() },
                            if trans.other_attrs.len() > 0 { format!(" {}", trans.other_attrs) } else { String::new()},
                            trans.value,
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
                            "{}{}<select id=\"{}\" name=\"{}\"{}{}{}>{}</select></p>",
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
                        );
                    }
                    "hidden" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{}></p>",
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

        // Buttons of Form
        // -----------------------------------------------------------------------------------------
        let buttons = format!(
            "<input type=\"submit\" value=\"{}\">",
            if method == "get" { "Submit" } else { "Save" }
        );

        Ok((form, controls, buttons))
    }
}

// DYNAMIC FORM ARGUMENTS
// #################################################################################################
pub mod dynamic_arguments {
    // Method
    // (HTTP protocol method)
    // *********************************************************************************************
    pub enum Method {
        Get,
        Post,
    }

    impl Default for Method {
        fn default() -> Self {
            Method::Get
        }
    }

    impl Method {
        pub fn get_data(&self) -> String {
            match self {
                Self::Get => "get".to_string(),
                Self::Post => "post".to_string(),
            }
        }
    }

    // Enctype
    // (How to encode form data)
    // *********************************************************************************************
    pub enum Enctype {
        Application,
        Multipart,
        Text,
    }

    impl Default for Enctype {
        fn default() -> Self {
            Enctype::Application
        }
    }

    impl Enctype {
        pub fn get_data(&self) -> String {
            match self {
                Self::Application => "application/x-www-form-urlencoded".to_string(),
                Self::Multipart => "multipart/form-data".to_string(),
                Self::Text => "text/plain".to_string(),
            }
        }
    }
}

// POST-PROCESSING
// #################################################################################################
// The return type for the `save()` method
// (for post-processing)
#[derive(Debug)]
pub struct PostProcess {
    pub attrs_map: HashMap<String, Transport>,
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
