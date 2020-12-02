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
    pub const SERVICE_NAME: &str = "TEST_1_XL6P1c5yZWPchU";
    pub const DATABASE_NAME: &str = "TEST_m6UaMrJECEy3UY_K";
    pub const DB_CLIENT_NAME: &str = "TEST_default_7BNrW_XZTZFrZz5p";
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_xUkYBm2zjcZ2P_6g";

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxF64", default = 0.0, unique = true)]
        pub checkbox: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "radioF64", default = 1.0)]
        pub radio: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "numberF64")]
        pub number: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeF64", default = 5.0, min = 1.0, max = 12.0)]
        pub range: Option<f64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64",
            default = 1.0,
            select = "[[1.0, \"Volvo\"], [2.0, \"Saab\"], [3.0, \"Mercedes\"], [4.0, \"Audi\"]]"
        )]
        pub select: Option<f64>,
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
            "TEST_default_7BNrW_XZTZFrZz5p".to_string(),
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
    app_name::mango_migration()?;

    let mut test_model = app_name::TestModel {
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(OutputType::Hash)?;
    let result_2 = test_model_2.save(OutputType::Wig)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash());
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
        0_f64,
        map_wigets.get("checkbox").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("checkbox").unwrap().value.is_empty());
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
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
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    // select
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("select").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert!(map_wigets.get("select").unwrap().value.is_empty());

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
        assert_eq!(0_f64, doc.get_f64("checkbox")?);
        assert_eq!(1_f64, doc.get_f64("radio")?);
        assert_eq!(Some(()), doc.get("number").unwrap().as_null());
        assert_eq!(5_f64, doc.get_f64("range")?);
        assert_eq!(1_f64, doc.get_f64("select")?);
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(OutputType::Hash)?;
    // Validating update
    assert!(result.bool(), "{}", result.hash());
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // Validating values
    // checkbox
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("checkbox").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        0_f64,
        map_wigets.get("checkbox").unwrap().value.parse::<f64>()?
    );
    // radio
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    // number
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    // select
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert!(map_wigets.get("select").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("select").unwrap().value.parse::<f64>()?
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
        assert_eq!(0_f64, doc.get_f64("checkbox")?);
        assert_eq!(1_f64, doc.get_f64("radio")?);
        assert_eq!(Some(()), doc.get("number").unwrap().as_null());
        assert_eq!(5_f64, doc.get_f64("range")?);
        assert_eq!(1_f64, doc.get_f64("select")?);
    }

    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    Ok(())
}
