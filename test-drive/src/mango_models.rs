use async_trait::async_trait;
use futures::stream::StreamExt;
use mango_orm::{
    create_model,
    models::{Meta, Model},
    widgets::{FieldType, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, Bson},
    options::UpdateModifications,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

const SERVICE_NAME: &'static str = "account"; // SERVICE_NAME or APP_NAME or PROJECT_NAME
const DATABASE_NAME: &'static str = "test_drive";

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    struct Category {
        title: String
    }
}
#[async_trait]
impl Model for Category {
    // Example:
    // Define attributes for widgets of fields
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "title",
            Widget {
                label: "Category Name".to_string(),
                value: FieldType::InputText(String::new()),
                maxlength: 40,
                hint: "Please enter Category name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Category Name"),
                ..Default::default()
            },
        );
        raw_attrs
    }
    // Example:
    // Define (If necessary) HTML form for page templates
    fn form() -> String {
        let attrs: HashMap<String, Transport> = Self::form_attrs();
        let mut form_text = String::from("<form action=\"/\" method=\"GET\">");
        for (_, trans) in attrs {
            match trans.field_type.as_str() {
                "checkbox" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class={} {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.checked { "checked" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "radio" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class={} {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.checked { "checked" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "color" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "date" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "datetime" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "email" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "file" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "image" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "number" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "password" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "range" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "tel" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "text" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "url" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<input id=\"{}\" type=\"{}\" name=\"{}\" value=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>",
                        form_text,
                        label,
                        trans.id,
                        trans.field_type,
                        trans.name,
                        trans.value,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs
                    );
                }
                "textarea" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    form_text = format!(
                        "{}\n{}\n<textarea id=\"{}\" name=\"{}\" maxlength=\"{}\" {} class=\"{}\" {}>\n{}\n</textarea>",
                        form_text,
                        label,
                        trans.id,
                        trans.name,
                        trans.maxlength,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs,
                        trans.value,
                    );
                }
                "select" => {
                    let label = format!("<label for=\"{}\">{}:</label>", trans.id, trans.label);
                    let mut options = String::new();
                    for item in trans.select {
                        options = format!(
                            "{}\n<option value=\"{}\">{}</option>",
                            options, item.1, item.0
                        );
                    }
                    form_text = format!(
                        "{}\n{}\n<select id=\"{}\" name=\"{}\" {} class=\"{}\" {}>\n{}\n</select>",
                        form_text,
                        label,
                        trans.id,
                        trans.name,
                        if trans.required { "required" } else { "" },
                        trans.some_classes,
                        trans.other_attrs,
                        options,
                    );
                }
                _ => panic!("Invalid input type."),
            }
        }
        format!("{}\n</form>", form_text)
    }
}

create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    struct User {
        username: String,
        email: String
    }
}
#[async_trait]
impl Model for User {
    // Example:
    // Define attributes for widgets of fields
    fn raw_attrs() -> HashMap<&'static str, Widget> {
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your Name".to_string(),
                value: FieldType::InputText("Rust".to_string()),
                maxlength: 40,
                hint: "Please enter your real name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Your Name"),
                ..Default::default()
            },
        );
        raw_attrs.insert(
            "email",
            Widget {
                label: "Your Email".to_string(),
                value: FieldType::InputEmail(String::new()),
                maxlength: 78,
                hint: "Enter your work email.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                ..Default::default()
            },
        );
        raw_attrs
    }
}
