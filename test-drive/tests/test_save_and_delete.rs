use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::Client;
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
    pub const UNIQUE_APP_KEY: &str = "11de1G87Q41n46b2";
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
        pub ipv4: IPField,
        pub ipv6: IPField,
    }

    impl Control for TestModel {
        fn custom() -> Self {
            Self {
                checkbox: BoolField {
                    ..Default::default()
                },
                date: DateField {
                    required: true,
                    default: Some("1970-02-28".into()),
                    min: "1970-01-01".into(),
                    max: "1970-03-01".into(),
                    ..Default::default()
                },
                datetime: DateTimeField {
                    required: true,
                    default: Some("1970-02-28T00:00".into()),
                    min: "1970-01-01T00:00".into(),
                    max: "1970-03-01T00:00".into(),
                    ..Default::default()
                },
                file: FileField {
                    required: true,
                    default: Some(FileData {
                        path: "./resources/media/default/no_file.odt".into(),
                        url: "/media/default/no_file.odt".into(),
                        ..Default::default()
                    }),
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
                    ..Default::default()
                },
                number_i32: I32Field {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                radio_i32: I32Field {
                    input_type: "radio".into(),
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_i32: I32Field {
                    input_type: "range".into(),
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                number_u32: U32Field {
                    required: true,
                    default: Some(0),
                    min: 0,
                    max: 1,
                    ..Default::default()
                },
                radio_u32: U32Field {
                    input_type: "radio".into(),
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_u32: U32Field {
                    input_type: "range".into(),
                    required: true,
                    default: Some(1),
                    min: 0,
                    max: 1,
                    ..Default::default()
                },
                number_i64: I64Field {
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                radio_i64: I64Field {
                    input_type: "radio".into(),
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_i64: I64Field {
                    input_type: "range".into(),
                    required: true,
                    default: Some(0),
                    min: -1,
                    max: 1,
                    ..Default::default()
                },
                number_f64: F64Field {
                    required: true,
                    default: Some(0.0),
                    min: 0.0,
                    max: 1.0,
                    ..Default::default()
                },
                radio_f64: F64Field {
                    input_type: "radio".into(),
                    required: true,
                    default: Some(0.0),
                    choices: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                range_f64: F64Field {
                    input_type: "range".into(),
                    required: true,
                    default: Some(0.0),
                    min: -1.0,
                    max: 1.0,
                    ..Default::default()
                },
                radio_text: TextField {
                    input_type: "radio".into(),
                    required: true,
                    default: Some("value".to_string()),
                    choices: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text: ChoiceTextField {
                    required: true,
                    default: Some("value".into()),
                    choices: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text_dyn: ChoiceTextDynField {
                    ..Default::default()
                },
                select_text_mult: ChoiceTextMultField {
                    required: true,
                    default: Some(vec!["value".into(), "value 2".into()]),
                    choices: vec![
                        ("value".into(), "Title".into()),
                        ("value 1".into(), "Title 1".into()),
                        ("value 2".into(), "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_text_mult_dyn: ChoiceTextMultDynField {
                    ..Default::default()
                },
                select_i32: ChoiceI32Field {
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i32_dyn: ChoiceI32DynField {
                    ..Default::default()
                },
                select_i32_mult: ChoiceI32MultField {
                    required: true,
                    default: Some(vec![0, 1]),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i32_mult_dyn: ChoiceI32MultDynField {
                    ..Default::default()
                },
                select_u32: ChoiceU32Field {
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_u32_dyn: ChoiceU32DynField {
                    ..Default::default()
                },
                select_u32_mult: ChoiceI32MultField {
                    required: true,
                    default: Some(vec![0, 1]),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_u32_mult_dyn: ChoiceU32MultDynField {
                    ..Default::default()
                },
                select_i64: ChoiceI64Field {
                    required: true,
                    default: Some(0),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i64_dyn: ChoiceI64DynField {
                    ..Default::default()
                },
                select_i64_mult: ChoiceI64MultField {
                    required: true,
                    default: Some(vec![0, 1]),
                    choices: vec![
                        (0, "Title".into()),
                        (1, "Title 1".into()),
                        (2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_i64_mult_dyn: ChoiceI64MultDynField {
                    ..Default::default()
                },
                select_f64: ChoiceF64Field {
                    required: true,
                    default: Some(0.0),
                    choices: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_f64_dyn: ChoiceF64DynField {
                    ..Default::default()
                },
                select_f64_mult: ChoiceF64MultField {
                    required: true,
                    default: Some(vec![0.0, 0.1]),
                    choices: vec![
                        (0.0, "Title".into()),
                        (0.1, "Title 1".into()),
                        (0.2, "Title 2".into()),
                    ],
                    ..Default::default()
                },
                select_f64_mult_dyn: ChoiceF64MultDynField {
                    ..Default::default()
                },
                text: TextField {
                    required: true,
                    default: Some("Some text".to_string()),
                    ..Default::default()
                },
                slug: SlugField {
                    slug_sources: vec!["email".into(), "phone".into()],
                    ..Default::default()
                },
                color: ColorField {
                    required: true,
                    default: Some("#ffffff".to_string()),
                    ..Default::default()
                },
                email: EmailField {
                    required: true,
                    ..Default::default()
                },
                password: PasswordField {
                    required: true,
                    ..Default::default()
                },
                phone: PhoneField {
                    required: true,
                    default: Some("+12029182132".to_string()),
                    ..Default::default()
                },
                url: URLField {
                    required: true,
                    default: Some("https://ru.wikipedia.org/wiki/URL".to_string()),
                    ..Default::default()
                },
                ip: IPField {
                    required: true,
                    default: Some("192.168.123.132".to_string()),
                    ..Default::default()
                },
                ipv4: IPField {
                    field_type: "IPv4Field".into(),
                    required: true,
                    default: Some("192.168.50.1".to_string()),
                    ..Default::default()
                },
                ipv6: IPField {
                    field_type: "IPv6Field".into(),
                    required: true,
                    default: Some("1050:0:0:0:5:600:300c:326b".to_string()),
                    ..Default::default()
                },
                /*
                hash: Hash::default(),
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
async fn test_save_and_delete() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // =============================================================================================
    type TestModel = models::TestModel;
    //
    let mut test_model = TestModel::new().await?;
    test_model.password.set("j2972K4R3uQeVFPF");
    test_model.email.set("jane32@enhanceronly.com");

    // Create document
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.save(&client, None, None).await?;
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
    let output_data = test_model.delete(&client, None).await?;
    assert!(
        output_data.is_valid(),
        "Delete document - is_valid(): {}",
        output_data.err_msg()
    );

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
