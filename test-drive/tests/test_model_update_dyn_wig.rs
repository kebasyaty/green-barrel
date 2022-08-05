use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::bson::doc;
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
    pub const UNIQUE_PROJECT_KEY: &str = "test89Dj89Ng675";
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
        //
        //
        // Add field attributes.
        // -----------------------------------------------------------------------------------------
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextDyn", minlength = 10, maxlength = 20)]
        pub select_text_dyn_2: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextMultDyn", minlength = 10, maxlength = 20)]
        pub select_text_mult_dyn_2: Option<Vec<String>>,
        //
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32Dyn", min = 5, max = 100)]
        pub select_i32_dyn_2: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32MultDyn", min = 5, max = 100)]
        pub select_i32_mult_dyn_2: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32Dyn", min = 5, max = 100)]
        pub select_u32_dyn_2: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32MultDyn", min = 5, max = 100)]
        pub select_u32_mult_dyn_2: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64Dyn", min = 5, max = 100)]
        pub select_i64_dyn_2: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64MultDyn", min = 5, max = 100)]
        pub select_i64_mult_dyn_2: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64Dyn", min = 5.0, max = 100.0)]
        pub select_f64_dyn_2: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64MultDyn", min = 5.0, max = 100.0)]
        pub select_f64_mult_dyn_2: Option<Vec<f64>>,
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
    // Get cached Model data.
    let (model_cache, client_cache) = TestModel::get_cache_data_for_query()?;
    // Get Model metadata.
    let meta: Meta = model_cache.meta;
    // Get access to the technical base of the project.
    let coll = {
        let green_tech_keyword = format!(
            "green_tech__{}__{}",
            meta.project_name.clone(),
            meta.unique_project_key.clone()
        );
        let db = client_cache.database(&green_tech_keyword);
        db.collection("dynamic_widgets")
    };
    //
    let filter = doc! {
        "database": meta.database_name.clone(),
        "collection": meta.collection_name.clone()
    };
    //
    //
    // NEGATIVE TESTS
    // *********************************************************************************************
    //
    // Error: field name does not match.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "field_name",
        "value": 0,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "field_name, value = 0"
    );
    //
    // Error: Value type does not match widget type.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": 1,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_dyn, value = 1"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": 2,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_mult_dyn, value = 2"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": "Some text 1",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_dyn, value = 'Some text 1'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": "Some text 2",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_mult_dyn, value = 'Some text 2'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": "Some text 3",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_dyn, value = 'Some text 3'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": "Some text 4",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_mult_dyn, value = 'Some text 4'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": "Some text 5",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_dyn, value = 'Some text 5'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": "Some text 6",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_mult_dyn, value = 'Some text 6'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": "Some text 7",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_dyn, value = 'Some text 7'"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": "Some text 8",
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_mult_dyn, value = 'Some text 8'"
    );
    //
    //
    // Error: Title length is more than 150 characters.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "x".repeat(151),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "title > 150 characters"
    );
    //
    //
    // Error: minlength and maxlength
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn_2",
        "value": "x".repeat(9),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_dyn_2, value < 10 characters ; minlength = 10"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn_2",
        "value": "x".repeat(21),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_mult_dyn_2, value > 20 characters ; maxlength = 20"
    );
    //
    // Error: min and max
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn_2",
        "value": 4,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_dyn_2, value = 4 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn_2",
        "value": 101,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_mult_dyn_2, value = 101 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn_2",
        "value": 4,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_dyn_2, value = 4 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn_2",
        "value": 101,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_mult_dyn_2, value = 101 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn_2",
        "value": 4,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_dyn_2, value = 4 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn_2",
        "value": 101,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_mult_dyn_2, value = 101 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 4,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_dyn_2, value = 4 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 101,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_mult_dyn_2, value = 101 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 4.9,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_dyn_2, value = 4.9 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 100.1,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_mult_dyn_2, value = 100.1 ; max = 100"
    );
    //
    //
    // Check that if there are errors, the dynamic data is not saved.
    // ---------------------------------------------------------------------------------------------
    //
    // Get the target array from the dynamic data collection.
    let obj_fields_doc = {
        let curr_dyn_date_doc = coll.find_one(filter.clone(), None)?.unwrap();
        curr_dyn_date_doc.get_document("fields").unwrap().clone()
    };
    //
    assert!(
        obj_fields_doc.get_array("select_text_dyn").unwrap().len() == 0,
        "select_text_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_text_mult_dyn")
            .unwrap()
            .len()
            == 0,
        "select_text_mult_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i32_dyn").unwrap().len() == 0,
        "select_i32_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i32_mult_dyn")
            .unwrap()
            .len()
            == 0,
        "select_i32_mult_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_u32_dyn").unwrap().len() == 0,
        "select_u32_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_u32_mult_dyn")
            .unwrap()
            .len()
            == 0,
        "select_u32_mult_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i64_dyn").unwrap().len() == 0,
        "select_i64_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i64_mult_dyn")
            .unwrap()
            .len()
            == 0,
        "select_i64_mult_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_f64_dyn").unwrap().len() == 0,
        "select_f64_dyn ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_f64_mult_dyn")
            .unwrap()
            .len()
            == 0,
        "select_f64_mult_dyn ; len == 0"
    );
    //
    //
    assert!(
        obj_fields_doc.get_array("select_text_dyn_2").unwrap().len() == 0,
        "select_text_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_text_mult_dyn_2")
            .unwrap()
            .len()
            == 0,
        "select_text_mult_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i32_dyn_2").unwrap().len() == 0,
        "select_i32_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i32_mult_dyn_2")
            .unwrap()
            .len()
            == 0,
        "select_i32_mult_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_u32_dyn_2").unwrap().len() == 0,
        "select_u32_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_u32_mult_dyn_2")
            .unwrap()
            .len()
            == 0,
        "select_u32_mult_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i64_dyn_2").unwrap().len() == 0,
        "select_i64_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i64_mult_dyn_2")
            .unwrap()
            .len()
            == 0,
        "select_i64_mult_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_f64_dyn_2").unwrap().len() == 0,
        "select_f64_dyn_2 ; len == 0"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_f64_mult_dyn_2")
            .unwrap()
            .len()
            == 0,
        "select_f64_mult_dyn_2 ; len == 0"
    );
    //
    //
    // POSITIVE TESTS
    // *********************************************************************************************
    //
    // Title length is 150 characters.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_mult_dyn -> title == 150 characters"
    );
    //
    //
    // Test of extreme numerical values.
    // ---------------------------------------------------------------------------------------------
    //
    // I32
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": i32::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn, value = i32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn, value = i32::MAX"
    );
    //
    // U32
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn, value = u32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn, value = u32::MAX"
    );
    //
    // I64
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn, value = i64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn, value = i64::MAX"
    );
    //
    // F64
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn, value = f64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn, value = f64::MAX"
    );
    //
    //
    // Ok: minlength and maxlength
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn_2",
        "value": "x".repeat(10),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_dyn_2, value == 10 characters ; minlength = 10"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn_2",
        "value": "x".repeat(20),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_mult_dyn_2, value == 20 characters ; maxlength = 20"
    );
    //
    //
    // Ok: min and max
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn_2 -> value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn_2, value = 6 ; min = 5.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn_2, value = 99 ; max = 100.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 5.1,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn_2, value = 5.1 ; min = 5.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 99.9,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn_2, value = 99.9 ; max = 100.0"
    );
    //
    //
    // Check for saved dynamic data.
    // ---------------------------------------------------------------------------------------------
    //
    // Get the target array from the dynamic data collection.
    let obj_fields_doc = {
        let curr_dyn_date_doc = coll.find_one(filter.clone(), None)?.unwrap();
        curr_dyn_date_doc.get_document("fields").unwrap().clone()
    };
    //
    assert!(
        obj_fields_doc.get_array("select_text_dyn").unwrap().len() == 1,
        "select_text_dyn ; len == 1"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_text_mult_dyn")
            .unwrap()
            .len()
            == 1,
        "select_text_mult_dyn ; len == 1"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i32_dyn").unwrap().len() == 2,
        "select_i32_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i32_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_i32_mult_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_u32_dyn").unwrap().len() == 2,
        "select_u32_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_u32_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_u32_mult_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i64_dyn").unwrap().len() == 2,
        "select_i64_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i64_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_i64_mult_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_f64_dyn").unwrap().len() == 2,
        "select_f64_dyn ; len == 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_f64_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_f64_mult_dyn ; len == 2"
    );
    //
    //
    assert!(
        obj_fields_doc.get_array("select_text_dyn_2").unwrap().len() == 1,
        "select_text_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_text_mult_dyn_2")
            .unwrap()
            .len()
            == 1,
        "select_text_mult_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i32_dyn_2").unwrap().len() == 1,
        "select_i32_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i32_mult_dyn_2")
            .unwrap()
            .len()
            == 1,
        "select_i32_mult_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_u32_dyn_2").unwrap().len() == 1,
        "select_u32_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_u32_mult_dyn_2")
            .unwrap()
            .len()
            == 1,
        "select_u32_mult_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i64_dyn_2").unwrap().len() == 1,
        "select_i64_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i64_mult_dyn_2")
            .unwrap()
            .len()
            == 1,
        "select_i64_mult_dyn_2 ; len == 1"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_f64_dyn_2").unwrap().len() == 2,
        "select_f64_dyn_2 ; len == 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_f64_mult_dyn_2")
            .unwrap()
            .len()
            == 2,
        "select_f64_mult_dyn_2 ; len == 2"
    );
    //
    //
    // NEGATIVE TESTS
    // *********************************************************************************************
    //
    // Error when duplicating dynamic data.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn_2",
        "value": "x".repeat(10),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_dyn_2, value == 10 characters ; minlength = 10"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn_2",
        "value": "x".repeat(20),
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_mult_dyn_2, value == 20 characters ; maxlength = 20"
    );
    let dyn_data = json!({
        "field_name": "select_i32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_dyn_2 -> value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_dyn_2, value = 6 ; min = 5.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_mult_dyn_2, value = 99 ; max = 100.0"
    );
    //
    //
    // POSITIVE TESTS
    // *********************************************************************************************
    //
    // Delete dynamic data.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn_2",
        "value": "x".repeat(10),
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_dyn_2, value == 10 characters ; minlength = 10"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn_2",
        "value": "x".repeat(20),
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_text_mult_dyn_2, value == 20 characters ; maxlength = 20"
    );
    let dyn_data = json!({
        "field_name": "select_i32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_dyn_2 -> value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_u32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_i64_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_dyn_2, value = 6 ; min = 5.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_ok(),
        "select_f64_mult_dyn_2, value = 99 ; max = 100.0"
    );
    //
    //
    //
    // NEGATIVE TESTS
    // *********************************************************************************************
    //
    // Error when re-deleting already deleted dynamic data.
    // ---------------------------------------------------------------------------------------------
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn_2",
        "value": "x".repeat(10),
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_dyn_2, value == 10 characters ; minlength = 10"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn_2",
        "value": "x".repeat(20),
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_text_mult_dyn_2, value == 20 characters ; maxlength = 20"
    );
    let dyn_data = json!({
        "field_name": "select_i32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_dyn_2 -> value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_u32_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_dyn_2, value = 6 ; min = 5"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_i64_mult_dyn_2, value = 99 ; max = 100"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn_2",
        "value": 6,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_dyn_2, value = 6 ; min = 5.0"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn_2",
        "value": 99,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_wig(dyn_data).is_err(),
        "select_f64_mult_dyn_2, value = 99 ; max = 100.0"
    );
    //

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
