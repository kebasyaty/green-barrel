//! Rendering HTML-controls code for Form.

use std::collections::HashMap;
use std::error::Error;

use crate::widgets::{Enctype, HttpMethod, Widget};

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
        map_widgets: &HashMap<String, Widget>,
    ) -> Result<String, Box<dyn Error>> {
        //
        // Controls of Form.
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        for field_name in fields_name {
            let attrs = map_widgets.get(field_name).unwrap();
            // Messages common to the entire Form - Is required.
            // Hint: alternatively use in popup.
            if !attrs.common_msg.is_empty() {
                controls = format!("<p class=\"warning\">{}</p>{}", attrs.common_msg, controls);
            }
            match attrs.input_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.pattern.is_empty() {
                            format!(" pattern=\"{}\"", attrs.pattern)
                        } else {
                            String::new()
                        },
                        if !attrs.minlength > 0 {
                            format!(" minlength=\"{}\"", attrs.minlength)
                        } else {
                            String::new()
                        },
                        if !attrs.maxlength > 0 {
                            format!(" maxlength=\"{}\"", attrs.maxlength)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
                "checkbox" => {
                    controls = format!(
                        "{}<p style=\"display:{};\"><input{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.checked { " checked" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label>", attrs.id, attrs.label)
                        } else {
                            format!("<label for=\"{}\">{}:</label>", attrs.id, "Untitled")
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
                "radio" => {
                    let mut inputs = String::new();
                    for (idx, item) in attrs.options.iter().enumerate() {
                        inputs = format!(
                            "{}<p style=\"display:{};\"><input{}{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                            inputs,
                            if attrs.is_hide { "none" } else { "block" },
                            format!(" id=\"{}\"-{}", attrs.id, idx),
                            format!(" type=\"{}\"", attrs.input_type),
                            format!(" name=\"{}\"", attrs.name),
                            format!(" value=\"{}\"", item.0),
                            if item.0 == attrs.value {
                                " checked"
                            } else {
                                ""
                            },
                            if attrs.disabled { " disabled" } else { "" },
                            if attrs.readonly { " readonly" } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\"", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.label.is_empty() {
                                format!("<label for=\"{}\">{}:</label>", attrs.id, attrs.label)
                            } else {
                                format!("<label for=\"{}\">{}:</label>", attrs.id, "Untitled")
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
                    controls = format!("{}{}", controls, inputs);
                }
                "date" | "datetime" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.pattern.is_empty() {
                            format!(" pattern=\"{}\"", attrs.pattern)
                        } else {
                            String::new()
                        },
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
                "file" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
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
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
                "range" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<input{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
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
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
                "textarea" => {
                    controls = format!(
                        "{}<p style=\"display:{};\">{}<textarea{}{}{}{}{}{}{}{}{}{}>{}</textarea>{}{}{}</p>",
                        controls,
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.minlength > 0 {
                            format!(" minlength=\"{}\"", attrs.minlength)
                        } else {
                            String::new()
                        },
                        if !attrs.maxlength > 0 {
                            format!(" maxlengt\"{}\"", attrs.maxlength)
                        } else {
                            String::new()
                        },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        attrs.value,
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
                        if attrs.is_hide { "none" } else { "block" },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        match attrs.widget.contains("Mult") {
                            true => format!(" name=\"{}[]\" multiple", attrs.name),
                            false => format!(" name=\"{}\"", attrs.name),
                        },
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
                        "{}<input{}{}{}{}{}{}{}>",
                        controls,
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
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
            url_action.unwrap_or("/"),
            http_method.unwrap_or_default().value(),
            enctype.unwrap_or_default().value(),
            format!("{}{}{}", controls, reset_btn, submit_btn)
        );
        //
        Ok(form)
    }
}
