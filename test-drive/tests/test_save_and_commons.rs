use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};
use std::error::Error;

mod data_test {
    use super::*;

    // Test application settings
    // =============================================================================================
    pub const PROJECT_NAME: &str = "project_name";
    // The unique key for this test.
    // To generate a key: https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    pub const UNIQUE_PROJECT_KEY: &str = "testA9x933d329Nf";
    //
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Models
    // =============================================================================================
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
                checkbox: CheckBox::default(),
                date: InputDate::default(),
                datetime: InputDateTime::default(),
                file: InputFile::default(),
                image: InputImage::default(),
                number_i32: NumberI32::default(),
                radio_i32: RadioI32::default(),
                range_i32: RangeI32::default(),
                number_u32: NumberU32::default(),
                radio_u32: RadioU32::default(),
                range_u32: RangeU32::default(),
                number_i64: NumberI64::default(),
                radio_i64: RadioI64::default(),
                range_i64: RangeI64::default(),
                number_f64: NumberF64::default(),
                radio_f64: RadioF64::default(),
                range_f64: RangeF64::default(),
                radio_text: RadioText::default(),
                select_text: SelectText::default(),
                select_text_dyn: SelectTextDyn::default(),
                select_text_mult: SelectTextMult::default(),
                select_text_mult_dyn: SelectTextMultDyn::default(),
                select_i32: SelectI32::default(),
                select_i32_dyn: SelectI32Dyn::default(),
                select_i32_mult: SelectI32Mult::default(),
                select_i32_mult_dyn: SelectI32MultDyn::default(),
                select_u32: SelectU32::default(),
                select_u32_dyn: SelectU32Dyn::default(),
                select_u32_mult: SelectI32Mult::default(),
                select_u32_mult_dyn: SelectU32MultDyn::default(),
                select_i64: SelectI64::default(),
                select_i64_dyn: SelectI64Dyn::default(),
                select_i64_mult: SelectI64Mult::default(),
                select_i64_mult_dyn: SelectI64MultDyn::default(),
                select_f64: SelectF64::default(),
                select_f64_dyn: SelectF64Dyn::default(),
                select_f64_mult: SelectF64Mult::default(),
                select_f64_mult_dyn: SelectF64MultDyn::default(),
                text: InputText::default(),
                slug: AutoSlug::default(),
                color: InputColor::default(),
                email: InputEmail::default(),
                password: InputPassword::default(),
                phone: InputPhone::default(),
                url: InputUrl::default(),
                ip: InputIP::default(),
                ipv4: InputIPv4::default(),
                ipv6: InputIPv6::default(),
                textarea: TextArea::default(),
                /*
                hash: HiddenHash::default(),
                created_at: HiddenDateTime::default(),
                updated_at: HiddenDateTime::default(),
                */
                ..Default::default()
            }
        }
    }

    // Test migration
    // =============================================================================================
    // Get metadata list
    pub fn get_metadata_list() -> Result<Vec<Meta>, Box<dyn Error>> {
        let metadata_list = vec![TestModel::meta()?];
        Ok(metadata_list)
    }

    // Migration
    pub fn run_migration() -> Result<(), Box<dyn Error>> {
        // Caching MongoDB clients.
        {
            let mut client_store = MONGODB_CLIENT_STORE.write()?;
            client_store.insert(
                "default".to_string(),
                mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
            );
        }

        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, get_metadata_list()?)?;

        // Monitor initialization.
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            metadata_list: get_metadata_list()?,
        };
        monitor.migrat()?;

        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_save_and_commons() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    data_test::run_migration()?;

    // Testing
    // =============================================================================================
    type TestModel = data_test::TestModel;
    //
    // No data
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new()?;
    let output_data = test_model.save(None, None)?;
    assert!(
        output_data.is_valid(),
        "is_valid(): {}",
        output_data.err_msg()
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
        !output_data.json_for_admin()?.is_empty(),
        "json_for_admin() == is_empty()"
    );
    test_model = output_data.update()?;
    assert!(
        test_model.slug.get().is_none(),
        "test_model.slug.get() != is_none()"
    );

    // With data
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new()?;
    test_model.checkbox.set(true);
    test_model.date.set("1900-01-31");
    test_model.datetime.set("1900-01-31T00:00");
    test_model.file.set(FileData {
        path: "./media/hello_world.odt".into(),
        url: "/media/hello_world.odt".into(),
        ..Default::default()
    });
    test_model.image.set(ImageData {
        path: "./media/no-image-found.png".into(),
        url: "/media/no-image-found.png".into(),
        ..Default::default()
    });
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
    test_model.phone.set("+1 202-918-2132");
    test_model.url.set("https://ru.wikipedia.org/wiki/URL");
    test_model.ip.set("192.168.123.132");
    test_model.ipv4.set("192.168.50.1");
    test_model.ipv6.set("1050:0:0:0:5:600:300c:326b");
    test_model.textarea.set("Some text");

    for _ in 1..=10 {
        let output_data = test_model.save(None, None)?;

        assert!(
            output_data.is_valid(),
            "is_valid(): {}",
            output_data.err_msg()
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
            !output_data.json_for_admin()?.is_empty(),
            "json_for_admin() == is_empty()"
        );
        test_model = output_data.update()?;
        assert!(
            test_model.slug.get().is_none(),
            "test_model.slug.get() != is_none()"
        );
    }

    // Delete test database
    // =============================================================================================
    del_test_db(
        data_test::PROJECT_NAME,
        data_test::UNIQUE_PROJECT_KEY,
        data_test::get_metadata_list()?,
    )?;

    Ok(())
}
