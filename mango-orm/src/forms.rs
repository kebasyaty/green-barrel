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
    ) -> Result<String, Box<dyn Error>> {
        // Other attributes of the `form` tag.
        // Will be automatically added - "id\"{}-form\" action=\"{}\" method=\"{}\" enctype=\"{}\""
        // -----------------------------------------------------------------------------------------
        // Example: format!("name=\"{}\" class=\"{}\", name, class, etc...)
        let other_form_attributes = format!("");

        // Controles Form
        // -----------------------------------------------------------------------------------------
        let mut controls_form = String::new();
        for (_, trans) in attrs {
            let id_field = format!("{}--{}", model_name, trans.id);
            let label = format!(
                "<p><label for=\"{}\">{}:</label><br>",
                id_field, trans.label
            );
            match trans.field_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}></p>",
                        controls_form,
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
                    controls_form = format!("{}{}</p>", controls_form, tags);
                }
                "date" | "datetime" => {
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>{}</textarea></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>{}</select></p>",
                        controls_form,
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
                    controls_form = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        controls_form,
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
        Ok(format!(
            "{}>{}<input type=\"submit\" value=\"{}\"></form>",
            other_form_attributes,
            controls_form,
            if method == "get" { "Submit" } else { "Save" }
        ))
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
