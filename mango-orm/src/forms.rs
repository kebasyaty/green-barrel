//! # Forms
//!
//! `Form` - Define form settings for models (widgets, html).

use crate::widgets::{Transport, Widget};
use std::collections::HashMap;
use std::error::Error;

// FORMS
// #################################################################################################
/// Define form settings for models.
/// ************************************************************************************************
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
        method: String,
    ) -> Result<(String, String), Box<dyn Error>> {
        // Other attributes of the `form` tag.
        // Will be automatically added - "id\"{}-form\" action=\"{}\" method=\"{}\" enctype=\"{}\""
        // -----------------------------------------------------------------------------------------
        // Example: format!("name=\"{}\" class=\"{}\", name, class, etc...)
        let other_form_attributes = "".to_string();

        // Controles Form
        // -----------------------------------------------------------------------------------------
        let mut controles_html = String::new();
        for (_, trans) in attrs {
            let id_field = format!("{}--{}", model_name, trans.id);
            let label = format!(
                "<p><label for=\"{}\">{}:</label><br>",
                id_field, trans.label
            );
            match trans.field_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
                        label,
                        id_field,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "checkbox" => {
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}></p>",
                        controles_html,
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
                    controles_html = format!("{}{}</p>", controles_html, tags);
                }
                "date" | "datetime" => {
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>{}</textarea></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>{}</select></p>",
                        controles_html,
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
                    controles_html = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controles_html,
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
        Ok((
            format!(
                "{}<input type=\"submit\" value=\"{}\"></form>",
                controles_html,
                if method == "get" { "Submit" } else { "Save" }
            ),
            other_form_attributes,
        ))
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
