//! # Forms
//!
//! `ToForm` - Define form settings for models (widgets, html).
//! `Widget` - Form controls parameters.
//! `OutputType` - Output types for the `save()` method.
//! `TransMapWidgetType` - For transporting of Widget types map to implementation of methods.
//! `TransMapWidgets` - For transporting of Widgets map to implementation of methods.

// FORMS
// #################################################################################################
// Widget
// ( Form controls parameters )
// *************************************************************************************************
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct Widget {
    pub id: String, // "model-name--field-name" ( The value is determined automatically )
    pub label: String,
    pub widget: String,
    pub input_type: String, // The value is determined automatically
    pub name: String,       // The value is determined automatically
    pub value: String,
    pub placeholder: String,
    pub pattern: String, // Validating a field using a client-side regex
    pub minlength: usize,
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub unique: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub step: String,
    pub min: String,
    pub max: String,
    pub other_attrs: String, // "autofocus multiple size=\"some number\" ..."
    pub css_classes: String, // "class-name class-name ..."
    pub options: Vec<(String, String)>, // <value, Title>
    pub hint: String,
    pub warning: String, // The value is determined automatically
    pub error: String,   // The value is determined automatically
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            id: String::new(),
            label: String::new(),
            widget: String::from("inputText"),
            input_type: String::from("text"),
            name: String::new(),
            value: String::new(),
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            checked: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: String::from("0"),
            min: String::from("0"),
            max: String::from("0"),
            other_attrs: String::new(),
            css_classes: String::new(),
            options: Vec::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
        }
    }
}

// For transporting of Widgets map to implementation of methods
// <field name, Widget>
#[derive(serde::Deserialize)]
pub struct TransMapWidgets {
    pub map_widgets: std::collections::HashMap<String, Widget>,
}

// Form settings
// *************************************************************************************************
pub trait ToForm {
    // Get a store key
    // ( key = collection name, used in forms exclusively for store access )
    // ---------------------------------------------------------------------------------------------
    fn key_store() -> Result<String, Box<dyn std::error::Error>>;

    // Get map of widgets for Form fields
    // <field name, Widget>
    fn widgets() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>>;
}

pub trait HtmlControls {
    // Rendering HTML-controls code for Form
    // ( If necessary, customize the code generation yourself using html and css from
    // Bootstrap, Material Design, etc. )
    fn to_html(
        fields_name: &Vec<String>,
        map_widgets: std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Controls of Form
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        for field_name in fields_name {
            let attrs = map_widgets.get(field_name).unwrap();
            match attrs.input_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
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
                        "{}<p><input{}{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                        controls,
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(
                            " value=\"{}\"",
                            if attrs.widget == "checkBoxBool".to_string() {
                                true.to_string()
                            } else {
                                attrs.value.clone()
                            }
                        ),
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
                    for item in attrs.options.iter() {
                        inputs = format!(
                            "{}<p><input{}{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                            inputs,
                            format!(" id=\"{}\"", attrs.id),
                            format!(" type=\"{}\"", attrs.input_type),
                            format!(" name=\"{}\"", attrs.name),
                            format!(" value=\"{}\"", item.0),
                            if attrs.checked { " checked " } else { "" },
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
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
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
                        "{}<p>{}<input{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
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
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
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
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
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
                        "{}<p>{}<textarea{}{}{}{}{}{}{}{}{}{}>{}</textarea>{}{}{}</p>",
                        controls,
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
                        "{}<p>{}<select{}{}{}{}{}{}{}>{}</select>{}{}{}</p>",
                        controls,
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
                _ => panic!("Invalid input type."),
            }
        }
        // Add buttons
        // -----------------------------------------------------------------------------------------
        controls = format!("{}<p><input type=\"submit\" value=\"Save\"></p>", controls);

        Ok(controls)
    }
}

// OUTPUT TYPES FOR THE `SAVE()` METHOD
// #################################################################################################
// Output type
pub enum OutputType {
    Hash,
    Wig,
    Json,
    Html,
}

// Output data
// ( Wig - Widgets )
#[derive(Debug)]
pub enum OutputData {
    Hash((String, bool, mongodb::bson::document::Document)),
    Wig(
        (
            std::collections::HashMap<String, Widget>,
            bool,
            mongodb::bson::document::Document,
        ),
    ),
    Json((String, bool, mongodb::bson::document::Document)),
    Html((String, bool, mongodb::bson::document::Document)),
}

impl OutputData {
    // Get Hash-line
    pub fn hash(&self) -> &str {
        match self {
            Self::Hash(data) => &data.0,
            _ => panic!("`hash()` : Doesn't match the output type."),
        }
    }
    // Get Map of Widgets
    // ( Wig - Widgets )
    pub fn wig(&self) -> std::collections::HashMap<String, Widget> {
        match self {
            Self::Wig(data) => data.0.clone(),
            _ => panic!("`wig()` : Doesn't match the output type."),
        }
    }
    // Get Json-line
    pub fn json(&self) -> &str {
        match self {
            Self::Json(data) => &data.0,
            _ => panic!("`json()` : Doesn't match the output type."),
        }
    }
    // Get Html-line
    pub fn html(&self) -> &str {
        match self {
            Self::Html(data) => &data.0,
            _ => panic!("`html()` : Doesn't match the output type."),
        }
    }
    // Get Boolean
    pub fn bool(&self) -> bool {
        match self {
            Self::Hash(data) => data.1,
            Self::Wig(data) => data.1,
            Self::Json(data) => data.1,
            Self::Html(data) => data.1,
        }
    }
    // Get Document
    pub fn doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::Hash(data) => data.2.clone(),
            Self::Wig(data) => data.2.clone(),
            Self::Json(data) => data.2.clone(),
            Self::Html(data) => data.2.clone(),
        }
    }
}
