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
                    collection: format!("{}__{}",
                        $service.to_lowercase(),
                        stringify!($sname).to_lowercase()
                    )
                }
            }

            // Check model changes and (if required) apply to the database
            pub async fn migrat<'a>(keyword: &'a str, client: &Client) {
                static STRUCT_NAME: &'static str = stringify!($sname);
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                // Checking for the presence of fields
                if FIELD_NAMES.len() == 0 {
                    panic!("The model structure has no fields.");
                }
                // Metadata of model (database name, collection name, etc)
                let meta: Meta = Self::meta();
                // Technical database for `models::Monitor`
                let mango_orm_keyword = format!("mango_orm_{}", keyword);
                // Checking the status of Widgets
                let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
                // List of existing databases
                let database_names: Vec<String> =
                    client.list_database_names(None, None).await.unwrap();
                // Map of default values and value types from `value` attribute -
                // (String, String) -> index 0 = type ; index 1 = value
                let mut default_values: HashMap<&'static str, (&'static str, String)> = HashMap::new();

                // Checking Widgets
                // ---------------------------------------------------------------------------------
                for (field, widget) in attrs {
                    // Checking for the correct field name
                    if !FIELD_NAMES.contains(&field) {
                        panic!(
                            "Service: `{}` -> Model: `{}` -> Field: `{}` : Incorrect field name.",
                            $service, STRUCT_NAME, field
                        )
                    }
                    // Add in map default value
                    default_values.insert(field, (widget.value.get_data_type(), widget.value.get_raw_data()));
                    // Checking attribute states
                    match widget.value {
                        // InputCheckBox -----------------------------------------------------------
                        FieldType::InputCheckBox(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `maxlength` = only 0 (zero).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `other_attrs` - must not contain the word `checked`.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputCheckBox` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputColor --------------------------------------------------------------
                        FieldType::InputColor(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputColor` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputColor` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputDate ---------------------------------------------------------------
                        FieldType::InputDate(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDate` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDate` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputDateTime -----------------------------------------------------------
                        FieldType::InputDateTime(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDateTime` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputDateTime` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputEmail --------------------------------------------------------------
                        FieldType::InputEmail(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputEmail` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputEmail` : `select` = only blank vec![].",
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
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputFile` : `select` = only blank vec![].",
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
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputImage` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputNumber -------------------------------------------------------------
                        FieldType::InputNumberI32(_) | FieldType::InputNumberU32(_) | FieldType::InputNumberI64(_) | FieldType::InputNumberF64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputNumber` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputNumber` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputPassword -----------------------------------------------------------
                        FieldType::InputPassword(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.value != FieldType::InputPassword(String::new()) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `value` = only DataType::Text(String::new()).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputPassword` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputRadio --------------------------------------------------------------
                        FieldType::InputRadio(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `maxlength` = only 0 (zero).",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `other_attrs` - must not contain the word `checked`.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRadio` : `select` - must not be an empty vec![]",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputRange --------------------------------------------------------------
                        FieldType::InputRangeI32(_) | FieldType::InputRangeU32(_) | FieldType::InputRangeI64(_) | FieldType::InputRangeF64(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRange` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputRange` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputTel ----------------------------------------------------------------
                        FieldType::InputTel(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTel` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputTel` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputText ---------------------------------------------------------------
                        FieldType::InputText(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputText` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputText` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // InputUrl ----------------------------------------------------------------
                        FieldType::InputUrl(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputUrl` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `InputUrl` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // TextArea ----------------------------------------------------------------
                        FieldType::TextArea(_) => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `TextArea` : `relation_model` = only blank string.",
                                    $service, STRUCT_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `TextArea` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        // Select ------------------------------------------------------------------
                        FieldType::SelectText(_) | FieldType::SelectI32(_) | FieldType::SelectU32(_) | FieldType::SelectI64(_) | FieldType::SelectF64(_) => {
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
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ForeignKey` : `select` = only blank vec![].",
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
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `ManyToMany` : `select` = only blank vec![].",
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
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> FieldType `OneToOne` : `select` = only blank vec![].",
                                    $service, STRUCT_NAME, field
                                )
                            }
                        }
                        _ => panic!("Service: `{}` -> Model: `{}` -> Field: `{}` : `field_type` - Non-existent field type.",
                        $service, STRUCT_NAME, field),
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
                let mut run_check: bool = false;
                for item in mango_orm_fnames {
                    //
                }
                if run_check {
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
                // -----------------------------------------------------------------------------
                let db: Database = client.database(&meta.database);
                if !database_names.contains(&meta.database) ||
                    !db.list_collection_names(None).await.unwrap().contains(&meta.collection) {
                    db.create_collection(&meta.collection, None).await.unwrap();
                }

                // Update the state of models for `models::Monitor`
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
