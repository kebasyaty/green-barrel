use mango_orm::*;
use mango_orm::{migration::Monitor, test_tool::del_test_db};
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
    pub const UNIQUE_PROJECT_KEY: &str = "wurdhmgp4tr3lw";
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            default = "Lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "hiddenText",
            default = "Hidden lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub hidden_text: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "radioText", default = "Lorem ipsum")]
        pub radio: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputColor", default = "#000000")]
        pub color: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputEmail", maxlength = 74)]
        pub email: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPassword", minlength = 8)]
        pub password: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPhone")]
        pub phone: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputUrl")]
        pub url: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIP", default = "127.0.0.1")]
        pub ip: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv4", default = "127.0.0.1")]
        pub ipv4: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv6", default = "::ffff:7f00:1")]
        pub ipv6: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "textArea", default = "Lorem ipsum")]
        pub textarea: Option<String>,
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
        DB_MAP_CLIENT_NAMES.write()?.insert(
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
        monitor.migrat();
        // Add metadata and widgects map to cache.
        TestModel::to_cache()?;
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
    // text
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("text").unwrap().value);
    // hidden_text
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Hidden lorem ipsum".to_string(),
        map_wigets.get("hidden_text").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("hidden_text").unwrap().value);
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("radio").unwrap().value);
    // color
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("color").unwrap().value);
    // email
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("ipv4").unwrap().value);
    // ipv6
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("ipv6").unwrap().value);
    // textarea
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("textarea").unwrap().value);

    // Validating values in database
    {
        let form_store = FORM_CACHE.read()?;
        let client_store = DB_MAP_CLIENT_NAMES.read()?;
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
        assert_eq!("Lorem ipsum", doc.get_str("text")?);
        assert_eq!("Hidden lorem ipsum", doc.get_str("hidden_text")?);
        assert_eq!("Lorem ipsum", doc.get_str("radio")?);
        assert_eq!("#000000", doc.get_str("color")?);
        assert_eq!(Some(()), doc.get("email").unwrap().as_null());
        assert!(doc.get("password").is_none());
        assert_eq!(Some(()), doc.get("phone").unwrap().as_null());
        assert_eq!(Some(()), doc.get("url").unwrap().as_null());
        assert_eq!("127.0.0.1", doc.get_str("ip")?);
        assert_eq!("127.0.0.1", doc.get_str("ipv4")?);
        assert_eq!("::ffff:7f00:1", doc.get_str("ipv6")?);
        assert_eq!("Lorem ipsum", doc.get_str("textarea")?);
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
    // text
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // hidden_text
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("hidden_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Hidden lorem ipsum".to_string(),
        map_wigets.get("hidden_text").unwrap().value
    );
    // radio
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("radio").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("color").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ip").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv4").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv6").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let result = test_model.save(None, None, None)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("textarea").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.read()?;
        let client_store = DB_MAP_CLIENT_NAMES.read()?;
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
        assert_eq!("Lorem ipsum", doc.get_str("text")?);
        assert_eq!("Hidden lorem ipsum", doc.get_str("hidden_text")?);
        assert_eq!("Lorem ipsum", doc.get_str("radio")?);
        assert_eq!("#000000", doc.get_str("color")?);
        assert_eq!(Some(()), doc.get("email").unwrap().as_null());
        assert!(doc.get("password").is_none());
        assert_eq!(Some(()), doc.get("phone").unwrap().as_null());
        assert_eq!(Some(()), doc.get("url").unwrap().as_null());
        assert_eq!("127.0.0.1", doc.get_str("ip")?);
        assert_eq!("127.0.0.1", doc.get_str("ipv4")?);
        assert_eq!("::ffff:7f00:1", doc.get_str("ipv6")?);
        assert_eq!("Lorem ipsum", doc.get_str("textarea")?);
    }

    // ---------------------------------------------------------------------------------------------
    del_test_db(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
