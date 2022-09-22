use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use mongodb::{bson::doc, sync::Client};
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
    pub const UNIQUE_PROJECT_KEY: &str = "testkC1n08ZfjgS";
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
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, &get_metadata_list()?)?;

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
fn test_model() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    data_test::run_migration()?;

    // Body of test
    // =============================================================================================
    type TestModel = data_test::TestModel;
    //
    // Module: mango-orm/src/models/caching.rs
    // ---------------------------------------------------------------------------------------------
    // new
    assert!(TestModel::new().is_ok(), "new() == is_ok()");
    // to_json
    assert!(!TestModel::json()?.is_empty(), "json() == is_empty()");
    // model_to_json_for_admin
    assert!(
        !TestModel::model_to_json_for_admin()?.is_empty(),
        "model_to_json_for_admin == is_empty()"
    );
    // Get cached Model data
    let _cache_data: (ModelCache, Client) = TestModel::get_cache_data_for_query()?;
    //
    //
    // Module: mango-orm/src/models/db_query_api/commons.rs
    // ---------------------------------------------------------------------------------------------
    // aggregate
    let pipeline = vec![doc! {}];
    let result = TestModel::aggregate(pipeline, None);
    assert!(result.is_err(), "aggregate() == is_err()");
    // count_documents
    let result = TestModel::count_documents(None, None)?;
    assert_eq!(result, 0, "count_documents() == 0_i64");
    // delete_many
    let query = doc! {};
    let result = TestModel::delete_many(query, None)?;
    assert!(result.is_valid(), "delete_many == is_valid()");
    assert!(
        result.err_msg().is_empty(),
        "delete_many() == err_msg() == is_empty()"
    );
    assert!(
        result.deleted_count()? == 0,
        "delete_many() == deleted_count"
    );
    // delete_one
    let query = doc! {};
    let result = TestModel::delete_one(query, None)?;
    assert!(result.is_valid(), "delete_one() == is_valid()");
    assert!(
        result.err_msg().is_empty(),
        "delete_one() == err_msg() == is_empty()"
    );
    assert!(
        result.deleted_count()? == 0,
        "delete_one() == deleted_count"
    );
    // distinct
    let field_name = "checkbox";
    let filter = doc! {};
    let result = TestModel::distinct(field_name, Some(filter), None)?;
    assert!(result.is_empty(), "distinct() == is_empty()");
    // estimated_document_count
    let result = TestModel::estimated_document_count(None)?;
    assert_eq!(result, 0, "estimated_document_count == 0_i64");
    // find_many_to_doc
    let result = TestModel::find_many_to_doc_list(None, None)?;
    assert!(result.is_none(), "find_many_to_doc_list() == is_none()");
    // find_many_to_json
    let result = TestModel::find_many_to_json(None, None)?;
    assert!(result.is_empty());
    // find_one_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one(filter, None)?;
    assert!(result.is_none(), "find_many_to_json() == is_none()");
    // find_one_to_json
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_json(filter, None)?;
    assert!(result.is_empty(), "find_one_to_json() == is_empty()");
    // find_one_to_model_instance
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_to_instance(filter, None)?;
    assert!(result.is_none(), "find_one_to_instance() == is_none()");
    // find_one_and_delete_to_doc
    let filter = doc! {"username": "user_1"};
    let result = TestModel::find_one_and_delete(filter, None)?;
    assert!(result.is_none(), "find_one_and_delete() == is_none()");
    // name
    let result = TestModel::name()?;
    assert!(!result.is_empty(), "name() == is_empty()");
    // namespace
    let result = TestModel::namespace()?;
    assert!(!result.db.is_empty(), "namespace() == is_empty()");
    assert!(!result.coll.is_empty(), "namespace() == is_empty()");

    // Delete test database
    // =============================================================================================
    del_test_db(
        data_test::PROJECT_NAME,
        data_test::UNIQUE_PROJECT_KEY,
        &data_test::get_metadata_list()?,
    )?;

    Ok(())
}
