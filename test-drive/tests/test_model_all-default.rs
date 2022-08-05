use mango_orm::test_tool::del_test_db;
use mango_orm::*;
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
        #[serde(default)]
        #[field_attrs(widget = "checkBox")]
        pub checkbox: Option<bool>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputDate")]
        pub date: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputDateTime")]
        pub datetime: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputFile")]
        pub file: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputImage")]
        pub image: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "radioF64")]
        pub radio_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberF64")]
        pub number_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeF64")]
        pub range_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenF64")]
        pub hidden_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "radioI32")]
        pub radio_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberI32")]
        pub number_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeI32")]
        pub range_i32: Option<i32>,
        //
        #[field_attrs(widget = "hiddenI32")]
        pub hidden_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "radioI64")]
        pub radio_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberI64")]
        pub number_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeI64")]
        pub range_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenI64")]
        pub hidden_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "radioU32")]
        pub radio_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberU32")]
        pub number_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeU32")]
        pub range_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenU32")]
        pub hidden_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "radioText")]
        pub radio_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectText")]
        pub select_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextDyn")]
        pub select_text_dyn: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextMult")]
        pub select_text_mult: Option<Vec<String>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextMultDyn")]
        pub select_text_mult_dyn: Option<Vec<String>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32")]
        pub select_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32Dyn")]
        pub select_i32_dyn: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32Mult")]
        pub select_i32_mult: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32MultDyn")]
        pub select_i32_mult_dyn: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32")]
        pub select_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32Dyn")]
        pub select_u32_dyn: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32Mult")]
        pub select_u32_mult: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32MultDyn")]
        pub select_u32_mult_dyn: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64")]
        pub select_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64Dyn")]
        pub select_i64_dyn: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64Mult")]
        pub select_i64_mult: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64MultDyn")]
        pub select_i64_mult_dyn: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64")]
        pub select_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64Dyn")]
        pub select_f64_dyn: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64Mult")]
        pub select_f64_mult: Option<Vec<f64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64MultDyn")]
        pub select_f64_mult_dyn: Option<Vec<f64>>,
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
