use mango_orm::*;
use mango_orm::{migration::Monitor, test_tool::del_test_base};
use metamorphose::Model;
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Client,
};
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const PROJECT_NAME: &str = "project_name";
    pub const UNIQUE_PROJECT_KEY: &str = "37xtBe3LL8x2Fkf";
    pub const SERVICE_NAME: &str = "TEST_3_7xtBe3LL8x2Fkf";
    pub const DATABASE_NAME: &str = "TEST_mQE_3Tx8aNU7WHfN";
    pub const DB_CLIENT_NAME: &str = "TEST_default_uSH2VyyYNqAn_X9v";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(
            widget = "inputDateTime",
            default = "1970-02-28T00:00",
            min = "1970-01-01T00:00",
            max = "1970-03-01T00:00",
            unique = true
        )]
        pub date: Option<String>,
    }

    // Test migration
    // *********************************************************************************************
    // Model list
    pub fn model_list() -> Result<Vec<Meta>, Box<dyn std::error::Error>> {
        Ok(vec![TestModel::meta()?])
    }
    // Test, migration service `Mango`
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients
        DB_MAP_CLIENT_NAMES.lock()?.insert(
            "TEST_default_uSH2VyyYNqAn_X9v".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_base(PROJECT_NAME, UNIQUE_PROJECT_KEY, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            models: model_list()?,
        };
        monitor.migrat();
        //
        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_with_filling_values() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        date: Some("1970-02-27T00:00".to_string()),
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        date: Some("1970-02-27T00:00".to_string()),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None, None)?;
    let result_2 = test_model_2.save(None, None, None)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validation of `unique`
    assert!(!result_2.bool());
    // Validation of `hash`
    assert!(test_model_2.hash.is_none());
    // Validating values in widgets
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "1970-02-27T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store.get(&app_name::TestModel::key()[..]).unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        let doc = coll.find_one(filter, None)?.unwrap();
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let dt_value: chrono::DateTime<chrono::Utc> = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::parse_from_str("1970-02-27T00:00", "%Y-%m-%dT%H:%M")?,
            chrono::Utc,
        );
        assert_eq!(&dt_value, doc.get_datetime("date")?);
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(None, None, None)?;
    // Validating update
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // Validating values
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store.get(&app_name::TestModel::key()[..]).unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        let doc = coll.find_one(filter, None)?.unwrap();
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let dt_value: chrono::DateTime<chrono::Utc> = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::parse_from_str("1970-02-27T00:00", "%Y-%m-%dT%H:%M")?,
            chrono::Utc,
        );
        assert_eq!(&dt_value, doc.get_datetime("date")?);
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
