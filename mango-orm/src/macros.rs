//! # Macros
//!
//!  Custom macros.

/// Macro for converting Structure to Model
#[macro_export]
macro_rules! model_info {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $name {
            $(pub $fname : $ftype),*
        }

        impl $name {
            pub fn struct_name() -> &'static str {
                static NAME: &'static str  = stringify!($name);
                NAME
            }

            pub fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }

            // Checking Models and creating migrations to the Database.
            pub async fn migrat(_client: Client) {
                let _meta: Meta = Self::meta();
                let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
                static STRUCT_NAME: &'static str  = stringify!($name);
                // Checking Widgets
                for (_field, widget) in attrs {
                    match widget.field_type {
                        FieldType::InputCheckBox => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "{} FieldType `InputCheckBox` -> relation_model = blank string",
                                    STRUCT_NAME
                                )
                            }
                        }
                        _ => panic!("{} - Non-existent field type.", STRUCT_NAME),
                    }
                }
            }
        }
    }
}
