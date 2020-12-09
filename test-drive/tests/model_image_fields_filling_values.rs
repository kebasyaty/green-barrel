use mango_orm::*;
use mango_orm::{forms::ImageData, migration::Monitor, test_tool::del_test_base};
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
    pub const SERVICE_NAME: &str = "TEST_hC_6tGGxnAHU1HhQ";
    pub const DATABASE_NAME: &str = "TEST_J4LQH8t_ttshB7nH";
    pub const DB_CLIENT_NAME: &str = "TEST_default_2dQycNc2bDtFk8_T";
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_pd4e5_CjNGBaesYy";

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(
            widget = "inputImage",
            default = "{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}"
        )]
        pub image: Option<String>,
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
            "TEST_default_2dQycNc2bDtFk8_T".to_string(),
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
        image: Some(
            "{\"path\":\"./media/beautiful-mountains.jpg\",\"url\":\"/media/beautiful-mountains.jpg\"}"
                .to_string(),
        ),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let image_data = ImageData {
        path: "./media/beautiful-mountains.jpg".to_string(),
        url: "/media/beautiful-mountains.jpg".to_string(),
        name: "beautiful-mountains.jpg".to_string(),
        size: 241138_u32,
        width: 1024_u32,
        height: 748_u32,
    };
    let result = test_model.save()?;
    // Validating create
    assert!(result.bool()?, "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validating values in widgets
    // checkbox
    let result = test_model.save()?;
    let map_wigets = result.wig()?;
    assert_eq!(
        "{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}",
        map_wigets.get("image").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}",
        map_wigets.get("image").unwrap().value
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
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("image"));
        assert_eq!(
            image_data,
            from_document::<ImageData>(doc.get_document("image")?.clone())?
        );
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save()?;
    // Validating update
    assert!(result.bool()?, "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // Validating values
    // checkbox
    let result = test_model.save()?;
    let map_wigets = result.wig()?;
    assert_eq!(
        "{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}",
        map_wigets.get("image").unwrap().value
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        "{\"path\":\"./media/no-image-found.png\",\"url\":\"/media/no-image-found.png\"}",
        map_wigets.get("image").unwrap().value
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
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("image"));
        assert_eq!(
            image_data,
            from_document::<ImageData>(doc.get_document("image")?.clone())?
        );
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
