use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::sync::Client;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, error::Error, fs, path::Path};

mod settings {
    pub const PROJECT_NAME: &str = "test_project_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_PROJECT_KEY: &str = "d7UCc8YQ7lP595BB";
    //
    pub const SERVICE_NAME: &str = "test_service_name";
    pub const DATABASE_NAME: &str = "test_database_name";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}

mod models {
    use super::*;
    use settings::{
        DATABASE_NAME, DB_QUERY_DOCS_LIMIT, PROJECT_NAME, SERVICE_NAME, UNIQUE_PROJECT_KEY,
    };

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
                ..Default::default()
            }
        }
    }
}

mod migration {
    use super::*;

    // Get metadata list
    pub fn get_model_key_list() -> Result<Vec<String>, Box<dyn Error>> {
        let model_key_list = vec![models::TestModel::key()?];
        Ok(model_key_list)
    }

    // Migration
    pub fn run_migration(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        _validators: &HashMap<String, Regex>,
        _media_dir: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(meta_store, client)?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            settings::PROJECT_NAME,
            settings::UNIQUE_PROJECT_KEY,
            get_model_key_list()?,
            meta_store,
            client,
        )?;

        // Monitor initialization.
        let monitor = Monitor {
            project_name: settings::PROJECT_NAME,
            unique_project_key: settings::UNIQUE_PROJECT_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(meta_store, client)?;

        Ok(())
    }
}

mod app_state {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppState {
        media_root: String,
        media_url: String,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                // Root partition for storing files.
                media_root: String::from("./media"),
                // Url address to the root section.
                media_url: String::from("/media"),
            }
        }
    }

    pub fn get_app_state() -> Result<AppState, Box<dyn Error>> {
        let path = Path::new("./AppState.toml");
        if !path.is_file() {
            fs::File::create(path)?;
            let cfg = AppState::default();
            confy::store_path(path, cfg)?;
        }
        Ok(confy::load_path::<AppState>(path)?)
    }

    pub fn get_media_dir(app_state: AppState) -> HashMap<String, String> {
        [
            ("media_root".into(), app_state.media_root),
            ("media_url".into(), app_state.media_url),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_dyn_fields() -> Result<(), Box<dyn Error>> {
    // This is required for all projects.
    // =============================================================================================
    let app_state = app_state::get_app_state()?;
    let media_dir = app_state::get_media_dir(app_state);
    let meta_store = get_meta_store();
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let validators = get_validators()?;
    migration::run_migration(&meta_store, &client, &validators, &media_dir)?;

    // YOUR CODE ...
    // =============================================================================================
    type TestModel = models::TestModel;

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
        db.collection::<Document>("dynamic_fields")
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
        TestModel::update_dyn_field(dyn_data).is_err(),
        "field_name not match"
    );
    //
    // Error: Value type does not match field type.
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_text_mult_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_f64_mult_dyn, value = f64::MAX"
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
        obj_fields_doc.get_array("select_text_dyn").unwrap().len() == 2,
        "select_text_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_text_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_text_mult_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i32_dyn").unwrap().len() == 2,
        "select_i32_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i32_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_i32_mult_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_u32_dyn").unwrap().len() == 2,
        "select_u32_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_u32_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_u32_mult_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_i64_dyn").unwrap().len() == 2,
        "select_i64_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_i64_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_i64_mult_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc.get_array("select_f64_dyn").unwrap().len() == 2,
        "select_f64_dyn ; len != 2"
    );
    //
    assert!(
        obj_fields_doc
            .get_array("select_f64_mult_dyn")
            .unwrap()
            .len()
            == 2,
        "select_f64_mult_dyn ; len != 2"
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
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_text_mult_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": false,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_mult_dyn, value = f64::MAX"
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
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_text_mult_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
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
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i32_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i32_dyn, value = i32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i32_mult_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i32_mult_dyn, value = i32::MAX"
    );
    //
    // U32
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_u32_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_u32_dyn, value = u32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_u32_mult_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_u32_mult_dyn, value = u32::MAX"
    );
    //
    // I64
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i64_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i64_dyn, value = i64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i64_mult_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_i64_mult_dyn, value = i64::MAX"
    );
    //
    // F64
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_f64_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_f64_dyn, value = f64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_f64_mult_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_ok(),
        "select_f64_mult_dyn, value = f64::MAX"
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
        "field_name": "select_text_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_text_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_text_mult_dyn -> title == 150 characters"
    );
    //
    let dyn_data = json!({
        "field_name": "select_text_mult_dyn",
        "value": "Some text 2",
        "title": "x".repeat(150),
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
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
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i32_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i32_dyn, value = i32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i32_mult_dyn, value = i32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i32_mult_dyn",
        "value": i32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i32_mult_dyn, value = i32::MAX"
    );
    //
    // U32
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_u32_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_u32_dyn, value = u32::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_u32_mult_dyn, value = u32::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_u32_mult_dyn",
        "value": u32::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_u32_mult_dyn, value = u32::MAX"
    );
    //
    // I64
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i64_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i64_dyn, value = i64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i64_mult_dyn, value = i64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_i64_mult_dyn",
        "value": i64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_i64_mult_dyn, value = i64::MAX"
    );
    //
    // F64
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_dyn, value = f64::MAX"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MIN,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_mult_dyn, value = f64::MIN"
    );
    //
    let dyn_data = json!({
        "field_name": "select_f64_mult_dyn",
        "value": f64::MAX,
        "title": "Title",
        "is_delete": true,
    });
    assert!(
        TestModel::update_dyn_field(dyn_data).is_err(),
        "select_f64_mult_dyn, value = f64::MAX"
    );

    // Delete test database
    // =============================================================================================
    del_test_db(
        data_test::PROJECT_NAME,
        data_test::UNIQUE_PROJECT_KEY,
        data_test::get_metadata_list()?,
    )?;

    Ok(())
}
