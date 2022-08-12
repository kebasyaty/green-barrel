//! Rendering HTML-controls code for Form.

use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

use crate::helpers::enumerations::{Enctype, HttpMethod};

/// Rendering HTML-controls code for Form.
pub trait GenerateHtml {
    /// Rendering HTML-controls code for Form.
    /// Hint: If necessary, customize the code generation yourself using html and
    /// css классы from Bootstrap, Material Design, etc.
    ///
    //// # Example:
    ///
    /// ```
    /// // For code customization.
    /// #[Model(
    ///     is_use_custom_html = true,
    /// )]
    /// #[derive(Serialize, Deserialize, Default, Debug)]
    /// pub struct ModelName {
    ///     Add your fields ...
    /// }
    ///
    /// impl GenerateHtml for ModelName {
    ///     fn generate_html(&self) {
    ///         Add your custom code...
    ///    }
    /// }
    /// ```
    ///
    fn generate_html(
        url_action: Option<&str>,
        http_method: Option<HttpMethod>,
        enctype: Option<Enctype>,
        service_name: &str,
        model_name: &str,
        fields_name: &Vec<String>,
        model_json: &Value,
    ) -> Result<String, Box<dyn Error>> {
        //
        // Controls of Form.
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        //
        for field_name in fields_name {
            let attrs = model_json.get(field_name).unwrap();
            // Alert message for the entire web form - Is required.
            // Hint: alternatively use in popup.
            let alert = attrs.get("alert").unwrap().as_str().unwrap();
            let input_type = attrs.get("input_type").unwrap().as_str().unwrap();
            let is_hide = attrs.get("is_hide").unwrap().as_bool().unwrap();
            let label = attrs.get("label").unwrap().as_str().unwrap();
            let id = attrs.get("id").unwrap().as_str().unwrap();
            let name = attrs.get("name").unwrap().as_str().unwrap();
            let required = attrs.get("required").unwrap().as_bool().unwrap();
            let disabled = attrs.get("disabled").unwrap().as_bool().unwrap();
            let readonly = attrs.get("readonly").unwrap().as_bool().unwrap();
            let placeholder = attrs.get("placeholder").unwrap().as_str().unwrap();
            let pattern = attrs.get("pattern").unwrap().as_str().unwrap();
            let other_attrs = attrs.get("other_attrs").unwrap().as_str().unwrap();
            let hint = attrs.get("hint").unwrap().as_str().unwrap();
            let warning = attrs.get("warning").unwrap().as_str().unwrap();
            let error = attrs.get("error").unwrap().as_str().unwrap();
            let css_classes = attrs.get("css_classes").unwrap().as_str().unwrap();
            //
            if !alert.is_empty() {
                controls = format!("<p class=\"warning\">{}</p>{}", alert, controls);
            }
            //
            match input_type {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    let value = attrs.get("value").unwrap().as_str().unwrap();
                    let minlength = attrs.get("minlength").unwrap().as_u64().unwrap();
                    let maxlength = attrs.get("maxlength").unwrap().as_u64().unwrap();
                    //
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        format!(" value=\"{}\"", value),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", placeholder)
                        } else {
                            String::new()
                        },
                        if !pattern.is_empty() {
                            format!(" pattern=\"{}\"", pattern)
                        } else {
                            String::new()
                        },
                        if !minlength > 0 {
                            format!(" minlength=\"{}\"", minlength)
                        } else {
                            String::new()
                        },
                        if !maxlength > 0 {
                            format!(" maxlength=\"{}\"", maxlength)
                        } else {
                            String::new()
                        },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "checkbox" => {
                    controls = format!(
                        "{}<p style=\"display:{};\"><input{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        if attrs.checked { " checked" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label>", id, label)
                        } else {
                            format!("<label for=\"{}\">{}:</label>", id, "Untitled")
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "radio" => {
                    let mut inputs = String::new();
                    for (idx, item) in attrs.options.iter().enumerate() {
                        inputs = format!(
                            "{}<p style=\"display:{};\"><input{}{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                            inputs,
                            if is_hide { "none" } else { "block" },
                            format!(" id=\"{}\"-{}", id, idx),
                            format!(" type=\"{}\"", input_type),
                            format!(" name=\"{}\"", name),
                            format!(" value=\"{}\"", item.0),
                            if item.0 == attrs.value {
                                " checked"
                            } else {
                                ""
                            },
                            if disabled { " disabled" } else { "" },
                            if readonly { " readonly" } else { "" },
                            if !css_classes.is_empty() {
                                format!(" class=\"{}\"", css_classes)
                            } else {
                                String::new()
                            },
                            if !other_attrs.is_empty() {
                                format!(" {}", other_attrs)
                            } else {
                                String::new()
                            },
                            if !label.is_empty() {
                                format!("<label for=\"{}\">{}:</label>", id, label)
                            } else {
                                format!("<label for=\"{}\">{}:</label>", id, "Untitled")
                            },
                            if !hint.is_empty() {
                                format!("<br><small class=\"hint\">{}</small>", hint)
                            } else {
                                String::new()
                            },
                            if !warning.is_empty() {
                                format!("<br><small class=\"warning\">{}</small>", warning)
                            } else {
                                String::new()
                            },
                            if !error.is_empty() {
                                format!("<br><small class=\"error\">{}</small>", error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    controls = format!("{}{}", controls, inputs);
                }
                "date" | "datetime" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        format!(" value=\"{}\"", attrs.value),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", placeholder)
                        } else {
                            String::new()
                        },
                        if !pattern.is_empty() {
                            format!(" pattern=\"{}\"", pattern)
                        } else {
                            String::new()
                        },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\" ", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "file" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "number" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        format!(" value=\"{}\"", attrs.value),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", placeholder)
                        } else {
                            String::new()
                        },
                        if attrs.step != "0" {
                            format!(" step=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.min != "0" {
                            format!(" min=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.max != "0" {
                            format!(" max=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "range" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        format!(" value=\"{}\"", attrs.value),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if attrs.step != "0" {
                            format!(" step=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.min != "0" {
                            format!(" min=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.max != "0" {
                            format!(" max=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "textarea" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<textarea{}{}{}{}{}{}{}{}{}{}>{}</textarea>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        format!(" name=\"{}\"", name),
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !minlength > 0 {
                            format!(" minlength=\"{}\"", minlength)
                        } else {
                            String::new()
                        },
                        if !maxlength > 0 {
                            format!(" maxlengt\"{}\"", maxlength)
                        } else {
                            String::new()
                        },
                        if !placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", placeholder)
                        } else {
                            String::new()
                        },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        attrs.value,
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "select" => {
                    let mut options = String::new();
                    for item in attrs.options.iter() {
                        options = format!(
                            "{}<option{}value=\"{}\">{}</option>",
                            options,
                            if attrs.value == item.0 {
                                " selected "
                            } else {
                                " "
                            },
                            item.0,
                            item.1
                        );
                    }
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<select{}{}{}{}{}{}{}>{}</select>{}{}{}</p>",
                        controls,
                        if is_hide { "none" } else { "block" },
                        if !label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", id, label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", id),
                        match attrs.widget.contains("Mult") {
                            true => format!(" name=\"{}[]\" multiple", name),
                            false => format!(" name=\"{}\"", name),
                        },
                        if required { " required" } else { "" },
                        if disabled { " disabled" } else { "" },
                        if readonly { " readonly" } else { "" },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        },
                        options,
                        if !hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", hint)
                        } else {
                            String::new()
                        },
                        if !warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", warning)
                        } else {
                            String::new()
                        },
                        if !error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", error)
                        } else {
                            String::new()
                        }
                    );
                }
                "hidden" => {
                    controls = format!(
                        "{}<input{}{}{}{}{}{}{}>",
                        controls,
                        format!(" id=\"{}\"", id),
                        format!(" type=\"{}\"", input_type),
                        format!(" name=\"{}\"", name),
                        format!(" value=\"{}\"", attrs.value),
                        if required { " required" } else { "" },
                        if !css_classes.is_empty() {
                            format!(" class=\"{}\"", css_classes)
                        } else {
                            String::new()
                        },
                        if !other_attrs.is_empty() {
                            format!(" {}", other_attrs)
                        } else {
                            String::new()
                        }
                    );
                }
                _ => Err(format!("Invalid input type."))?,
            }
        }
        // Add form and buttons
        // -----------------------------------------------------------------------------------------
        let service_name = service_name
            .split('_')
            .map(|word| {
                let mut chr: Vec<char> = word.chars().collect();
                chr[0] = chr[0].to_uppercase().nth(0).unwrap();
                chr.into_iter().collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("");
        let reset_btn = r#"<p><input type="reset" value="Reset"></p>"#;
        let submit_btn = r#"<p><input type="submit" value="Save"></p>"#;
        let form = format!(
            r#"<form id="{}-{}-Form" action="{}" method="{}" enctype="{}">{}</form>"#,
            service_name,
            model_name,
            url_action.unwrap_or("#"),
            http_method.unwrap_or_default().value(),
            enctype.unwrap_or_default().value(),
            format!("{}{}{}", controls, reset_btn, submit_btn)
        );
        //
        Ok(form)
    }
}
