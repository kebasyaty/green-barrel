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
            pub async fn migrat(_client: &Client) {
                let _meta: Meta = Self::meta();
                let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
                static STRUCT_NAME: &'static str = stringify!($sname);
                // Checking Widgets
                for (field, widget) in attrs {
                    match widget.field_type {
                        // InputCheckBox -----------------------------------------------------------
                        FieldType::InputCheckBox => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Model: `{}` - FieldType `InputCheckBox` -> `relation_model` = only blank string",
                                    STRUCT_NAME
                                )
                            } else if widget.value.get_data() != "true" || widget.value.get_data() != "false" {
                                panic!(
                                    "Model: `{}` - FieldType `InputCheckBox` -> `value` = only true or false",
                                    STRUCT_NAME
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Model: `{}` - FieldType `InputCheckBox` -> `maxlength` = only 0 (zero)",
                                    STRUCT_NAME
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Model: `{}` - FieldType `InputCheckBox` -> `select` = only vec![]",
                                    STRUCT_NAME
                                )
                            }
                        }
                        // InputColor --------------------------------------------------------------
                        FieldType::InputColor => {}
                        // InputDate ---------------------------------------------------------------
                        FieldType::InputDate => {}
                        // InputEmail --------------------------------------------------------------
                        FieldType::InputEmail => {}
                        // InputFile ---------------------------------------------------------------
                        FieldType::InputFile => {}
                        // InputImage --------------------------------------------------------------
                        FieldType::InputImage => {}
                        // InputNumber -------------------------------------------------------------
                        FieldType::InputNumber => {}
                        // InputPassword -----------------------------------------------------------
                        FieldType::InputPassword => {}
                        // InputRadio --------------------------------------------------------------
                        FieldType::InputRadio => {}
                        // InputRange --------------------------------------------------------------
                        FieldType::InputRange => {}
                        // InputTel ----------------------------------------------------------------
                        FieldType::InputTel => {}
                        // InputText ---------------------------------------------------------------
                        FieldType::InputText => {}
                        // InputTime ---------------------------------------------------------------
                        FieldType::InputTime => {}
                        // InputUrl ----------------------------------------------------------------
                        FieldType::InputUrl => {}
                        // TextArea ----------------------------------------------------------------
                        FieldType::TextArea => {}
                        // Select ------------------------------------------------------------------
                        FieldType::Select => {}
                        // ForeignKey --------------------------------------------------------------
                        FieldType::ForeignKey => {}
                        // ManyToMany --------------------------------------------------------------
                        FieldType::ManyToMany => {}
                        // OneToOne ----------------------------------------------------------------
                        FieldType::OneToOne => {}
                        _ => panic!("Model: `{}`; Field: `{}` - Non-existent field type.",
                                STRUCT_NAME, field),
                    }
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
