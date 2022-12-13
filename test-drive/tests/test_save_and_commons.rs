use chrono::Local;
use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, Client};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::Path};
use uuid::Uuid;

mod settings {
    pub const APP_NAME: &str = "test_project_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_APP_KEY: &str = "SkC1UDJ3Z4ucCMW8";
    //
    pub const SERVICE_NAME: &str = "test_service_name";
    pub const DATABASE_NAME: &str = "test_database_name";
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
}

mod models {
    use super::*;
    use settings::{APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME, UNIQUE_APP_KEY};

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
                image: InputImage {
                    required: true,
                    default: Some(ImageData {
                        path: "./resources/media/default/no_image.png".into(),
                        url: "/media/default/no_image.png".into(),
                        ..Default::default()
                    }),
                    thumbnails: vec![
                        ("xs".into(), 150),
                        ("sm".into(), 300),
                        ("md".into(), 600),
                        ("lg".into(), 1200),
                    ],
                    is_quality: false,
                    ..Default::default()
                },
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
    pub async fn run_migration(client: &Client) -> Result<(), Box<dyn Error>> {
        // Caching metadata.
        models::TestModel::caching(client).await?;

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(
            settings::APP_NAME,
            settings::UNIQUE_APP_KEY,
            get_model_key_list()?,
            client,
        )
        .await?;

        // Monitor initialization.
        let monitor = Monitor {
            app_name: settings::APP_NAME,
            unique_app_key: settings::UNIQUE_APP_KEY,
            // Register models
            model_key_list: get_model_key_list()?,
        };
        monitor.migrat(client).await?;

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

mod helpers {
    use super::*;

    // Create a temporary file for the test
    pub fn copy_file(file_path: &str) -> Result<String, Box<dyn Error>> {
        let f_path = Path::new(file_path);
        if !f_path.is_file() {
            Err(format!("File is missing - {file_path}"))?
        }
        let dir_tmp = "./resources/media/tmp";
        fs::create_dir_all(dir_tmp)?;
        let f_name = Uuid::new_v4().to_string();
        let ext = f_path.extension().unwrap().to_str().unwrap();
        let f_tmp = format!("{dir_tmp}/{f_name}.{ext}");
        fs::copy(file_path, f_tmp.clone())?;
        Ok(f_tmp)
    }
}

// TEST
// #################################################################################################
#[tokio::test]
async fn test_save_and_commons() -> Result<(), Box<dyn Error>> {
    // THIS IS REQUIRED FOR ALL PROJECTS
    // Hint: This is done to be able to add data to streams.
    // =============================================================================================
    let _app_state = app_state::get_app_state()?;
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    migration::run_migration(&client).await?;

    // YOUR CODE ...
    // =============================================================================================
    type TestModel = models::TestModel;
    let tz = Local::now().format("%z").to_string();

    for num in 1..=10 {
        let f_path = helpers::copy_file("./resources/media/default/no_file.odt")?;
        let img_path = helpers::copy_file("./resources/media/default/no_image.png")?;

        let mut test_model = TestModel::new().await?;
        test_model.checkbox.set(true);
        test_model.date.set("1900-01-31");
        test_model.datetime.set("1900-01-31T00:00");
        test_model.file.set(f_path.as_str());
        test_model.image.set(img_path.as_str());
        test_model.number_i32.set(0);
        test_model.range_i32.set(0);
        test_model.number_u32.set(0);
        test_model.range_u32.set(0);
        test_model.number_i64.set(0);
        test_model.range_i64.set(0);
        test_model.number_f64.set(0.0);
        test_model.range_f64.set(0.0);
        test_model.text.set("Some text");
        test_model.color.set("#ffffff");
        test_model.email.value = Some(format!("x{num}@x.xx"));
        test_model.password.set("j2972K4R3uQeVFPF");
        test_model.phone.set("+1 202-918-2132");
        test_model.url.set("https://ru.wikipedia.org/wiki/URL");
        test_model.ip.set("192.168.123.132");
        test_model.ipv4.set("192.168.50.1");
        test_model.ipv6.set("1050:0:0:0:5:600:300c:326b");
        test_model.textarea.set("Some text");
        test_model.tz.set(tz.as_str());

        let output_data = test_model.save(&client, None, None).await?;
        test_model = output_data.update()?;

        assert!(
            output_data.is_valid(),
            "is_valid(): {}",
            output_data.err_msg()
        );
        assert!(
            test_model.slug.get().is_some(),
            "test_model.slug.get() != is_some()"
        );
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
    }

    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 10, "count_documents() != 10");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 10, "estimated_document_count() != 10");
    // find_many_to_doc_list
    let result = TestModel::find_many_to_doc_list(&client, None, None).await?;
    assert!(result.is_some(), "find_many_to_doc_list() != is_some()");
    assert_eq!(
        result.unwrap().len(),
        10,
        "find_many_to_doc_list(): len() != 10"
    );
    // find_many_to_json
    let result = TestModel::find_many_to_json(&client, None, None).await?;
    assert!(result.is_some(), "find_many_to_json() != is_some()");
    // find_one_to_doc
    let filter = doc! {"email": "x10@x.xx"};
    let result = TestModel::find_one_to_doc(filter, &client, None).await?;
    assert!(result.is_some(), "find_one_to_doc() != is_some()");
    // find_one_to_json
    let filter = doc! {"email": "x5@x.xx"};
    let result = TestModel::find_one_to_json(filter, &client, None).await?;
    assert!(!result.is_empty(), "find_one_to_json() == is_empty()");
    // find_one_to_instance
    let filter = doc! {"email": "x1@x.xx"};
    let result = TestModel::find_one_to_instance(filter, &client, None).await?;
    assert!(result.is_some(), "find_one_to_instance() != is_some()");
    // collection_name
    let result = TestModel::collection_name(&client).await?;
    assert!(!result.is_empty(), "collection_name() == is_empty()");
    // namespace
    let result = TestModel::namespace(&client).await?;
    assert!(!result.db.is_empty(), "namespace(): db == is_empty()");
    assert!(!result.coll.is_empty(), "namespace(): coll == is_empty()");
    // find_one_and_delete
    let filter = doc! {"email": "x2@x.xx"};
    let result = TestModel::find_one_and_delete(filter, &client, None).await?;
    assert!(result.is_some(), "find_one_and_delete() != is_some()");
    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 9, "count_documents() != 9");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 9, "estimated_document_count() != 9");
    // delete_one
    let query = doc! {"email": "x3@x.xx"};
    let result = TestModel::delete_one(query, &client, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(result.deleted_count()? == 1, "delete_one() != 1");
    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 8, "count_documents() != 8");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 8, "estimated_document_count() != 8");
    // delete_many
    let query = doc! {"email": {"$in": ["x4@x.xx", "x6@x.xx"]}};
    let result = TestModel::delete_many(query, &client, None).await?;
    assert!(result.is_valid(), "is_valid(): {}", result.err_msg());
    assert!(
        result.deleted_count()? == 2,
        "delete_many(): deleted_count() != 2"
    );
    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 6, "count_documents() != 6");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 6, "estimated_document_count() != 6");

    // Delete test database
    // =============================================================================================
    del_test_db(
        settings::APP_NAME,
        settings::UNIQUE_APP_KEY,
        migration::get_model_key_list()?,
        &client,
    )
    .await?;

    Ok(())
}
