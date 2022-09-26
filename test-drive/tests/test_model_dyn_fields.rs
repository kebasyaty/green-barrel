use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

mod data_test {
    use super::*;

    // Test application settings
    // =============================================================================================
    pub const PROJECT_NAME: &str = "project_name";
    // The unique key for this test.
    // To generate a key: https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    pub const UNIQUE_PROJECT_KEY: &str = "testD89j89Ng675e";
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
        pub select_text_dyn: SelectTextDyn,
        pub select_text_mult_dyn: SelectTextMultDyn,
        pub select_i32_dyn: SelectI32Dyn,
        pub select_i32_mult_dyn: SelectI32MultDyn,
        pub select_u32_dyn: SelectU32Dyn,
        pub select_u32_mult_dyn: SelectU32MultDyn,
        pub select_i64_dyn: SelectI64Dyn,
        pub select_i64_mult_dyn: SelectI64MultDyn,
        pub select_f64_dyn: SelectF64Dyn,
        pub select_f64_mult_dyn: SelectF64MultDyn,
    }

    impl Control for TestModel {
        fn custom_default() -> Self {
            Self {
                select_text_dyn: SelectTextDyn::default(),
                select_text_mult_dyn: SelectTextMultDyn::default(),
                select_i32_dyn: SelectI32Dyn::default(),
                select_i32_mult_dyn: SelectI32MultDyn::default(),
                select_u32_dyn: SelectU32Dyn::default(),
                select_u32_mult_dyn: SelectU32MultDyn::default(),
                select_i64_dyn: SelectI64Dyn::default(),
                select_i64_mult_dyn: SelectI64MultDyn::default(),
                select_f64_dyn: SelectF64Dyn::default(),
                select_f64_mult_dyn: SelectF64MultDyn::default(),
                /*
                hash: HiddenHash::default(),
                created_at: HiddenDateTime::default(),
                updated_at: HiddenDateTime::default(),
                */
                ..Default::default()
            }
        }
    }

    // Test migration
    // =============================================================================================
    // Get metadata list
    pub fn get_metadata_list() -> Result<Vec<Meta>, Box<dyn Error>> {
        let metadata_list = vec![TestModel::meta()?];
        Ok(metadata_list)
    }

    // Migration
    pub fn run_migration() -> Result<(), Box<dyn Error>> {
        // Caching MongoDB clients.
        {
            let mut client_store = MONGODB_CLIENT_STORE.write()?;
            client_store.insert(
                "default".to_string(),
                mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
            );
        }

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, get_metadata_list()?)?;

        // Monitor initialization.
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            metadata_list: get_metadata_list()?,
        };
        monitor.migrat()?;

        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_dyn_fields() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    data_test::run_migration()?;

    // Testing
    // =============================================================================================
    type TestModel = data_test::TestModel;
    // Get cached Model data.
    let (model_cache, client_cache) = TestModel::get_cache_data_for_query()?;
    // Get Model metadata.
    let meta: Meta = model_cache.meta;
    // Get access to the technical base of the project.
    let _coll = {
        let green_tech_keyword = format!(
            "green_tech__{}__{}",
            meta.project_name.clone(),
            meta.unique_project_key.clone()
        );
        let db = client_cache.database(&green_tech_keyword);
        db.collection("dynamic_widgets")
    };
    //
    let _filter = doc! {
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_mult_dyn, value = 'Some text 8'"
    );
    //
    //

    // Delete test database
    // =============================================================================================
    del_test_db(
        data_test::PROJECT_NAME,
        data_test::UNIQUE_PROJECT_KEY,
        data_test::get_metadata_list()?,
    )?;

    Ok(())
}
