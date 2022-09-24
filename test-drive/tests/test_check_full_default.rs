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
    pub const UNIQUE_PROJECT_KEY: &str = "test49E57Rtbn94";
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
fn test_check_full_default() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    data_test::run_migration()?;

    // Testing
    // =============================================================================================
    type TestModel = data_test::TestModel;
    //
    // Positive
    // ---------------------------------------------------------------------------------------------
    let mut test_model = TestModel::new()?;
    let output_data = test_model.check(None)?;
    assert!(
        output_data.is_valid(),
        "is_valid(): {}",
        output_data.err_msg()
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
        !output_data.json_for_admin()?.is_empty(),
        "json_for_admin() == is_empty()"
    );

    // Delete test database
    // =============================================================================================
    del_test_db(
        data_test::PROJECT_NAME,
        data_test::UNIQUE_PROJECT_KEY,
        data_test::get_metadata_list()?,
    )?;

    Ok(())
}
