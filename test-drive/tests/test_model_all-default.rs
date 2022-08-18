use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, sync::Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // =============================================================================================
    pub const PROJECT_NAME: &str = "project_name";
    // The unique key for this test.
    // To generate a key: https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    pub const UNIQUE_PROJECT_KEY: &str = "testkC1n08ZfjgS";
    //
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Models
    // =============================================================================================
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        pub checkbox: CheckBox,
        //
        pub date: InputDate,
        pub datetime: InputDateTime,
        //
        pub file: InputFile,
        pub image: InputImage,
        //
        pub radio_f64: RadioF64,
        pub number_f64: NumberF64,
        pub range_f64: RangeF64,
        pub radio_i32: RadioI32,
        pub number_i32: NumberI32,
        pub range_i32: RangeI32,
        pub radio_i64: RadioI64,
        pub number_i64: NumberI64,
        pub range_i64: RangeI64,
        pub radio_u32: RadioU32,
        pub number_u32: NumberU32,
        pub range_u32: RangeU32,
        pub radio_text: RadioText,
        //
        pub select_text: SelectText,
        pub select_text_dyn: SelectTextDyn,
        pub select_text_mult: SelectTextMult,
        pub select_text_mult_dyn: SelectTextMultDyn,
        //
        pub select_i32: SelectI32,
        pub select_i32_dyn: SelectI32Dyn,
        pub select_i32_mult: SelectI32Mult,
        pub select_i32_mult_dyn: SelectI32MultDyn,
        //
        pub select_u32: SelectU32,
        pub select_u32_dyn: SelectU32Dyn,
        pub select_u32_mult: SelectI32Mult,
        pub select_u32_mult_dyn: SelectU32MultDyn,
        //
        pub select_i64: SelectI64,
        pub select_i64_dyn: SelectI64Dyn,
        pub select_i64_mult: SelectI64Mult,
        pub select_i64_mult_dyn: SelectI64MultDyn,
        //
        pub select_f64: SelectF64,
        pub select_f64_dyn: SelectF64Dyn,
        pub select_f64_mult: SelectF64Mult,
        pub select_f64_mult_dyn: SelectF64MultDyn,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputText")]
        pub text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputSlug", slug_sources = r#"["email"]"#)]
        pub slug: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenText")]
        pub hidden_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputColor")]
        pub color: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputEmail", maxlength = 320, unique = true)]
        pub email: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputPassword")]
        pub password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputPhone")]
        pub phone: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputUrl")]
        pub url: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIP")]
        pub ip: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIPv4")]
        pub ipv4: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIPv6")]
        pub ipv6: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "textArea")]
        pub textarea: Option<String>,
    }

    // Test migration
    // =============================================================================================
    // Model list
    pub fn model_list() -> Result<Vec<Meta>, Box<dyn std::error::Error>> {
        Ok(vec![TestModel::meta()?])
    }
    // Test, migration service `Mango`
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients
        MONGODB_CLIENT_STORE.write()?.insert(
            "default".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            models: model_list()?,
        };
        monitor.migrat()?;
        // Add metadata and widgects map to cache.
        TestModel::to_cache()?;
        //
        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_all_default() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    app_name::mango_migration()?;

    // Body of test
    // =============================================================================================
    type TestModel = app_name::TestModel;
    //
    // Module: mango-orm/src/models/caching.rs
    // ---------------------------------------------------------------------------------------------
    // to_wig
    assert!(!TestModel::to_wig()?.is_empty(), "to_wig.is_empty");
    // to_json
    assert!(!TestModel::to_json()?.is_empty(), "to_json.is_empty");
    // model_to_json_for_admin
    assert!(
        !TestModel::model_to_json_for_admin()?.is_empty(),
        "model_to_json_for_admin.is_empty"
    );
    // to_html
    assert!(
        TestModel::to_html(None, None, None).is_ok(),
        "to_html.is_ok"
    );
    assert!(
        !TestModel::to_html(None, None, None)?.is_empty(),
        "to_html.is_empty"
    );
    // Get cached Model data
    let _cache_data: (ModelCache, Client) = TestModel::get_cache_data_for_query()?;
    //
    //
    // Module: mango-orm/src/models/db_query_api/commons.rs
    // ---------------------------------------------------------------------------------------------
    // aggregate
    let pipeline = vec![doc! {}];
    let result = TestModel::aggregate(pipeline, None);
    assert!(result.is_err(), "aggregate.is_err");
    // count_documents
    let result = TestModel::count_documents(None, None)?;
    assert_eq!(result, 0_i64, "count_documents == 0_i64");
    // delete_many
    let query = doc! {};
    let result = TestModel::delete_many(query, None)?;
    assert!(result.is_valid(), "delete_many.is_valid");
    assert!(result.err_msg().is_empty(), "delete_many.err_msg.is_empty");
    assert!(result.deleted_count()? == 0, "delete_many.deleted_count");
    // delete_one
    let query = doc! {};
    let result = TestModel::delete_one(query, None)?;
    assert!(result.is_valid(), "delete_one.is_valid");
    assert!(result.err_msg().is_empty(), "delete_one.err_msg.is_empty");
    assert!(result.deleted_count()? == 0, "delete_one.deleted_count");
    // distinct
    let field_name = "checkbox";
    let filter = doc! {};
    let result = TestModel::distinct(field_name, Some(filter), None)?;
    assert!(result.is_empty(), "distinct.is_empty");
    // estimated_document_count
    let result = TestModel::estimated_document_count(None)?;
    assert_eq!(result, 0_i64, "estimated_document_count == 0_i64");
    // find_many_to_doc
    let result = TestModel::find_many_to_doc(None, None)?;
    assert!(result.is_none(), "find_many_to_doc.is_none");
    // find_many_to_json
    let result = TestModel::find_many_to_json(None, None)?;
    assert!(result.is_empty());
    // find_one_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_doc(filter, None)?;
    assert!(result.is_none(), "find_many_to_json.is_none");
    // find_one_to_json
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_json(filter, None)?;
    assert!(result.is_empty(), "find_one_to_json.is_empty");
    // find_one_to_wig
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_wig(filter, None)?;
    assert!(result.is_none(), "find_one_to_wig.is_none");
    // find_one_to_model_instance
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_model_instance(filter, None)?;
    assert!(result.is_none(), "find_one_to_model_instance.is_none");
    // find_one_and_delete_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete_to_doc(filter, None)?;
    assert!(result.is_none(), "find_one_and_delete_to_doc.is_none");
    // find_one_and_delete_to_json
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete_to_json(filter, None)?;
    assert!(result.is_empty(), "find_one_and_delete_to_json.is_empty()");
    // find_one_and_delete_to_model_instance
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete_to_model_instance(filter, None)?;
    assert!(
        result.is_none(),
        "find_one_and_delete_to_model_instance.is_none"
    );
    // name
    let result = TestModel::name()?;
    assert!(!result.is_empty(), "name.is_empty");
    // namespace
    let result = TestModel::namespace()?;
    assert!(!result.db.is_empty(), "namespace.is_empty");
    assert!(!result.coll.is_empty(), "namespace.is_empty");

    // Delete test database
    // =============================================================================================
    del_test_db(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    //
    Ok(())
}
