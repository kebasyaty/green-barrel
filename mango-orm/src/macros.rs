//! # Macros
//!
//!  Custom macros for mango models.

// MACROS ==========================================================================================
/// Macro for converting Structure to Model
#[macro_export]
macro_rules! create_model {
    ($service:expr, $database:expr, struct $sname:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $sname {
            $(pub $fname : $ftype),*
        }

        impl $sname {
            // Get structure name
            pub fn struct_name() -> &'static str {
                stringify!($sname)
            }

            // Get array of field names
            pub fn field_names() -> &'static [&'static str] {
                &[$(stringify!($fname)),*]
            }

            // Metadata (database name, collection name, etc)
            pub fn meta() -> Meta {
                Meta {
                    database: $database.to_lowercase(),
                    collection: format!("{}_{}",
                        $service.to_lowercase(),
                        stringify!($sname).to_lowercase()
                    )
                }
            }

            // Checking Models and creating migrations to the Database.
            pub async fn migrat(client: &Client) {
                static STRUCT_NAME: &'static str = stringify!($sname);
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];

                let meta: Meta = Self::meta();
                let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
                let database_names: Vec<String> = client.list_database_names(None, None).await.unwrap();

                // Checking Widgets
                for (field, widget) in attrs {
                    // Checking for the correct field name
                    if !FIELD_NAMES.contains(&field) {
                        panic!(
                            "Service: `{}` -> Model: `{}` -> Field: `{}` : Incorrect field name.",
                            $service, STRUCT_NAME, field
                        )
                    }
                    // Checking the relationship of attribute states
                    match widget.field_type {
                        // InputCheckBox -----------------------------------------------------------
                        FieldType::InputCheckBox => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != DataType::Bool(false) || widget.value != DataType::Bool(true) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `value` = only false or true.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `maxlength` = only 0 (zero).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputColor --------------------------------------------------------------
                        FieldType::InputColor => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputColor` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputColor` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputDate ---------------------------------------------------------------
                        FieldType::InputDate => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDate` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDate` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputEmail --------------------------------------------------------------
                        FieldType::InputEmail => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputEmail` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputEmail` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputFile ---------------------------------------------------------------
                        FieldType::InputFile => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputFile` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != DataType::Text(String::new()) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputFile` : `value` = only DataType::Text(String::new()).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputFile` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputImage --------------------------------------------------------------
                        FieldType::InputImage => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputImage` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != DataType::Text(String::new()) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputImage` : `value` = only DataType::Text(String::new()).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputImage` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputNumber -------------------------------------------------------------
                        FieldType::InputNumber => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputNumber` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputPassword -----------------------------------------------------------
                        FieldType::InputPassword => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != DataType::Text(String::new()) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `value` = only DataType::Text(String::new()).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputRadio --------------------------------------------------------------
                        FieldType::InputRadio => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != DataType::Bool(false) || widget.value != DataType::Bool(true) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `value` = only false or true.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `maxlength` = only 0 (zero).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputRange --------------------------------------------------------------
                        FieldType::InputRange => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRange` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputTel ----------------------------------------------------------------
                        FieldType::InputTel => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTel` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTel` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputText ---------------------------------------------------------------
                        FieldType::InputText => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputText` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputText` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputTime ---------------------------------------------------------------
                        FieldType::InputTime => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTime` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTime` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputUrl ----------------------------------------------------------------
                        FieldType::InputUrl => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputUrl` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputUrl` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // TextArea ----------------------------------------------------------------
                        FieldType::TextArea => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `TextArea` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `TextArea` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // Select ------------------------------------------------------------------
                        FieldType::Select => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // ForeignKey --------------------------------------------------------------
                        FieldType::ForeignKey => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ForeignKey` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ForeignKey` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // ManyToMany --------------------------------------------------------------
                        FieldType::ManyToMany => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ManyToMany` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ManyToMany` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // OneToOne ----------------------------------------------------------------
                        FieldType::OneToOne => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `OneToOne` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `OneToOne` : `select` = only vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        _ => panic!("Service: `{}` -> Model: `{}` -> Field: `{}` : `field_type` - Non-existent field type.",
                        $service, STRUCT_NAME, field),
                    }
                }

                // Create a new database
                if !database_names.contains(&meta.database) {
                    println!("No database: {}", meta.database);
                }
            }
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
