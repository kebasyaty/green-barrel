//! # Macros
//!
//!  Custom macros.

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
                        FieldType::InputCheckBox => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Model: `{}` - FieldType `InputCheckBox` -> relation_model = blank string",
                                    STRUCT_NAME
                                )
                            }
                        }
                        _ => panic!("Model: `{}`; Field: `{}` - Non-existent field type.",
                                STRUCT_NAME, field),
                    }
                }
            }
        }
    }
}
