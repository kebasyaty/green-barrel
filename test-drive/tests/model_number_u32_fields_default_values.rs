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
    pub const UNIQUE_PROJECT_KEY: &str = "FwCYN7pb6XT8ZRz";
    pub const SERVICE_NAME: &str = "TEST_FwCYN7pb6XT_8ZRz";
    pub const DATABASE_NAME: &str = "TEST_4TwTzJgL_q8HzPB8";
    pub const DB_CLIENT_NAME: &str = "TEST_default_EYzBv9swfkk3PYr_";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(widget = "radioU32", default = 1, unique = true)]
        pub radio: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "numberU32", unique = true)]
        pub number: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "rangeU32", default = 5, min = 1, max = 12, unique = true)]
        pub range: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "hiddenU32", default = 3, min = 1, max = 12, unique = true)]
        pub hidden: Option<u32>,
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
            "TEST_default_EYzBv9swfkk3PYr_".to_string(),
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
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    // number
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    // hidden
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("hidden").unwrap().value.is_empty());

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
        assert_eq!(1_i64, doc.get_i64("radio")?);
        assert_eq!(Some(()), doc.get("number").unwrap().as_null());
        assert_eq!(5_i64, doc.get_i64("range")?);
        assert_eq!(3_i64, doc.get_i64("hidden")?);
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
    // radio
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    // hidden
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("hidden").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
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
        assert_eq!(1_i64, doc.get_i64("radio")?);
        assert_eq!(Some(()), doc.get("number").unwrap().as_null());
        assert_eq!(5_i64, doc.get_i64("range")?);
        assert_eq!(3_i64, doc.get_i64("hidden")?);
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
