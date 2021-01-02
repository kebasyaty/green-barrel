use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_YsVbsqXu3F8YVSp_";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
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
}

// TEST
// #################################################################################################
#[test]
fn test_form_with_default_values() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_i32").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i32_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_u32").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_u32_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_i64").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i64_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_f64").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );
    // select_f64_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_f64_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::form_key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validation
    assert!(result.bool());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_text_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_i32").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i32_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_u32").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_u32_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_i64").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_i64_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    assert!(map_wigets.get("select_f64").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );
    // select_f64_dyn
    let map_wigets = result.wig();
    assert!(map_wigets.get("select_f64_dyn").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::form_key()[..]).unwrap();
    }

    Ok(())
}
