use async_lock::RwLock;
use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, error::Error, fs, path::Path};

mod settings {
    pub const PROJECT_NAME: &str = "test_project_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_PROJECT_KEY: &str = "33J2PLYG729WSCK5";
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
                checkbox: CheckBox {
                    required: true,
                    ..Default::default()
                },
                date: InputDate {
                    required: true,
                    default: Some("1970-02-28".to_string()),
                    min: "1970-01-01".into(),
                    max: "1970-03-01".into(),
                    ..Default::default()
                },
                datetime: InputDateTime {
                    required: true,
                    default: Some("1970-02-28T00:00".to_string()),
                    min: "1970-01-01T00:00".into(),
                    max: "1970-03-01T00:00".into(),
                    ..Default::default()
                },
                file: InputFile {
                    required: true,
                    default: Some(FileData {
                        path: "./media/default/no_file.odt".into(),
                        url: "/media/default/no_file.odt".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                image: InputImage {
                    required: true,
                    default: Some(ImageData {
                        path: "./media/default/no_image.png".into(),
                        url: "/media/default/no_image.png".into(),
                        ..Default::default()
                    }),
                    thumbnails: vec![
                        ("xs".into(), 150),
                        ("sm".into(), 300),
                        ("md".into(), 600),
                        ("lg".into(), 1200),
                    ],
                    ..Default::default()
                },
                number_i32: NumberI32 {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                radio_i32: RadioI32 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_i32: RangeI32 {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                number_u32: NumberU32 {
                    required: true,
                    default: Some(0),
                    min: 0,
                    max: 1,
                    ..Default::default()
                },
                radio_u32: RadioU32 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_u32: RangeU32 {
                    required: true,
                    default: Some(1),
                    min: 0,
                    max: 1,
                    ..Default::default()
                },
                number_i64: NumberI64 {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                radio_i64: RadioI64 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_i64: RangeI64 {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                number_f64: NumberF64 {
                    required: true,
                    default: Some(0.0),
                    min: 0.0,
                    max: 1.0,
                    ..Default::default()
                },
                radio_f64: RadioF64 {
                    required: true,
                    default: Some(0.0),
                    options: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_f64: RangeF64 {
                    required: true,
                    default: Some(0.0),
                    min: -1.0,
                    max: 1.0,
                    ..Default::default()
                },
                radio_text: RadioText {
                    required: true,
                    default: Some("value".to_string()),
                    options: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text: SelectText {
                    required: true,
                    default: Some("value".into()),
                    options: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text_dyn: SelectTextDyn {
                    ..Default::default()
                },
                select_text_mult: SelectTextMult {
                    required: true,
                    default: Some(vec!["value".into(), "value 2".into()]),
                    options: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text_mult_dyn: SelectTextMultDyn {
                    ..Default::default()
                },
                select_i32: SelectI32 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i32_dyn: SelectI32Dyn {
                    ..Default::default()
                },
                select_i32_mult: SelectI32Mult {
                    required: true,
                    default: Some(vec![0, 1]),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i32_mult_dyn: SelectI32MultDyn {
                    ..Default::default()
                },
                select_u32: SelectU32 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_u32_dyn: SelectU32Dyn {
                    ..Default::default()
                },
                select_u32_mult: SelectI32Mult {
                    required: true,
                    default: Some(vec![0, 1]),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_u32_mult_dyn: SelectU32MultDyn {
                    ..Default::default()
                },
                select_i64: SelectI64 {
                    required: true,
                    default: Some(0),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i64_dyn: SelectI64Dyn {
                    ..Default::default()
                },
                select_i64_mult: SelectI64Mult {
                    required: true,
                    default: Some(vec![0, 1]),
                    options: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i64_mult_dyn: SelectI64MultDyn {
                    ..Default::default()
                },
                select_f64: SelectF64 {
                    required: true,
                    default: Some(0.0),
                    options: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_f64_dyn: SelectF64Dyn {
                    ..Default::default()
                },
                select_f64_mult: SelectF64Mult {
                    required: true,
                    default: Some(vec![0.0, 0.1]),
                    options: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_f64_mult_dyn: SelectF64MultDyn {
                    ..Default::default()
                },
                text: InputText {
                    required: true,
                    default: Some("Some text".to_string()),
                    ..Default::default()
                },
                slug: AutoSlug {
                    slug_sources: vec!["email".into(), "phone".into()],
                    ..Default::default()
                },
                color: InputColor {
                    required: true,
                    default: Some("#ffffff".to_string()),
                    ..Default::default()
                },
                email: InputEmail {
                    required: true,
                    ..Default::default()
                },
                password: InputPassword {
                    required: true,
                    ..Default::default()
                },
                phone: InputPhone {
                    required: true,
                    default: Some("+1 202-918-2132".to_string()),
                    ..Default::default()
                },
                url: InputUrl {
                    required: true,
                    default: Some("https://ru.wikipedia.org/wiki/URL".to_string()),
                    ..Default::default()
                },
                ip: InputIP {
                    required: true,
                    default: Some("192.168.123.132".to_string()),
                    ..Default::default()
                },
                ipv4: InputIPv4 {
                    required: true,
                    default: Some("192.168.50.1".to_string()),
                    ..Default::default()
                },
                ipv6: InputIPv6 {
                    required: true,
                    default: Some("1050:0:0:0:5:600:300c:326b".to_string()),
                    ..Default::default()
                },
                textarea: TextArea {
                    required: true,
                    default: Some("Some text".to_string()),
                    ..Default::default()
                },
                /*
                hash: HiddenHash::default(),
                created_at: HiddenDateTime::default(),
                updated_at: HiddenDateTime::default(),
                */
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

        // Run fixtures
        models::TestModel::run_fixture("test", meta_store, client).await?;

        Ok(())
    }
}

mod app_state {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppState {
        pub app_name: String,
        pub media_root: String,
        pub media_url: String,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                app_name: "App Name".into(),
                media_root: "./resources/media".into(), // the resources directory is recommended to be used as a standard
                media_url: "/media".into(),
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
async fn test_fixtures() -> Result<(), Box<dyn Error>> {
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

    let count = TestModel::estimated_document_count(&meta_store, &client, None).await?;
    assert_eq!(count, 2, "count != 2");

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
