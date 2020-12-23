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
    pub const SERVICE_NAME: &str = "TEST_cR2qUPgThV1EMJ_U";
    pub const DATABASE_NAME: &str = "TEST_ksj_7AL6BUcuJTFk";
    pub const DB_CLIENT_NAME: &str = "TEST_default_a1gs9fWC_AX3kQ7F";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_uRjh3kVa2H7dz_pd";

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
            "TEST_default_a1gs9fWC_AX3kQ7F".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_base(KEYWORD, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            keyword: KEYWORD,
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
fn test_model_with_default_values() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None)?;
    let result_2 = test_model_2.save(None, None)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validation of `unique`
    assert!(!result_2.bool());
    // Validation of `hash`
    assert!(test_model_2.hash.is_none());
    // Validating values in widgets
    // checkbox
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("date").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("date").unwrap().value);

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::key_store()?[..])
            .unwrap();
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
            chrono::NaiveDateTime::parse_from_str("1970-02-28T00:00", "%Y-%m-%dT%H:%M")?,
            chrono::Utc,
        );
        assert_eq!(&dt_value, doc.get_datetime("date")?);
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(None, None)?;
    // Validating update
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // Validating values
    // checkbox
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("date").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::key_store()?[..])
            .unwrap();
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
            chrono::NaiveDateTime::parse_from_str("1970-02-28T00:00", "%Y-%m-%dT%H:%M")?,
            chrono::Utc,
        );
        assert_eq!(&dt_value, doc.get_datetime("date")?);
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
