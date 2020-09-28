//! # Forms
//!
//! `Form` - Define form settings for models.

use crate::widgets::{Transport, Widget};
use std::collections::HashMap;
use std::error::Error;

// FORMS
// =================================================================================================
/// Define form settings for models.
/// ************************************************************************************************
pub trait Form {
    // Customizing widgets by model fields ---------------------------------------------------------
    fn widgets() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>>;
    // Customizing HTML form  (If necessary) for page templates
    fn html(
        attrs: HashMap<String, Transport>,
        model_name: &str,
        action: &str,
        method: String,
        enctype: &str,
    ) -> Result<String, Box<dyn Error>> {
        let mut form_text = format!(
            "<form id\"{}-form\" action=\"{}\" method=\"{}\" enctype=\"{}\">",
            model_name, action, method, enctype
        );
        for (_, trans) in attrs {
            let id_field = format!("{}--{}", model_name, trans.id);
            let label = format!(
                "<p><label for=\"{}\">{}:</label><br>",
                id_field, trans.label
            );
            match trans.field_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class={} {}></p>",
                        form_text,
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
                    form_text = format!("{}{}</p>", form_text, tags);
                }
                "date" | "datetime" => {
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>{}</textarea></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>{}</select></p>",
                        form_text,
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
                    form_text = format!(
                        "{}{}<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}></p>",
                        form_text,
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
            "{}<input type=\"submit\" value=\"{}\"></form>",
            form_text,
            if method == "get" { "Submit" } else { "Save" }
        ))
    }
}

// TESTS
// =================================================================================================
#[cfg(test)]
mod tests {
    //
}
