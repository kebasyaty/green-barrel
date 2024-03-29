use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use serde::{Deserialize, Serialize};
use std::error::Error;

mod settings {
    // Project name.
    // Valid characters: _ a-z A-Z 0-9
    // Hint: PROJECT_NAM it is recommended not to change.
    // Max size: 20
    // First character: a-z A-Z
    pub const APP_NAME: &str = "test_app_name";
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 20
    // First character: a-z A-Z
    pub const DATABASE_NAME: &str = "test_app_name";
    // The unique key for this test.
    // To generate a key (This is not an advertisement): https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 16
    pub const UNIQUE_APP_KEY: &str = "SkC1UDJ3Z4ucCMW8";
    //
    pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 30
    // First character: a-z A-Z
    pub const SERVICE_NAME: &str = "test_service_name";
}

mod models {
    use super::*;
    use settings::{APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME, UNIQUE_APP_KEY};

    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        pub checkbox: BoolField,
        //
        pub date: DateField,
        pub datetime: DateTimeField,
        //
        pub file: FileField,
        pub image: ImageField,
        //
        pub number_i32: I32Field,
        pub radio_i32: I32Field,
        pub range_i32: I32Field,
        //
        pub number_u32: U32Field,
        pub radio_u32: U32Field,
        pub range_u32: U32Field,
        //
        pub number_i64: I64Field,
        pub radio_i64: I64Field,
        pub range_i64: I64Field,
        //
        pub number_f64: F64Field,
        pub radio_f64: F64Field,
        pub range_f64: F64Field,
        //
        pub radio_text: TextField,
        //
        pub select_text: ChoiceTextField,
        pub select_text_dyn: ChoiceTextDynField,
        pub select_text_mult: ChoiceTextMultField,
        pub select_text_mult_dyn: ChoiceTextMultDynField,
        //
        pub select_i32: ChoiceI32Field,
        pub select_i32_dyn: ChoiceI32DynField,
        pub select_i32_mult: ChoiceI32MultField,
        pub select_i32_mult_dyn: ChoiceI32MultDynField,
        //
        pub select_u32: ChoiceU32Field,
        pub select_u32_dyn: ChoiceU32DynField,
        pub select_u32_mult: ChoiceI32MultField,
        pub select_u32_mult_dyn: ChoiceU32MultDynField,
        //
        pub select_i64: ChoiceI64Field,
        pub select_i64_dyn: ChoiceI64DynField,
        pub select_i64_mult: ChoiceI64MultField,
        pub select_i64_mult_dyn: ChoiceI64MultDynField,
        //
        pub select_f64: ChoiceF64Field,
        pub select_f64_dyn: ChoiceF64DynField,
        pub select_f64_mult: ChoiceF64MultField,
        pub select_f64_mult_dyn: ChoiceF64MultDynField,
        //
        pub text: TextField,
        pub slug: SlugField,
        pub color: ColorField,
        pub email: EmailField,
        pub password: PasswordField,
        pub phone: PhoneField,
        pub url: URLField,
        pub ip: IPField,
    }

    impl Control for TestModel {
        fn custom() -> Self {
            Self {
                email: EmailField {
                    unique: true,
                    ..Default::default()
                },
                image: ImageField {
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
                radio_text: TextField {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                radio_i32: I32Field {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_i32: I32Field {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_u32: U32Field {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_u32: U32Field {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_i64: I64Field {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_i64: I64Field {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_f64: F64Field {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_f64: F64Field {
                    input_type: "range".into(),
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
            client,
            settings::APP_NAME,
            settings::UNIQUE_APP_KEY,
            get_model_key_list()?,
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

// TEST
// #################################################################################################
#[tokio::test]
async fn test_save_and_commons() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // =============================================================================================
    type TestModel = models::TestModel;

    for num in 1..=10 {
        let mut test_model = TestModel::new().await?;
        test_model.checkbox.set(true);
        test_model.date.set("1900-01-31");
        test_model.datetime.set("1900-01-31T00:00");
        test_model.file.set("./some_files/resume.pdf", false, None);
        test_model.image.set("./some_files/avatar.png", false, None);
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
        test_model.phone.set("+12029182132");
        test_model.url.set("https://ru.wikipedia.org/wiki/URL");
        test_model.ip.set("192.168.123.132");

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

    // Create index
    let options = IndexOptions::builder()
        .unique(true)
        .name("emailIdx".to_string())
        .build();
    let index = IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(options)
        .build();
    let result = TestModel::create_index(&client, index, None).await;
    assert!(result.is_ok(), "create_index() != is_ok()");
    // Drop index
    let name = "emailIdx";
    let result = TestModel::drop_index(&client, name, None).await;
    assert!(result.is_ok(), "drop_index() != is_ok()");
    // Create index
    let email_idx_options = IndexOptions::builder()
        .unique(true)
        .name("emailIdx".to_string())
        .build();
    let indexes = [IndexModel::builder()
        .keys(doc! { "email": 1 })
        .options(email_idx_options)
        .build()];
    let result = TestModel::create_indexes(&client, indexes, None).await;
    assert!(result.is_ok(), "create_indexes() != is_ok()");
    // Drop indexes
    let result = TestModel::drop_indexes(&client, None).await;
    assert!(result.is_ok(), "create_indexes() != is_ok()");
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
    let result = TestModel::find_one_to_doc(&client, filter, None).await?;
    assert!(result.is_some(), "find_one_to_doc() != is_some()");
    // find_one_to_json
    let filter = doc! {"email": "x5@x.xx"};
    let result = TestModel::find_one_to_json(&client, filter, None).await?;
    assert!(!result.is_empty(), "find_one_to_json() == is_empty()");
    // find_one_to_instance
    let filter = doc! {"email": "x1@x.xx"};
    let result = TestModel::find_one_to_instance(&client, filter, None).await?;
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
    let result = TestModel::find_one_and_delete(&client, filter, None).await?;
    assert!(result.is_some(), "find_one_and_delete() != is_some()");
    // count_documents
    let result = TestModel::count_documents(&client, None, None).await?;
    assert_eq!(result, 9, "count_documents() != 9");
    // estimated_document_count
    let result = TestModel::estimated_document_count(&client, None).await?;
    assert_eq!(result, 9, "estimated_document_count() != 9");
    // delete_one
    let query = doc! {"email": "x3@x.xx"};
    let result = TestModel::delete_one(&client, query, None).await?;
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
    let result = TestModel::delete_many(&client, query, None).await?;
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
        &client,
        settings::APP_NAME,
        settings::UNIQUE_APP_KEY,
        migration::get_model_key_list()?,
    )
    .await?;

    Ok(())
}
