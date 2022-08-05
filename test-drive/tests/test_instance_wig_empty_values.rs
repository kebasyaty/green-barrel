use green_barrel::test_tool::del_test_db;
use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};
use std::error::Error;

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // =============================================================================================
    pub const PROJECT_NAME: &str = "project_name";
    // The unique key for this test.
    // To generate a key: https://randompasswordgen.com/
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    pub const UNIQUE_PROJECT_KEY: &str = "test0A9933d329N";
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
        #[serde(default)]
        #[field_attrs(widget = "checkBox")]
        pub checkbox: Option<bool>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputDate", min = "1970-01-01", max = "1970-03-01")]
        pub date: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputDateTime",
            min = "1970-01-01T00:00",
            max = "1970-03-01T00:00"
        )]
        pub datetime: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputFile")]
        pub file: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputImage")]
        pub image: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "radioF64",
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub radio_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberF64")]
        pub number_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeF64", min = 1.0, max = 12.0)]
        pub range_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenF64", min = 1.0, max = 12.0)]
        pub hidden_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "radioI32",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberI32")]
        pub number_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeI32", min = 1, max = 12)]
        pub range_i32: Option<i32>,
        //
        #[field_attrs(widget = "hiddenI32", min = 1, max = 12)]
        pub hidden_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "radioI64",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberI64")]
        pub number_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeI64", min = 1, max = 12)]
        pub range_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenI64", min = 1, max = 12)]
        pub hidden_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "radioU32",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "numberU32")]
        pub number_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "rangeU32", min = 1, max = 12)]
        pub range_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenU32", min = 1, max = 12)]
        pub hidden_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "radioText",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub radio_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectText",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextDyn")]
        pub select_text_dyn: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextMult",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text_mult: Option<Vec<String>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectTextMultDyn")]
        pub select_text_mult_dyn: Option<Vec<String>>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32Dyn")]
        pub select_i32_dyn: Option<i32>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32_mult: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI32MultDyn")]
        pub select_i32_mult_dyn: Option<Vec<i32>>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32Dyn")]
        pub select_u32_dyn: Option<u32>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32_mult: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectU32MultDyn")]
        pub select_u32_mult_dyn: Option<Vec<u32>>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64Dyn")]
        pub select_i64_dyn: Option<i64>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64_mult: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectI64MultDyn")]
        pub select_i64_mult_dyn: Option<Vec<i64>>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64",
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64Dyn")]
        pub select_f64_dyn: Option<f64>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64Mult",
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64_mult: Option<Vec<f64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "selectF64MultDyn")]
        pub select_f64_mult_dyn: Option<Vec<f64>>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputText", minlength = 2, maxlength = 60)]
        pub text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputSlug", slug_sources = r#"["email"]"#)]
        pub slug: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "hiddenText", minlength = 2, maxlength = 60)]
        pub hidden_text: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputColor")]
        pub color: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputEmail", maxlength = 320, unique = true)]
        pub email: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputPassword", minlength = 8, maxlength = 16)]
        pub password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputPhone")]
        pub phone: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputUrl")]
        pub url: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIP")]
        pub ip: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIPv4")]
        pub ipv4: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "inputIPv6")]
        pub ipv6: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(widget = "textArea")]
        pub textarea: Option<String>,
    }

    // Test migration
    // =============================================================================================
    // Model list
    pub fn model_list() -> Result<Vec<Meta>, Box<dyn std::error::Error>> {
        Ok(vec![TestModel::meta()?])
    }
    // Test, migration service `Mango`
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients
        MONGODB_CLIENT_STORE.write()?.insert(
            "default".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            models: model_list()?,
        };
        monitor.migrat()?;
        // Add metadata and widgects map to cache.
        TestModel::to_cache()?;
        //
        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_instance_wig_empty_values() -> Result<(), Box<dyn Error>> {
    // Run migration
    // =============================================================================================
    app_name::mango_migration()?;

    // Body of test
    // =============================================================================================
    type TestModel = app_name::TestModel;
    //
    // Test model instance.
    // *********************************************************************************************
    let mut test_model = TestModel {
        hash: Some(String::new()),
        checkbox: Some(true),
        //
        date: Some("1970-02-28".to_string()),
        datetime: Some("1970-02-28T00:00".to_string()),
        //
        file: Some(
            r#"{"path":"./media/hello_world.odt","url":"/media/hello_world.odt"}"#.to_owned(),
        ),
        image: Some(
            r#"{"path":"./media/no-image-found.png","url":"/media/no-image-found.png"}"#.to_owned(),
        ),
        //
        radio_f64: Some(1.1),
        number_f64: Some(1.1),
        range_f64: Some(1.1),
        hidden_f64: Some(1.1),
        //
        radio_i32: Some(1),
        number_i32: Some(1),
        range_i32: Some(1),
        hidden_i32: Some(1),
        //
        radio_i64: Some(1),
        number_i64: Some(1),
        range_i64: Some(1),
        hidden_i64: Some(1),
        //
        radio_u32: Some(1),
        number_u32: Some(1),
        range_u32: Some(1),
        hidden_u32: Some(1),
        //
        select_text: Some("Some text".to_string()),
        select_text_dyn: Some("Some text".to_string()),
        select_text_mult: Some(vec!["Some text".to_string(), "Some text".to_string()]),
        select_text_mult_dyn: Some(vec!["Some text".to_string(), "Some text".to_string()]),
        //
        select_i32: Some(1),
        select_i32_dyn: Some(1),
        select_i32_mult: Some(vec![1, 2]),
        select_i32_mult_dyn: Some(vec![1, 2]),
        //
        select_u32: Some(1),
        select_u32_dyn: Some(1),
        select_u32_mult: Some(vec![1, 2]),
        select_u32_mult_dyn: Some(vec![1, 2]),
        //
        select_i64: Some(1),
        select_i64_dyn: Some(1),
        select_i64_mult: Some(vec![1, 2]),
        select_i64_mult_dyn: Some(vec![1, 2]),
        //
        select_f64: Some(1.1),
        select_f64_dyn: Some(1.1),
        select_f64_mult: Some(vec![1.1, 2.2]),
        select_f64_mult_dyn: Some(vec![1.1, 2.2]),
        //
        radio_text: Some("Some text".to_string()),
        text: Some("Some text".to_string()),
        slug: Some(String::new()),
        hidden_text: Some("Some text".to_string()),
        color: Some("#ffffff".to_string()),
        email: Some("brebroiquippammei-6985@yopmail.com".to_string()),
        password: Some("12345678".to_string()),
        phone: Some("1-726-771-2413".to_string()),
        url: Some("https://developer.mozilla.org".to_string()),
        ip: Some("127.0.0.1".to_string()),
        ipv4: Some("127.0.0.1".to_string()),
        ipv6: Some("::ffff:7f00:1".to_string()),
        textarea: Some("Some text".to_string()),
        //
        ..Default::default()
    };
    //
    // Module: mango-orm/src/models/db_query_api/paladins.rs
    // *********************************************************************************************
    // Check
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.check(None)?;
    assert!(output_data.is_valid(), "Check - is_valid");
    assert!(output_data.hash().is_empty(), "Check - hash.is_empty");
    assert!(output_data.object_id().is_err(), "Check - object_id.is_err");
    assert!(
        test_model.get_created_at().is_empty(),
        "Check - get_created_at.is_empty"
    );
    assert!(
        test_model.get_updated_at().is_empty(),
        "Check - get_updated_at.is_empty"
    );
    assert!(output_data.get_doc().is_some(), "Check - get_doc.is_some");
    assert!(
        !output_data.get_doc().unwrap().is_empty(),
        "Check - get_updated_at.is_empty"
    );
    assert!(!output_data.to_wig().is_empty(), "Check - to_wig.is_empty");
    assert!(output_data.to_json().is_ok(), "Check - to_json.is_ok");
    assert!(
        !output_data.to_json()?.is_empty(),
        "Check - to_json.is_empty"
    );
    assert!(
        output_data.to_json_for_admin().is_ok(),
        "Check - to_json_for_admin.is_ok"
    );
    assert!(
        !output_data.to_json_for_admin()?.is_empty(),
        "Check - to_json_for_admin.is_empty"
    );
    assert!(
        output_data.to_html(None, None, None).is_ok(),
        "Check - to_html.is_ok"
    );
    assert!(
        !output_data.to_html(None, None, None)?.is_empty(),
        "Check - to_html.is_empty"
    );
    //
    //
    // Create document in database
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.save(None, None)?;
    assert!(output_data.is_valid(), "Check - is_valid");
    let hash_1 = output_data.hash();
    assert!(!output_data.hash().is_empty(), "Create doc - hash.is_empty");
    assert!(
        output_data.object_id().is_ok(),
        "Create doc - object_id.is_ok"
    );
    assert!(
        !test_model.get_created_at().is_empty(),
        "Create doc - get_created_at.is_empty"
    );
    assert!(
        !test_model.get_updated_at().is_empty(),
        "Create doc - get_updated_at.is_empty"
    );
    assert!(
        output_data.get_doc().is_none(),
        "Create doc - get_doc.is_some"
    );
    assert!(!output_data.to_wig().is_empty(), "Check - to_wig.is_empty");
    assert!(output_data.to_json().is_ok(), "Check - to_json.is_ok");
    assert!(
        !output_data.to_json()?.is_empty(),
        "Create doc - to_json.is_empty"
    );
    assert!(
        output_data.to_json_for_admin().is_ok(),
        "Create doc - to_json_for_admin.is_ok"
    );
    assert!(
        !output_data.to_json_for_admin()?.is_empty(),
        "Create doc - to_json_for_admin.is_empty"
    );
    assert!(
        output_data.to_html(None, None, None).is_ok(),
        "Create doc - to_html.is_ok"
    );
    assert!(
        !output_data.to_html(None, None, None)?.is_empty(),
        "Create doc - to_html.is_empty"
    );
    //
    //
    // Update document in database
    // ---------------------------------------------------------------------------------------------
    let output_data = test_model.save(None, None)?;
    assert!(output_data.is_valid(), "Update doc - is_valid");
    let hash_2 = output_data.hash();
    assert_eq!(hash_1, hash_2, "hash_1 == hash_2");
    assert!(!output_data.hash().is_empty(), "Update doc - hash.is_empty");
    assert!(
        output_data.object_id().is_ok(),
        "Update doc - object_id.is_ok"
    );
    assert!(
        !test_model.get_created_at().is_empty(),
        "Update doc - get_created_at.is_empty"
    );
    assert!(
        !test_model.get_updated_at().is_empty(),
        "Update doc - get_updated_at.is_empty"
    );
    assert!(
        output_data.get_doc().is_none(),
        "Update doc - get_doc.is_some"
    );
    assert!(!output_data.to_wig().is_empty(), "Update doc - to_wig");
    assert!(output_data.to_json().is_ok(), "Update doc - to_json.is_ok");
    assert!(
        !output_data.to_json()?.is_empty(),
        "Update doc - to_json.is_empty"
    );
    assert!(
        output_data.to_json_for_admin().is_ok(),
        "Update doc - to_json_for_admin.is_ok"
    );
    assert!(
        !output_data.to_json_for_admin()?.is_empty(),
        "Update doc - to_json_for_admin.is_empty"
    );
    assert!(
        output_data.to_html(None, None, None).is_ok(),
        "Update doc - to_html.is_ok"
    );
    assert!(
        !output_data.to_html(None, None, None)?.is_empty(),
        "Update doc - to_html.is_empty"
    );

    // Delete test database
    // =============================================================================================
    del_test_db(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    //
    Ok(())
}
