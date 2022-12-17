use green_barrel::*;
use metamorphose::Model;
use mongodb::{
    bson::{doc, Document},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

mod settings {
    // Project name.
    // Valid characters: _ a-z A-Z 0-9
    // Hint: PROJECT_NAM it is recommended not to change.
    // Max size: 20
    // First character: a-z A-Z
    pub const APP_NAME: &str = "test_app_name";
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 20
    // First character: a-z A-Z
    pub const DATABASE_NAME: &str = "test_app_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_APP_KEY: &str = "d7UCc8YQ7lP595BB";
    //
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 30
    // First character: a-z A-Z
    pub const SERVICE_NAME: &str = "test_service_name";
}

mod models {
    use super::*;
    use settings::{APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME, UNIQUE_APP_KEY};

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
    pub async fn run_migration(client: &Client) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(client).await?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            client,
            settings::APP_NAME,
            settings::UNIQUE_APP_KEY,
            get_model_key_list()?,
        )
        .await?;

        // Monitor initialization.
        let monitor = Monitor {
            app_name: settings::APP_NAME,
            unique_app_key: settings::UNIQUE_APP_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(client).await?;

        Ok(())
    }
}

// TEST
// #################################################################################################
#[tokio::test]
async fn test_model_dyn_fields() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // =============================================================================================
    type TestModel = models::TestModel;

    // Get a key to access the metadata store.
    let key = TestModel::key()?;
    // Get metadata store.
    let store = { META_STORE.lock().await.clone() };
    // Get metadata of Model.
    let meta = if let Some(meta) = store.get(&key) {
        meta
    } else {
        Err(format!(
            "Model key: `{key}` ; Method: `run_fixture()` => \
            Failed to get data from cache.",
        ))?
    };
    // Get access to the technical base of the project.
    let coll = {
        let green_tech_keyword = format!(
            "green_tech__{}__{}",
            meta.app_name.clone(),
            meta.unique_app_key.clone()
        );
        let db = client.database(&green_tech_keyword);
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
        "select_f64_mult_dyn, value = 'Some text 8'"
    );
    //
    //
    // Check that if there are errors, the dynamic data is not saved.
    // ---------------------------------------------------------------------------------------------
    //
    // Get the target array from the dynamic data collection.
    let obj_fields_doc = {
        let curr_dyn_date_doc = coll.find_one(filter.clone(), None).await?.unwrap();
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
        "select_f64_mult_dyn, value = f64::MAX"
    );
    //
    //
    // Check for saved dynamic data.
    // ---------------------------------------------------------------------------------------------
    //
    // Get the target array from the dynamic data collection.
    let obj_fields_doc = {
        let curr_dyn_date_doc = coll.find_one(filter.clone(), None).await?.unwrap();
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data).await.is_ok(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
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
        TestModel::update_dyn_field(&client, dyn_data)
            .await
            .is_err(),
        "select_f64_mult_dyn, value = f64::MAX"
    );

    // Delete test database
    // =============================================================================================
    del_test_db(
        &client,
        settings::APP_NAME,
        settings::UNIQUE_APP_KEY,
        migration::get_model_key_list()?,
    )
    .await?;

    Ok(())
}
