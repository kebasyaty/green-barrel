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
    pub const SERVICE_NAME: &str = "TEST_Fq_Vs5rA2PPCEWt4";
    pub const DATABASE_NAME: &str = "TEST_bxw_5K3KvCL7HL6d";
    pub const DB_CLIENT_NAME: &str = "TEST_default_b_4XMZJU1rdTUf5k";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_5myhW5X_UAXGsHMz";

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxI64", default = 0, unique = true)]
        pub checkbox: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "radioI64", default = 1)]
        pub radio: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "numberI64")]
        pub number: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeI64", default = 5, min = 1, max = 12)]
        pub range: Option<i64>,
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
            "TEST_default_b_4XMZJU1rdTUf5k".to_string(),
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
fn test_model_with_filling_values() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        checkbox: Some(12_i64),
        radio: Some(20_i64),
        number: Some(105_i64),
        range: Some(9_i64),
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        checkbox: Some(12_i64),
        radio: Some(20_i64),
        number: Some(105_i64),
        range: Some(9_i64),
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
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        0_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        12_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        20_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result_2.wig();
    assert_eq!(
        105_i64,
        map_wigets.get("number").unwrap().value.parse::<i64>()?
    );
    // range
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        9_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::model_key()[..])
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
        assert_eq!(12_i64, doc.get_i64("checkbox")?);
        assert_eq!(20_i64, doc.get_i64("radio")?);
        assert_eq!(105_i64, doc.get_i64("number")?);
        assert_eq!(9_i64, doc.get_i64("range")?);
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
    assert_eq!(
        12_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        0_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    // radio
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        20_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        105_i64,
        map_wigets.get("number").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        9_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::model_key()[..])
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
        assert_eq!(12_i64, doc.get_i64("checkbox")?);
        assert_eq!(20_i64, doc.get_i64("radio")?);
        assert_eq!(105_i64, doc.get_i64("number")?);
        assert_eq!(9_i64, doc.get_i64("range")?);
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
