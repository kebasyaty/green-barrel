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
    pub const SERVICE_NAME: &str = "TEST_8JUBHtgyg8gE_jk5";
    pub const DATABASE_NAME: &str = "TEST_VHTzUP3Fg_WdYz7W";
    pub const DB_CLIENT_NAME: &str = "TEST_default_PgkpehXG3GK_XY4T";
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_j2zgL7sj6wVmv_Mg";

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
        #[field_attrs(widget = "checkBoxText", default = "Lorem ipsum")]
        pub checkbox: Option<String>,
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
        #[serde(default)]
        #[field_attrs(
            widget = "selectText",
            label = "Choose a car:",
            default = "mercedes",
            select = "[[\"volvo\",\"Volvo\"], [\"saab\",\"Saab\"], [\"mercedes\",\"Mercedes\"], [\"audi\",\"Audi\"]]"
        )]
        pub select: Option<String>,
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
            "TEST_default_PgkpehXG3GK_XY4T".to_string(),
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
fn test_model_with_filling_fields() -> Result<(), Box<dyn std::error::Error>> {
    app_name::mango_migration()?;

    let mut test_model = app_name::TestModel {
        text: Some("Lorem ipsum dolor sit amet".to_string()),
        checkbox: Some("Lorem ipsum dolor sit amet".to_string()),
        radio: Some("Lorem ipsum dolor sit amet".to_string()),
        color: Some("#ffffff".to_string()),
        email: Some("no_reply@email.net".to_string()),
        password: Some("12345678".to_string()),
        phone: Some("+00000000000".to_string()),
        url: Some("https://www.google.com/".to_string()),
        ip: Some("172.217.14.196".to_string()),
        ipv4: Some("172.217.14.196".to_string()),
        ipv6: Some("::ffff:acd9:ec4".to_string()),
        textarea: Some("Lorem ipsum dolor sit amet".to_string()),
        select: Some("audi".to_string()),
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        text: Some("Lorem ipsum dolor sit amet".to_string()),
        checkbox: Some("Lorem ipsum dolor sit amet".to_string()),
        radio: Some("Lorem ipsum dolor sit amet".to_string()),
        color: Some("#ffffff".to_string()),
        email: Some("no_reply@email.net".to_string()),
        password: Some("12345678".to_string()),
        phone: Some("+00000000000".to_string()),
        url: Some("https://www.google.com/".to_string()),
        ip: Some("172.217.14.196".to_string()),
        ipv4: Some("172.217.14.196".to_string()),
        ipv6: Some("::ffff:acd9:ec4".to_string()),
        textarea: Some("Lorem ipsum dolor sit amet".to_string()),
        select: Some("audi".to_string()),
        ..Default::default()
    };

    // Create
    // -----------------------------------------------------------------------------------------
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
    // text
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // checkbox
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("checkbox").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("checkbox").unwrap().value
    );
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "#ffffff".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(
        "no_reply@email.net".to_string(),
        map_wigets.get("email").unwrap().value
    );
    // password
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(
        "+00000000000".to_string(),
        map_wigets.get("phone").unwrap().value
    );
    // url
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(
        "https://www.google.com/".to_string(),
        map_wigets.get("url").unwrap().value
    );
    // ip
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    let map_wigets = result_2.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ip").unwrap().value
    );
    // ipv4
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "::ffff:acd9:ec4".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    // select
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "mercedes".to_string(),
        map_wigets.get("select").unwrap().value
    );
    let map_wigets = result_2.wig();
    assert_eq!("audi".to_string(), map_wigets.get("select").unwrap().value);

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
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("text")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("checkbox")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("radio")?);
        assert_eq!("#ffffff", doc.get_str("color")?);
        assert_eq!("no_reply@email.net", doc.get_str("email")?);
        assert!(argon2::verify_encoded(
            &doc.get_str("password")?,
            test_model.password.clone().unwrap().as_bytes()
        )
        .unwrap());
        assert_eq!("+00000000000", doc.get_str("phone")?);
        assert_eq!("https://www.google.com/", doc.get_str("url")?);
        assert_eq!("172.217.14.196", doc.get_str("ip")?);
        assert_eq!("172.217.14.196", doc.get_str("ipv4")?);
        assert_eq!("::ffff:acd9:ec4", doc.get_str("ipv6")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("textarea")?);
        assert_eq!("audi", doc.get_str("select")?);
    }

    // Update
    // -----------------------------------------------------------------------------------------
    let result = test_model.save(OutputType::Hash)?;
    // Validating update
    assert!(result.bool(), "{}", result.hash());
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validating values
    // text
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // checkbox
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("checkbox").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("checkbox").unwrap().value
    );
    // radio
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "#ffffff".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "no_reply@email.net".to_string(),
        map_wigets.get("email").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "+00000000000".to_string(),
        map_wigets.get("phone").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "https://www.google.com/".to_string(),
        map_wigets.get("url").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ip").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "::ffff:acd9:ec4".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    // select
    let result = test_model.save(OutputType::Wig)?;
    let map_wigets = result.wig();
    assert_eq!("audi".to_string(), map_wigets.get("select").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "mercedes".to_string(),
        map_wigets.get("select").unwrap().value
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
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("text")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("checkbox")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("radio")?);
        assert_eq!("#ffffff", doc.get_str("color")?);
        assert_eq!("no_reply@email.net", doc.get_str("email")?);
        assert!(argon2::verify_encoded(
            &doc.get_str("password")?,
            test_model.password.unwrap().as_bytes()
        )
        .unwrap());
        assert_eq!("+00000000000", doc.get_str("phone")?);
        assert_eq!("https://www.google.com/", doc.get_str("url")?);
        assert_eq!("172.217.14.196", doc.get_str("ip")?);
        assert_eq!("172.217.14.196", doc.get_str("ipv4")?);
        assert_eq!("::ffff:acd9:ec4", doc.get_str("ipv6")?);
        assert_eq!("Lorem ipsum dolor sit amet", doc.get_str("textarea")?);
        assert_eq!("audi", doc.get_str("select")?);
    }

    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    Ok(())
}
