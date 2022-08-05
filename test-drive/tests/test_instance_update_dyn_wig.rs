use mango_orm::test_tool::del_test_db;
use mango_orm::*;
use metamorphose::Model;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    pub const UNIQUE_PROJECT_KEY: &str = "test16A7l0dsw8x";
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
        #[field_attrs(widget = "selectTextDyn")]
        pub select_text_dyn: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextMultDyn")]
        pub select_text_mult_dyn: Option<Vec<String>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32Dyn")]
        pub select_i32_dyn: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32MultDyn")]
        pub select_i32_mult_dyn: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32Dyn")]
        pub select_u32_dyn: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32MultDyn")]
        pub select_u32_mult_dyn: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64Dyn")]
        pub select_i64_dyn: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64MultDyn")]
        pub select_i64_mult_dyn: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64Dyn")]
        pub select_f64_dyn: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64MultDyn")]
        pub select_f64_mult_dyn: Option<Vec<f64>>,
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
fn test_model_update_dyn_wig() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    app_name::mango_migration()?;

    // Body of test
    // =============================================================================================
    //
    type TestModel = app_name::TestModel;
    //
    // Module: mango-orm/src/models/caching.rs
    //
    //
    // POSITIVE TESTS
    // *********************************************************************************************
    //
    // Text
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_dyn - value = 'Some text' ; title = 'Title' ; is_delete = false"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_mult_dyn - value = 'Some text' ; title = 'Title' ; is_delete = false"
    );
    //
    // I32
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    // U32
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    // I64
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn - value = 100 ; title = 'Title' ; is_delete = false"
    );
    //
    // F64
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": 100.0,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn - value = 100.0 ; title = 'Title' ; is_delete = false"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": 100.0,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn - value = 100.0 ; title = 'Title' ; is_delete = false"
    );
    //
    //
    // ---------------------------------------------------------------------------------------------
    //
    let mut test_model = TestModel {
        select_text_dyn: Some("Some text".to_string()),
        select_text_mult_dyn: Some(vec!["Some text".to_string()]),
        select_i32_dyn: Some(100),
        select_i32_mult_dyn: Some(vec![100]),
        select_u32_dyn: Some(100),
        select_u32_mult_dyn: Some(vec![100]),
        select_i64_dyn: Some(100),
        select_i64_mult_dyn: Some(vec![100]),
        select_f64_dyn: Some(100.0),
        select_f64_mult_dyn: Some(vec![100.0]),
        ..Default::default()
    };
    //
    //
    // ---------------------------------------------------------------------------------------------
    //
    let _output_data = test_model.save(None, None)?;
    //
    let hash = test_model.get_hash();
    let object_id = ObjectId::with_string(hash.as_str())?;
    let filter = doc! {"_id": object_id};
    let instance = TestModel::find_one_to_model_instance(filter.clone(), None)?;
    //
    assert!(instance.is_some(), "instance - is_some 1");
    let instance = instance.unwrap();
    //
    assert_eq!(
        instance.select_text_dyn,
        Some("Some text".to_string()),
        r#"instance.select_text_dyn == Some("Some text".to_string())"#
    );
    assert_eq!(
        instance.select_text_mult_dyn,
        Some(vec!["Some text".to_string()]),
        r#"instance.select_text_mult_dyn == Some(vec!["Some text".to_string()])"#
    );
    assert_eq!(
        instance.select_i32_dyn,
        Some(100),
        r#"instance.select_i32_dyn == Some(100)"#
    );
    assert_eq!(
        instance.select_i32_mult_dyn,
        Some(vec![100]),
        r#"instance.select_i32_mult_dyn == Some(vec![100])"#
    );
    assert_eq!(
        instance.select_u32_dyn,
        Some(100),
        r#"instance.select_u32_dyn == Some(100)"#
    );
    assert_eq!(
        instance.select_u32_mult_dyn,
        Some(vec![100]),
        r#"instance.select_u32_mult_dyn == Some(vec![100])"#
    );
    assert_eq!(
        instance.select_i64_dyn,
        Some(100),
        r#"instance.select_i64_dyn == Some(100)"#
    );
    assert_eq!(
        instance.select_i64_mult_dyn,
        Some(vec![100]),
        r#"instance.select_i64_mult_dyn == Some(vec![100])"#
    );
    assert_eq!(
        instance.select_f64_dyn,
        Some(100.0),
        r#"instance.select_f64_dyn == Some(100.0)"#
    );
    assert_eq!(
        instance.select_f64_mult_dyn,
        Some(vec![100.0]),
        r#"instance.select_f64_mult_dyn == Some(vec![100.0])"#
    );
    //
    //
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_dyn - value = 'Some text' ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text",
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_mult_dyn - value = 'Some text' ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": 100,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn - value = 100 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": 100.0,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn - value = 100.0 ; title = 'Title' ; is_delete = true"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": 100.0,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn - value = 100.0 ; title = 'Title' ; is_delete = true"
    );
    //
    //
    let instance = TestModel::find_one_to_model_instance(filter, None)?;
    assert!(instance.is_some(), "instance - is_some 2");
    let instance = instance.unwrap();
    //
    assert_eq!(
        instance.select_text_dyn, None,
        r#"instance.select_text_dyn == None"#
    );
    assert_eq!(
        instance.select_text_mult_dyn, None,
        r#"instance.select_text_mult_dyn == None"#
    );
    assert_eq!(
        instance.select_i32_dyn, None,
        r#"instance.select_i32_dyn == None"#
    );
    assert_eq!(
        instance.select_i32_mult_dyn, None,
        r#"instance.select_i32_mult_dyn == None"#
    );
    assert_eq!(
        instance.select_u32_dyn, None,
        r#"instance.select_u32_dyn == None"#
    );
    assert_eq!(
        instance.select_u32_mult_dyn, None,
        r#"instance.select_u32_mult_dyn == None"#
    );
    assert_eq!(
        instance.select_i64_dyn, None,
        r#"instance.select_i64_dyn == None"#
    );
    assert_eq!(
        instance.select_i64_mult_dyn, None,
        r#"instance.select_i64_mult_dyn == None"#
    );
    assert_eq!(
        instance.select_f64_dyn, None,
        r#"instance.select_f64_dyn == None"#
    );
    assert_eq!(
        instance.select_f64_mult_dyn, None,
        r#"instance.select_f64_mult_dyn == None"#
    );

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
