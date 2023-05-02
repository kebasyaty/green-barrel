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
    pub const UNIQUE_APP_KEY: &str = "jlgx755W6315Ig02";
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
        pub checkbox: Bool,
        //
        pub date: Date,
        pub datetime: DateTime,
        //
        pub file: File,
        pub image: Image,
        //
        pub number_i32: I32,
        pub radio_i32: I32,
        pub range_i32: I32,
        //
        pub number_u32: U32,
        pub radio_u32: U32,
        pub range_u32: U32,
        //
        pub number_i64: I64,
        pub radio_i64: I64,
        pub range_i64: I64,
        //
        pub number_f64: F64,
        pub radio_f64: F64,
        pub range_f64: F64,
        //
        pub radio_text: Text,
        //
        pub select_text: ChoiceText,
        pub select_text_dyn: ChoiceTextDyn,
        pub select_text_mult: ChoiceTextMult,
        pub select_text_mult_dyn: ChoiceTextMultDyn,
        //
        pub select_i32: ChoiceI32,
        pub select_i32_dyn: ChoiceI32Dyn,
        pub select_i32_mult: ChoiceI32Mult,
        pub select_i32_mult_dyn: ChoiceI32MultDyn,
        //
        pub select_u32: ChoiceU32,
        pub select_u32_dyn: ChoiceU32Dyn,
        pub select_u32_mult: ChoiceI32Mult,
        pub select_u32_mult_dyn: ChoiceU32MultDyn,
        //
        pub select_i64: ChoiceI64,
        pub select_i64_dyn: ChoiceI64Dyn,
        pub select_i64_mult: ChoiceI64Mult,
        pub select_i64_mult_dyn: ChoiceI64MultDyn,
        //
        pub select_f64: ChoiceF64,
        pub select_f64_dyn: ChoiceF64Dyn,
        pub select_f64_mult: ChoiceF64Mult,
        pub select_f64_mult_dyn: ChoiceF64MultDyn,
        //
        pub text: Text,
        pub slug: Slug,
        pub color: Color,
        pub email: Email,
        pub password: Password,
        pub phone: Phone,
        pub url: Url,
        pub ip: IP,
    }

    impl Control for TestModel {
        fn custom() -> Self {
            Self {
                radio_text: Text {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                radio_i32: I32 {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_i32: I32 {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_u32: U32 {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_u32: U32 {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_i64: I64 {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_i64: I64 {
                    input_type: "range".into(),
                    ..Default::default()
                },
                radio_f64: F64 {
                    input_type: "radio".into(),
                    ..Default::default()
                },
                range_f64: F64 {
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
async fn test_error_check_options() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // =============================================================================================
    type TestModel = models::TestModel;
    //
    // Positive
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new().await?;
    let output_data = test_model.check(&client, None).await?;
    test_model = output_data.update()?;

    assert!(
        output_data.is_valid(),
        "is_valid(): {}",
        output_data.err_msg()
    );
    assert!(
        test_model.slug.get().is_none(),
        "test_model.slug.get() != is_none()"
    );
    assert!(
        output_data.get_doc().unwrap().is_empty(),
        "get_doc() != is_empty()"
    );
    assert!(output_data.hash().is_empty(), "hash() != is_empty()");
    assert!(
        output_data.created_at().is_none(),
        "created_at() != is_none()"
    );
    assert!(
        output_data.updated_at().is_none(),
        "updated_at() != is_none()"
    );
    assert!(output_data.obj_id()?.is_none(), "obj_id() != is_none()");
    assert!(!output_data.json()?.is_empty(), "json() == is_empty()");
    assert!(
        output_data.json_for_admin()?.is_some(),
        "json_for_admin() != is_some()"
    );

    // Negative - In select type, there are no options to select
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new().await?;
    test_model.checkbox.set(true);
    test_model.date.set("1900-01-31");
    test_model.datetime.set("1900-01-31T00:00");
    test_model.file.set("./some_files/resume.pdf", false, None);
    test_model.image.set("./some_files/avatar.png", false, None);
    test_model.number_i32.set(0);
    test_model.radio_i32.set(0);
    test_model.range_i32.set(0);
    test_model.number_u32.set(0);
    test_model.radio_u32.set(0);
    test_model.range_u32.set(0);
    test_model.number_i64.set(0);
    test_model.radio_i64.set(0);
    test_model.range_i64.set(0);
    test_model.number_f64.set(0.0);
    test_model.radio_f64.set(0.0);
    test_model.range_f64.set(0.0);
    test_model.radio_text.set("Some text");
    test_model.select_text.set("Some text");
    test_model.select_text_dyn.set("Some text");
    test_model
        .select_text_mult
        .set(vec!["Some text", "Some text 2"]);
    test_model
        .select_text_mult_dyn
        .set(vec!["Some text", "Some text 2"]);
    test_model.select_i32.set(0);
    test_model.select_i32_dyn.set(0);
    test_model.select_i32_mult.set(vec![0, 1]);
    test_model.select_i32_mult_dyn.set(vec![0, 1]);
    test_model.select_u32.set(0);
    test_model.select_u32_dyn.set(0);
    test_model.select_u32_mult.set(vec![0, 1]);
    test_model.select_u32_mult_dyn.set(vec![0, 1]);
    test_model.select_i64.set(0);
    test_model.select_i64_dyn.set(0);
    test_model.select_i64_mult.set(vec![0, 1]);
    test_model.select_i64_mult_dyn.set(vec![0, 1]);
    test_model.select_f64.set(0.0);
    test_model.select_f64_dyn.set(0.0);
    test_model.select_f64_mult.set(vec![0.0, 1.0]);
    test_model.select_f64_mult_dyn.set(vec![0.0, 1.0]);
    test_model.text.set("Some text");
    test_model.color.set("#ffffff");
    test_model.email.set("jane32@enhanceronly.com");
    test_model.password.set("j2972K4R3uQeVFPF");
    test_model.phone.set("+12029182132");
    test_model.url.set("https://ru.wikipedia.org/wiki/URL");
    test_model.ip.set("192.168.123.132");

    let output_data = test_model.check(&client, None).await?;
    test_model = output_data.update()?;

    assert!(!output_data.is_valid(), "is_valid() != false");
    assert!(
        test_model.slug.get().is_none(),
        "test_model.slug.get() != is_none()"
    );
    assert!(
        output_data.get_doc().unwrap().is_empty(),
        "get_doc() != is_empty()"
    );
    assert!(output_data.hash().is_empty(), "hash() != is_empty()");
    assert!(
        output_data.created_at().is_none(),
        "created_at() != is_none()"
    );
    assert!(
        output_data.updated_at().is_none(),
        "updated_at() != is_none()"
    );
    assert!(output_data.obj_id()?.is_none(), "obj_id() != is_none()");
    assert!(!output_data.json()?.is_empty(), "json() == is_empty()");
    assert!(
        output_data.json_for_admin()?.is_some(),
        "json_for_admin() != is_some()"
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
