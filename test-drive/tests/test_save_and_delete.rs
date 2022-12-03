use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::sync::Client;
use parking_lot::RwLock;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{collections::HashMap, error::Error, fs, path::Path};

mod settings {
    pub const PROJECT_NAME: &str = "test_project_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_PROJECT_KEY: &str = "11de1G87Q41n46b2";
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
    pub fn run_migration(
        meta_store: &Arc<RwLock<HashMap<String, Meta>>>,
        client: &Client,
        _validators: &HashMap<String, Regex>,
        _media_dir: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(meta_store, client)?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            settings::PROJECT_NAME,
            settings::UNIQUE_PROJECT_KEY,
            get_model_key_list()?,
            meta_store,
            client,
        )?;

        // Monitor initialization.
        let monitor = Monitor {
            project_name: settings::PROJECT_NAME,
            unique_project_key: settings::UNIQUE_PROJECT_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(meta_store, client)?;

        Ok(())
    }
}

mod app_state {
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct AppState {
        media_root: String,
        media_url: String,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                // Root partition for storing files.
                media_root: String::from("./media"),
                // Url address to the root section.
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

    pub fn get_media_dir(app_state: AppState) -> HashMap<String, String> {
        [
            ("media_root".into(), app_state.media_root),
            ("media_url".into(), app_state.media_url),
        ]
        .iter()
        .cloned()
        .collect()
    }
}

// TEST
// #################################################################################################
#[test]
fn test_save_and_delete() -> Result<(), Box<dyn Error>> {
    // This is required for all projects.
    // =============================================================================================
    let app_state = app_state::get_app_state()?;
    let media_dir = app_state::get_media_dir(app_state);
    let meta_store = Arc::new(get_meta_store());
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let validators = get_validators()?;
    migration::run_migration(&meta_store, &client, &validators, &media_dir)?;

    // YOUR CODE ...
    // =============================================================================================
    type TestModel = models::TestModel;
    //
    let mut test_model = TestModel::new(&meta_store)?;
    test_model.password.set("j2972K4R3uQeVFPF");
    test_model.email.set("jane32@enhanceronly.com");

    // Create document
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.save(&meta_store, &client, &validators, &media_dir, None, None)?;
    test_model = output_data.update()?;
    //
    assert!(
        output_data.is_valid(),
        "Create document - is_valid(): {}",
        output_data.err_msg()
    );
    assert!(output_data.get_doc().is_none(), "get_doc() != is_none()");
    assert!(!output_data.hash().is_empty(), "hash() == is_empty()");
    assert!(
        output_data.created_at().is_some(),
        "created_at() != is_some()"
    );
    assert!(
        output_data.updated_at().is_some(),
        "updated_at() != is_some()"
    );
    assert!(output_data.obj_id()?.is_some(), "obj_id() != is_some()");
    assert!(!output_data.json()?.is_empty(), "json() == is_empty()");
    assert!(
        output_data.json_for_admin()?.is_some(),
        "json_for_admin() != is_some()"
    );

    // Delete document
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.delete(&meta_store, &client, None)?;
    assert!(
        output_data.is_valid(),
        "Delete document - is_valid(): {}",
        output_data.err_msg()
    );

    // Delete test database
    // =============================================================================================
    del_test_db(
        settings::PROJECT_NAME,
        settings::UNIQUE_PROJECT_KEY,
        migration::get_model_key_list()?,
        &meta_store,
        &client,
    )?;

    Ok(())
}
