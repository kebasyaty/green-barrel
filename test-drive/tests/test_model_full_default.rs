use async_lock::RwLock;
use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, error::Error, fs, path::Path};

mod settings {
    pub const PROJECT_NAME: &str = "test_project_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_PROJECT_KEY: &str = "dNTFT8173D6i3255";
    //
    pub const SERVICE_NAME: &str = "test_service_name";
    pub const DATABASE_NAME: &str = "test_database_name";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}

mod models {
    use super::*;
    use settings::{
        DATABASE_NAME, DB_QUERY_DOCS_LIMIT, PROJECT_NAME, SERVICE_NAME, UNIQUE_PROJECT_KEY,
    };

    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        pub checkbox: CheckBox,
        //
        pub date: InputDate,
        pub datetime: InputDateTime,
        //
        pub file: InputFile,
        pub image: InputImage,
        //
        pub number_i32: NumberI32,
        pub radio_i32: RadioI32,
        pub range_i32: RangeI32,
        //
        pub number_u32: NumberU32,
        pub radio_u32: RadioU32,
        pub range_u32: RangeU32,
        //
        pub number_i64: NumberI64,
        pub radio_i64: RadioI64,
        pub range_i64: RangeI64,
        //
        pub number_f64: NumberF64,
        pub radio_f64: RadioF64,
        pub range_f64: RangeF64,
        //
        pub radio_text: RadioText,
        //
        pub select_text: SelectText,
        pub select_text_dyn: SelectTextDyn,
        pub select_text_mult: SelectTextMult,
        pub select_text_mult_dyn: SelectTextMultDyn,
        //
        pub select_i32: SelectI32,
        pub select_i32_dyn: SelectI32Dyn,
        pub select_i32_mult: SelectI32Mult,
        pub select_i32_mult_dyn: SelectI32MultDyn,
        //
        pub select_u32: SelectU32,
        pub select_u32_dyn: SelectU32Dyn,
        pub select_u32_mult: SelectI32Mult,
        pub select_u32_mult_dyn: SelectU32MultDyn,
        //
        pub select_i64: SelectI64,
        pub select_i64_dyn: SelectI64Dyn,
        pub select_i64_mult: SelectI64Mult,
        pub select_i64_mult_dyn: SelectI64MultDyn,
        //
        pub select_f64: SelectF64,
        pub select_f64_dyn: SelectF64Dyn,
        pub select_f64_mult: SelectF64Mult,
        pub select_f64_mult_dyn: SelectF64MultDyn,
        //
        pub text: InputText,
        pub slug: AutoSlug,
        pub color: InputColor,
        pub email: InputEmail,
        pub password: InputPassword,
        pub phone: InputPhone,
        pub url: InputUrl,
        pub ip: InputIP,
        pub ipv4: InputIPv4,
        pub ipv6: InputIPv6,
        pub textarea: TextArea,
    }

    impl Control for TestModel {
        fn custom_default() -> Self {
            Self {
                ..Default::default()
            }
        }
    }
}

mod migration {
    use super::*;

    // Get metadata list
    pub fn get_model_key_list() -> Result<Vec<String>, Box<dyn Error>> {
        let model_key_list = vec![models::TestModel::key()?];
        Ok(model_key_list)
    }

    // Migration
    pub async fn run_migration(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
    ) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(meta_store, client).await?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            settings::PROJECT_NAME,
            settings::UNIQUE_PROJECT_KEY,
            get_model_key_list()?,
            meta_store,
            client,
        )
        .await?;

        // Monitor initialization.
        let monitor = Monitor {
            project_name: settings::PROJECT_NAME,
            unique_project_key: settings::UNIQUE_PROJECT_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(meta_store, client).await?;

        Ok(())
    }
}

mod app_state {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppState {
        pub media_root: String,
        pub media_url: String,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                media_root: String::from("./media"),
                media_url: String::from("/media"),
            }
        }
    }

    pub fn get_app_state() -> Result<AppState, Box<dyn Error>> {
        let path = Path::new("./AppState.toml");
        if !path.is_file() {
            fs::File::create(path)?;
            let cfg = AppState::default();
            confy::store_path(path, cfg)?;
        }
        Ok(confy::load_path::<AppState>(path)?)
    }
}

// TEST
// #################################################################################################
#[tokio::test]
async fn test_model_full_default() -> Result<(), Box<dyn Error>> {
    // THIS IS REQUIRED FOR ALL PROJECTS
    // Hint: This is done to be able to add data to streams.
    // =============================================================================================
    let _app_state = app_state::get_app_state()?;
    let meta_store = Arc::new(get_meta_store());
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    migration::run_migration(&meta_store, &client).await?;

    // YOUR CODE ...
    // =============================================================================================
    type TestModel = models::TestModel;
    //
    // Module: green-barrel/src/models/caching.rs
    // ---------------------------------------------------------------------------------------------
    // new
    assert!(
        TestModel::new(&meta_store).await.is_ok(),
        "new() != is_ok()"
    );
    // to_json
    assert!(
        !TestModel::json(&meta_store).await?.is_empty(),
        "json() != is_empty()"
    );
    //
    // Module: green-barrel/src/models/db_query_api/commons.rs
    // ---------------------------------------------------------------------------------------------
    // aggregate
    let pipeline = vec![doc! {}];
    let result = TestModel::aggregate(pipeline, &meta_store, &client, None).await;
    assert!(result.is_err(), "aggregate() != is_err()");
    // count_documents
    let result = TestModel::count_documents(&meta_store, &client, None, None).await?;
    assert_eq!(result, 0, "count_documents() != 0");
    // delete_many
    let query = doc! {};
    let result = TestModel::delete_many(query, &meta_store, &client, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(
        result.deleted_count()? == 0,
        "delete_many(): deleted_count() != 0"
    );
    // delete_one
    let query = doc! {};
    let result = TestModel::delete_one(query, &meta_store, &client, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(result.deleted_count()? == 0, "delete_one() != 0");
    // distinct
    let field_name = "checkbox";
    let filter = doc! {};
    let result = TestModel::distinct(field_name, &meta_store, &client, Some(filter), None).await?;
    assert!(result.is_empty(), "distinct() != is_empty()");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&meta_store, &client, None).await?;
    assert_eq!(result, 0, "estimated_document_count != 0_i64");
    // find_many_to_doc_list
    let result = TestModel::find_many_to_doc_list(&meta_store, &client, None, None).await?;
    assert!(result.is_none(), "find_many_to_doc_list() != is_none()");
    // find_many_to_json
    let result = TestModel::find_many_to_json(&meta_store, &client, None, None).await?;
    assert!(result.is_none(), "find_many_to_json() != is_none");
    // find_one_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_doc(filter, &meta_store, &client, None).await?;
    assert!(result.is_none(), "find_one_to_doc() != is_none()");
    // find_one_to_json
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_json(filter, &meta_store, &client, None).await?;
    assert!(result.is_empty(), "find_one_to_json() != is_empty()");
    // find_one_to_instance
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_instance(filter, &meta_store, &client, None).await?;
    assert!(result.is_none(), "find_one_to_instance() != is_none()");
    // find_one_and_delete
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete(filter, &meta_store, &client, None).await?;
    assert!(result.is_none(), "find_one_and_delete() != is_none()");
    // collection_name
    let result = TestModel::collection_name(&meta_store, &client).await?;
    assert!(!result.is_empty(), "name() != is_empty()");
    // namespace
    let result = TestModel::namespace(&meta_store, &client).await?;
    assert!(!result.db.is_empty(), "namespace() != is_empty()");
    assert!(!result.coll.is_empty(), "namespace() != is_empty()");

    // Delete test database
    // =============================================================================================
    del_test_db(
        settings::PROJECT_NAME,
        settings::UNIQUE_PROJECT_KEY,
        migration::get_model_key_list()?,
        &meta_store,
        &client,
    )
    .await?;

    Ok(())
}
