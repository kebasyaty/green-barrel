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
    pub const UNIQUE_APP_KEY: &str = "ee09f3h6GBP08d2r";
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
        pub checkbox: CheckBox,
        //
        pub date: Date,
        pub datetime: DateTime,
        //
        pub file: File,
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
        pub slug: Slug,
        pub color: Color,
        pub email: Email,
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
async fn test_check_full_default() -> Result<(), Box<dyn Error>> {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    //
    migration::run_migration(&client).await?;

    // =============================================================================================
    type TestModel = models::TestModel;
    //
    // No data
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

    // Add data
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new().await?;
    test_model.checkbox.set(true);
    test_model.date.set("0000-01-01");
    test_model.datetime.set("2039-01-01T00:00");
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
    test_model.email.set("jane32@enhanceronly.com");
    test_model.password.set("j2972K4R3uQeVFPF");
    test_model.phone.set("+1 202-918-2132");
    test_model.url.set("https://ru.wikipedia.org/wiki/URL");
    test_model.ip.set("192.168.123.132");
    test_model.ipv4.set("192.168.50.1");
    test_model.ipv6.set("1050:0:0:0:5:600:300c:326b");
    test_model.textarea.set("Some text");

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
