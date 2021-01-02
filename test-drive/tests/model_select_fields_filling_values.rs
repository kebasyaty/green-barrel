use mango_orm::*;
use mango_orm::{migration::Monitor, test_tool::del_test_base};
use metamorphose::Model;
use mongodb::{
    bson::{doc, oid::ObjectId},
    sync::Client,
};
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_NN3_hhLvf6uNfCAz";
    pub const DATABASE_NAME: &str = "TEST_3y_LJGf6cTsw4PDX";
    pub const DB_CLIENT_NAME: &str = "TEST_default_YFmhGu2_vTmm3vyM";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    // Test keyword for for test technical database
    // ( Valid characters: _ a-z A-Z 0-9 ; Size: 6-48 )
    pub static KEYWORD: &str = "TEST_bjUyy_n7ta1a5sZT";

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        // text
        #[serde(default)]
        #[field_attrs(
            widget = "selectText",
            default = "volvo",
            select = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextDyn",
            select = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text_dyn: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextMult",
            select = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text_mult: Option<Vec<String>>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextMultDyn",
            select = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text_mult_dyn: Option<Vec<String>>,
        // i32
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32",
            default = 1,
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32: Option<i32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32Dyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32_dyn: Option<i32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32Mult",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32_mult: Option<Vec<i32>>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32MultDyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32_mult_dyn: Option<Vec<i32>>,
        // u32
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32",
            default = 1,
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32: Option<u32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32Dyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32_dyn: Option<u32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32Mult",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32_mult: Option<Vec<u32>>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32MultDyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32_mult_dyn: Option<Vec<u32>>,
        // i64
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64",
            default = 1,
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64: Option<i64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64Dyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64_dyn: Option<i64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64Mult",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64_mult: Option<Vec<i64>>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64MultDyn",
            select = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64_mult_dyn: Option<Vec<i64>>,
        // f64
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64",
            default = 1.1,
            select = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64: Option<f64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64Dyn",
            select = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64_dyn: Option<f64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64Mult",
            select = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64_mult: Option<Vec<f64>>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64MultDyn",
            select = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64_mult_dyn: Option<Vec<f64>>,
    }

    // Test migration
    // *********************************************************************************************
    // Model list
    pub fn model_list() -> Result<Vec<Meta>, Box<dyn std::error::Error>> {
        Ok(vec![TestModel::meta()?])
    }
    // Test, migration service `Mango`
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients
        DB_MAP_CLIENT_NAMES.lock()?.insert(
            "TEST_default_YFmhGu2_vTmm3vyM".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_base(KEYWORD, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            keyword: KEYWORD,
            // Register models
            models: model_list()?,
        };
        monitor.migrat();
        //
        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_with_default_values() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        // text
        select_text: Some("audi".to_string()),
        select_text_dyn: Some("audi".to_string()),
        select_text_mult: Some(vec!["saab".to_string(), "audi".to_string()]),
        select_text_mult_dyn: Some(vec!["saab".to_string(), "audi".to_string()]),
        // i32
        select_i32: Some(4),
        select_i32_dyn: Some(4),
        select_i32_mult: Some(vec![2, 4]),
        select_i32_mult_dyn: Some(vec![2, 4]),
        // u32
        select_u32: Some(4),
        select_u32_dyn: Some(4),
        select_u32_mult: Some(vec![2, 4]),
        select_u32_mult_dyn: Some(vec![2, 4]),
        // i64
        select_i64: Some(4),
        select_i64_dyn: Some(4),
        select_i64_mult: Some(vec![2, 4]),
        select_i64_mult_dyn: Some(vec![2, 4]),
        // f64
        select_f64: Some(4.4),
        select_f64_dyn: Some(4.4),
        select_f64_mult: Some(vec![2.2, 4.4]),
        select_f64_mult_dyn: Some(vec![2.2, 4.4]),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_dyn
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_dyn").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_text_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_text_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_text_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult_dyn").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_dyn").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_i32_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_i32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_i32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult_dyn").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_dyn").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_u32_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_u32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_u32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult_dyn").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_dyn").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_i64_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_i64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_i64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult_dyn").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );
    // select_f64_dyn
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_dyn").unwrap().options
    );
    // select_f64_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
    );
    // select_f64_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_f64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_f64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult_dyn").unwrap().options
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::model_key()[..])
            .unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        // text
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_text"));
        assert_eq!("audi", doc.get_str("select_text")?);
        assert_eq!("audi", doc.get_str("select_text_dyn")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("saab".to_string()),
                mongodb::bson::Bson::String("audi".to_string())
            ],
            doc.get_array("select_text_mult")?
        );
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("saab".to_string()),
                mongodb::bson::Bson::String("audi".to_string())
            ],
            doc.get_array("select_text_mult_dyn")?
        );

        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i32"));
        assert_eq!(4, doc.get_i32("select_i32")?);
        assert_eq!(4, doc.get_i32("select_i32_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult_dyn")?
        );
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_u32"));
        assert_eq!(4, doc.get_i64("select_u32")?);
        assert_eq!(4, doc.get_i64("select_u32_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult_dyn")?
        );
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i64"));
        assert_eq!(4, doc.get_i64("select_i64")?);
        assert_eq!(4, doc.get_i64("select_i64_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult_dyn")?
        );
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_f64"));
        assert_eq!(4.4, doc.get_f64("select_f64")?);
        assert_eq!(4.4, doc.get_f64("select_f64_dyn")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult")?
        );
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult_dyn")?
        );
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.bool(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_dyn
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_dyn").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_text_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_text_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_text_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult_dyn").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_dyn").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_i32_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_i32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_i32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult_dyn").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_dyn").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_u32_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_u32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_u32_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult_dyn").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_dyn
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_dyn").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_i64_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_i64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_i64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult_dyn").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );
    // select_f64_dyn
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64_dyn").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_dyn").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_dyn").unwrap().options
    );
    // select_f64_mult
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
    );
    // select_f64_mult_dyn
    let map_wigets = result.wig();
    assert!(map_wigets
        .get("select_f64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets
        .get("select_f64_mult_dyn")
        .unwrap()
        .value
        .is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult_dyn").unwrap().options
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.lock()?;
        let client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let form_cache: &FormCache = form_store
            .get(&app_name::TestModel::model_key()[..])
            .unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        // text
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_text"));
        assert_eq!("audi", doc.get_str("select_text")?);
        assert_eq!("audi", doc.get_str("select_text_dyn")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("saab".to_string()),
                mongodb::bson::Bson::String("audi".to_string())
            ],
            doc.get_array("select_text_mult")?
        );
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("saab".to_string()),
                mongodb::bson::Bson::String("audi".to_string())
            ],
            doc.get_array("select_text_mult_dyn")?
        );

        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i32"));
        assert_eq!(4, doc.get_i32("select_i32")?);
        assert_eq!(4, doc.get_i32("select_i32_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult_dyn")?
        );
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_u32"));
        assert_eq!(4, doc.get_i64("select_u32")?);
        assert_eq!(4, doc.get_i64("select_u32_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult_dyn")?
        );
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i64"));
        assert_eq!(4, doc.get_i64("select_i64")?);
        assert_eq!(4, doc.get_i64("select_i64_dyn")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult")?
        );
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult_dyn")?
        );
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_f64"));
        assert_eq!(4.4, doc.get_f64("select_f64")?);
        assert_eq!(4.4, doc.get_f64("select_f64_dyn")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult")?
        );
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult_dyn")?
        );
    }

    // ---------------------------------------------------------------------------------------------
    del_test_base(app_name::KEYWORD, &app_name::model_list()?)?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
