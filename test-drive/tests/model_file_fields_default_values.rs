use mango_orm::*;
use mango_orm::{forms::FileData, migration::Monitor, test_tool::del_test_base};
use metamorphose::Model;
use mongodb::{
    bson::{de::from_document, doc, oid::ObjectId},
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
    pub const UNIQUE_PROJECT_KEY: &str = "4wSJvbMCRjn1sxM";
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
            widget = "inputFile",
            default = r#"{
                "path":"./media/hello_world.odt",
                "url":"/media/hello_world.odt"
            }"#
        )]
        pub file: Option<String>,
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
        del_test_base(PROJECT_NAME, UNIQUE_PROJECT_KEY, &model_list()?)?;
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

    // Create
    // ---------------------------------------------------------------------------------------------
    let file_data = FileData {
        path: "./media/hello_world.odt".to_string(),
        url: "/media/hello_world.odt".to_string(),
        name: "hello_world.odt".to_string(),
        size: 9741_u32,
    };
    let result = test_model.save(None, None, None)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validating values in widgets
    // file
    let map_wigets = result.wig();
    assert!(map_wigets.get("file").unwrap().value.is_empty());
    /*
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/hello_world.odt","url":"/media/hello_world.odt"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("file").unwrap().value.as_str()
        )?
    );
    */

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
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("file"));
        assert_eq!(
            file_data,
            from_document::<FileData>(doc.get_document("file")?.clone())?
        );
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
    // file
    let map_wigets = result.wig();
    assert!(map_wigets.get("file").unwrap().value.is_empty());
    /*
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/hello_world.odt","url":"/media/hello_world.odt"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("file").unwrap().value.as_str()
        )?
    );
    */

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
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("file"));
        assert_eq!(
            file_data,
            from_document::<FileData>(doc.get_document("file")?.clone())?
        );
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
