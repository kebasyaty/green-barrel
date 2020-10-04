//! # Forms
//!
//! `Form` - Define form settings for models (widgets, html).

use crate::widgets::{Transport, Widget};
use std::collections::HashMap;
use std::error::Error;

// FORMS
// #################################################################################################
/// Define form settings for models.
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
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\"{}{}{} {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.maxlength > 0 { format!(" maxlength=\"{}\" ", trans.maxlength) } else { String::new() },
                        if trans.required { " required " } else { "" },
                        if trans.some_classes.len() > 0 { format!(" class=\"{}\" ", trans.some_classes) } else { String::new() },
                        trans.other_attrs
                    );
                    }
                    "checkbox" => {
                        controls = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.checked { "checked" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                    }
                    "radio" => {
                        let mut tags = String::new();
                        for item in trans.select {
                            tags = format!(
                            "{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}>",
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            item.1,
                            if trans.checked { "checked" } else { "" },
                            trans.some_classes,
                            trans.other_attrs
                        );
                        }
                        controls = format!("{}{}</p>", controls, tags);
                    }
                    "date" | "datetime" => {
                        controls = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                    }
                    "file" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            if trans.required { "required" } else { "" },
                            trans.some_classes,
                            trans.other_attrs
                        );
                    }
                    "image" => {
                        controls = format!(
                            "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                            controls,
                            label,
                            id_field,
                            trans.field_type,
                            trans.name,
                            if trans.required { "required" } else { "" },
                            trans.some_classes,
                            trans.other_attrs
                        );
                    }
                    "number" => {
                        controls = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                    }
                    "range" => {
                        controls = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                    }
                    "textarea" => {
                        controls = format!(
                        "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>{}</textarea></p>",
                        controls,
                        label,
                        id_field,
                        trans.name,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs,
                        trans.value,
                    );
                    }
                    "select" => {
                        let mut options = String::new();
                        for item in trans.select {
                            options = format!(
                                "{}<option {} value=\"{}\">{}</option>",
                                options,
                                if trans.value == item.1 {
                                    "selected"
                                } else {
                                    ""
                                },
                                item.1,
                                item.0
                            );
                        }
                        controls = format!(
                            "{}{}<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>{}</select></p>",
                            controls,
                            label,
                            id_field,
                            trans.name,
                            if trans.required { "required" } else { "" },
                            trans.some_classes,
                            trans.other_attrs,
                            options,
                        );
                    }
                    "hidden" => {
                        controls = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
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

/// DYNAMIC FORM ARGUMENTS
/// ################################################################################################
pub mod dynamic_arguments {
    /// Method
    /// (HTTP protocol method)
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

    /// Enctype
    /// (How to encode form data)
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

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
