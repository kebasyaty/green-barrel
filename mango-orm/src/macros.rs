//! # Macros
//!
//!  Custom macros.

/// Macro for converting Structure to Model
#[macro_export]
macro_rules! create_model {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $name {
            $(pub $fname : $ftype),*
        }

        impl $name {
            fn struct_name() -> &'static str {
                static NAME: &'static str  = stringify!($name);
                NAME
            }

            fn field_names() -> &'static [&'static str] {
                static NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                NAMES
            }
        }
    }
}
