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
    pub const SERVICE_NAME: &str = "TEST_QHZsU5vJ3_R3Y7NV";
    pub const DATABASE_NAME: &str = "TEST_3js_mKPQ5CnVxLBr";
    pub const DB_CLIENT_NAME: &str = "TEST_default_W3jN_YQvQ1etzZSV";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Test keyword for for test technical database
    // Valid characters: _ a-z A-Z 0-9
    // Size: 6-52
    pub static KEYWORD: &str = "TEST_H7U_Mdv8tdJ3Xa1M";

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxI32", default = 0, unique = true)]
        pub checkbox: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "radioI32", default = -1)]
        pub radio: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "numberI32")]
        pub number: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "rangeI32", default = 5, min = 1, max = 12)]
        pub range: Option<i32>,
        #[field_attrs(widget = "hiddenI32", default = 3, min = 1, max = 12)]
        pub hidden: Option<i32>,
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
            "TEST_default_W3jN_YQvQ1etzZSV".to_string(),
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
        checkbox: Some(12),
        radio: Some(-20),
        number: Some(105),
        range: Some(9),
        hidden: Some(11),
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        checkbox: Some(12),
        radio: Some(-20),
        number: Some(105),
        range: Some(9),
        hidden: Some(11),
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
        0_i32,
        map_wigets.get("checkbox").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        12_i32,
        map_wigets.get("checkbox").unwrap().value.parse::<i32>()?
    );
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        -1_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        -20_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    // number
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result_2.wig();
    assert_eq!(
        105_i32,
        map_wigets.get("number").unwrap().value.parse::<i32>()?
    );
    // range
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        9_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    // hidden
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        11_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
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
        assert_eq!(12_i32, doc.get_i32("checkbox")?);
        assert_eq!(-20_i32, doc.get_i32("radio")?);
        assert_eq!(105_i32, doc.get_i32("number")?);
        assert_eq!(9_i32, doc.get_i32("range")?);
        assert_eq!(11_i32, doc.get_i32("hidden")?);
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
        12_i32,
        map_wigets.get("checkbox").unwrap().value.parse::<i32>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        0_i32,
        map_wigets.get("checkbox").unwrap().value.parse::<i32>()?
    );
    // radio
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        -20_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        -1_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    // number
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        105_i32,
        map_wigets.get("number").unwrap().value.parse::<i32>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        9_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    // hidden
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        11_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
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
        assert_eq!(12_i32, doc.get_i32("checkbox")?);
        assert_eq!(-20_i32, doc.get_i32("radio")?);
        assert_eq!(105_i32, doc.get_i32("number")?);
        assert_eq!(9_i32, doc.get_i32("range")?);
        assert_eq!(11_i32, doc.get_i32("hidden")?);
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
