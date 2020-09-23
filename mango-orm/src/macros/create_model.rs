//! # Create Model
//!
//!  `create_model` - Macro for converting Structure to Model.

// MACRO ==========================================================================================
/// Macro for converting Structure to Model
#[macro_export]
macro_rules! create_model {
    ($service:expr, $database:expr, struct $sname:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $sname {
            $(pub $fname : $ftype),*
        }

        impl $sname {
            // Get model name
            pub fn model_name() -> &'static str {
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
                    collection: format!("{}__{}",
                        $service.to_lowercase(),
                        stringify!($sname).to_lowercase()
                    )
                }
            }

            // Get a map of pure attributes for page templates
            pub fn form_attrs() -> HashMap<String, Transport> {
                let raw_attrs: HashMap<&str, Widget> = Self::widgets();
                let mut clean_attrs: HashMap<String, Transport> = HashMap::new();
                for (field, widget) in &raw_attrs {
                    clean_attrs.insert(field.to_string(), widget.clean_attrs(field));
                }
                clean_attrs
            }

            // Get form attributes Json format for page templates
            pub fn json_attrs() -> String {
                let attrs: HashMap<String, Transport> = Self::form_attrs();
                let mut json_text = String::new();
                for (field, trans) in attrs {
                    let tmp = serde_json::to_string(&trans).unwrap(); // Transport
                    if json_text.len() > 0 {
                        json_text = format!("{},\"{}\":{}", json_text, field, tmp);
                    } else {
                        json_text = format!("\"{}\":{}", field, tmp);
                    }
                }
                format!("{{{}}}", json_text)
            }

            // Get Html form for page templates
            pub fn form_html(action: &str, method: Option<&str>, enctype: Option<&str>) -> String {
                Self::html(
                    Self::form_attrs(),
                    &stringify!($sname).to_lowercase(),
                    action,
                    if method.is_some() { method.unwrap().to_lowercase() } else { "get".to_string() },
                    if enctype.is_some() { enctype.unwrap() } else { "application/x-www-form-urlencoded" }
                )
            }

            // Check model changes and (if required) apply to the database
            pub async fn migrat<'a>(keyword: &'a str, client: &Client) {
                static MODEL_NAME: &'static str = stringify!($sname);
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                // Checking for the presence of fields
                if FIELD_NAMES.len() == 0 {
                    panic!("The model structure has no fields.");
                }
                // Create a map with field types
                let map_field_types: HashMap<&'static str, &'static str> =
                FIELD_NAMES.iter().map(|item| item.to_owned())
                .zip([$(stringify!($ftype)),*].iter().map(|item| item.to_owned())).collect();
                // Metadata of model (database name, collection name, etc)
                let meta: Meta = Self::meta();
                // Technical database for `models::Monitor`
                let mango_orm_keyword = format!("mango_orm_{}", keyword);
                // Checking the status of Widgets
                let attrs: HashMap<&'static str, Widget> = Self::widgets();
                // List of existing databases
                let database_names: Vec<String> =
                    client.list_database_names(None, None).await.unwrap();
                // Map of default values and value types from `value` attribute -
                // (String, String) -> index 0 = type ; index 1 = value
                let mut default_values: HashMap<&'static str, (&'static str, String)> = HashMap::new();

                // Checking Widgets
                // ---------------------------------------------------------------------------------
                // Looping over fields and attributes
                for (field, widget) in attrs {
                    // Checking for the correct field name
                    if !FIELD_NAMES.contains(&field) {
                        panic!(
                            "Service: `{}` -> Model: `{}` -> raw_attrs() : `{}` - Incorrect field name.",
                            $service, MODEL_NAME, field
                        )
                    }
                    // Add in map default value
                    default_values.insert(field, (widget.value.get_data_type(), widget.value.get_raw_data()));
                    // Checking attribute states
                    match widget.value {
                        // InputCheckBoxText -------------------------------------------------------
                        // InputCheckBoxI32
                        // InputCheckBoxU32
                        // InputCheckBoxI64
                        // InputCheckBoxF64
                        FieldType::InputCheckBoxText(_) | FieldType::InputCheckBoxI32(_) | FieldType::InputCheckBoxU32(_) | FieldType::InputCheckBoxI64(_) | FieldType::InputCheckBoxF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            match widget.value {
                                FieldType::InputCheckBoxText(_) => {
                                    enum_field_type = "InputCheckBoxText".to_string();
                                    data_field_type = "string".to_string();
                                }
                                FieldType::InputCheckBoxI32(_) => {
                                    enum_field_type = "InputCheckBoxI32".to_string();
                                    data_field_type = "i32".to_string();
                                }
                                FieldType::InputCheckBoxU32(_) => {
                                    enum_field_type = "InputCheckBoxU32".to_string();
                                    data_field_type = "u32".to_string();
                                }
                                FieldType::InputCheckBoxI64(_) => {
                                    enum_field_type = "InputCheckBoxI64".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::InputCheckBoxF64(_) => {
                                    enum_field_type = "InputCheckBoxF64".to_string();
                                    data_field_type = "f64".to_string();
                                }
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `maxlength` = only 0 (zero).",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `other_attrs` - must not contain the word `checked`.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }
                        }
                        // InputColor --------------------------------------------------------------
                        // InputDate
                        // InputDateTime
                        // InputEmail
                        FieldType::InputColor(_) | FieldType::InputDate(_) | FieldType::InputDateTime(_) | FieldType::InputEmail(_) => {
                            let mut enum_field_type = String::new();
                            match widget.value {
                                FieldType::InputColor(_) => { enum_field_type = "InputColor".to_string(); }
                                FieldType::InputDate(_) => { enum_field_type = "InputDate".to_string(); }
                                FieldType::InputDateTime(_) => { enum_field_type = "InputDateTime".to_string(); }
                                FieldType::InputEmail(_) => { enum_field_type = "InputEmail".to_string(); }
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "string" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputFile ---------------------------------------------------------------
                        // InputImage
                        FieldType::InputFile | FieldType::InputImage => {
                            let mut enum_field_type = String::new();
                            match widget.value {
                                FieldType::InputFile => { enum_field_type = "InputFile".to_string(); }
                                FieldType::InputImage => { enum_field_type = "InputImage".to_string(); }
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputNumber - i32 -------------------------------------------------------
                        FieldType::InputNumberI32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputNumber - u32 -------------------------------------------------------
                        FieldType::InputNumberU32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "u32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `u32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputNumber - i64 -------------------------------------------------------
                        FieldType::InputNumberI64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputNumber - f64 -------------------------------------------------------
                        FieldType::InputNumberF64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputNumber` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "f64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `f64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputPassword -----------------------------------------------------------
                        FieldType::InputPassword(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputPassword` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.value != FieldType::InputPassword(String::new()) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputPassword` : `value` = only DataType::Text(String::new()).",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputPassword` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputRadio --------------------------------------------------------------
                        FieldType::InputRadio(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRadio` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRadio` : `maxlength` = only 0 (zero).",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRadio` : `other_attrs` - must not contain the word `checked`.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRadio` : `select` - must not be an empty vec![]",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "bool" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `bool`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputRange - i32 --------------------------------------------------------
                        FieldType::InputRangeI32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputRange - u32 --------------------------------------------------------
                        FieldType::InputRangeU32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "u32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `u32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputRange - i64 --------------------------------------------------------
                        FieldType::InputRangeI64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputRange f64 ----------------------------------------------------------
                        FieldType::InputRangeF64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputRange` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "f64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `f64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputTel ----------------------------------------------------------------
                        FieldType::InputTel(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputTel` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputTel` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputText ---------------------------------------------------------------
                        FieldType::InputText(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputText` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputText` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // InputUrl ----------------------------------------------------------------
                        FieldType::InputUrl(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputUrl` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `InputUrl` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // TextArea ----------------------------------------------------------------
                        FieldType::TextArea(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `TextArea` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `TextArea` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // Select - Text -----------------------------------------------------------
                        FieldType::SelectText(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // Select - i32 ------------------------------------------------------------
                        FieldType::SelectI32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // Select - u32 ------------------------------------------------------------
                        FieldType::SelectU32(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "u32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `u32`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // Select - i64 ------------------------------------------------------------
                        FieldType::SelectI64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "i64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `i64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // Select - f64 ------------------------------------------------------------
                        FieldType::SelectF64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `Select` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "f64" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `f64`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // ForeignKey --------------------------------------------------------------
                        FieldType::ForeignKey => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `ForeignKey` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `ForeignKey` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // ManyToMany --------------------------------------------------------------
                        FieldType::ManyToMany => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `ManyToMany` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `ManyToMany` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        // OneToOne ----------------------------------------------------------------
                        FieldType::OneToOne => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `OneToOne` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> FieldType `OneToOne` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        _ => panic!("Service: `{}` -> Model: `{}` -> Field: `{}` : `field_type` - Non-existent field type.",
                        $service, MODEL_NAME, field),
                    }
                }

                // Check the field changes in the Model and (if required)
                // update documents in the current Collection
                // ---------------------------------------------------------------------------------
                // Get a list of current model field names from the technical database `mango_orm_keyword`
                let mango_orm_fnames: Vec<String> = {
                    let filter: Document = doc! {
                        "database": &meta.database, "collection": &meta.collection};
                    let model: Document = client.database(&mango_orm_keyword)
                        .collection("models").find_one(filter, None).await.unwrap().unwrap();
                    let fields: Vec<Bson> = model.get_array("fields").unwrap().to_vec();
                    fields.into_iter().map(|item: Bson| item.as_str().unwrap().to_string()).collect()
                };
                // Check if the set of fields in the collection of the current Model needs to be updated
                let mut run_document_modification: bool = false;
                if FIELD_NAMES.len() != mango_orm_fnames.len() {
                    run_document_modification = true;
                } else {
                    for item in FIELD_NAMES {
                        if mango_orm_fnames.iter().any(|item2| item2 != item) {
                            run_document_modification = true;
                            break;
                        }
                    }
                }
                // Start (if necessary) updating the set of fields in the current collection
                if run_document_modification {
                    // Get the database and collection of the current Model
                    let db: Database = client.database(&meta.database);
                    let collection: Collection = db.collection(&meta.collection);
                    // Get cursor to all documents of the current Model
                    let mut cursor: Cursor = collection.find(None, None).await.unwrap();
                    // Iterate through all documents in a current (model) collection
                    while let Some(result) = cursor.next().await {
                        let curr_doc: Document = result.unwrap();
                        // Create temporary blank document
                        let mut tmp_doc = doc! {};
                        // Loop over all fields of the model
                        for field in FIELD_NAMES {
                            // If the field exists, get its value
                            if curr_doc.contains_key(field) {
                                for item in curr_doc.iter() {
                                    if item.0 == field {
                                        tmp_doc.insert(field.to_string(), item.1);
                                        break;
                                    }
                                }
                            } else {
                                // If no field exists, get default value
                                let value = &default_values[field];
                                tmp_doc.insert(field.to_string(), match value.0 {
                                    "string" => Bson::String(value.1.clone()),
                                    "i32" => Bson::Int32(value.1.parse::<i32>().unwrap()),
                                    "u32" => Bson::Int64(value.1.parse::<i64>().unwrap()),
                                    "i64" => Bson::Int64(value.1.parse::<i64>().unwrap()),
                                    "f64" => Bson::Double(value.1.parse::<f64>().unwrap()),
                                    "bool" => Bson::Boolean(value.1.parse::<bool>().unwrap()),
                                    "none" => Bson::Null,
                                    _ => panic!("Invalid data type."),
                                });
                            }
                        }
                        // Save updated document
                        let query = doc! {"_id": curr_doc.get_object_id("_id").unwrap()};
                        let update = UpdateModifications::Document(tmp_doc);
                        collection.update_one(query, update, None).await.unwrap();
                    }
                }

                // Create a new database (if doesn't exist) and add new collection
                // ---------------------------------------------------------------------------------
                // Get the database for the current collection of Model
                let db: Database = client.database(&meta.database);
                // If there is no collection for the current Model, create it
                if !database_names.contains(&meta.database) ||
                    !db.list_collection_names(None).await.unwrap().contains(&meta.collection) {
                    db.create_collection(&meta.collection, None).await.unwrap();
                }

                // Update the state of models for `models::Monitor`
                // ---------------------------------------------------------------------------------
                // Get the technical database `mango_orm_keyword` for the current model
                let db: Database = client.database(&mango_orm_keyword);
                // Check if there is a technical database of the project, if not, causes panic
                if !database_names.contains(&mango_orm_keyword) ||
                    !db.list_collection_names(None).await.unwrap().contains(&"models".to_owned()) {
                    panic!("For migration not used `models::Monitor.refresh()`.");
                } else {
                    let collection = db.collection("models");
                    let filter = doc! {"database": &meta.database, "collection": &meta.collection};
                    let doc = doc!{
                        "database": &meta.database,
                        "collection": &meta.collection,
                        "fields": FIELD_NAMES,
                        "status": true
                    };
                    // Check if there is model state in the database
                    if collection.count_documents(filter.clone(), None).await.unwrap() == 0_i64 {
                        // Add model state information
                        collection.insert_one(doc, None).await.unwrap();
                    } else {
                        // Update model state information
                        let update = UpdateModifications::Document(doc);
                        collection.update_one(filter, update, None).await.unwrap();
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
