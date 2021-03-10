use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_e8pxg8A4ApGJuf_T";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "radioI32", default = -1)]
        pub radio: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "numberI32")]
        pub number: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "rangeI32", default = 5, min = 1, max = 12)]
        pub range: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "hiddenI32", default = 3, min = 1, max = 12)]
        pub hidden: Option<i32>,
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
    // radio
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        -1_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    // number
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    // hidden
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("hidden").unwrap().value.is_empty());

    // Validating cache
    {
        let form_store = FORM_CACHE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // radio
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        -1_i32,
        map_wigets.get("radio").unwrap().value.parse::<i32>()?
    );
    // number
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_i32,
        map_wigets.get("range").unwrap().value.parse::<i32>()?
    );
    // hidden
    let map_wigets = result.wig();
    assert!(map_wigets.get("hidden").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_i32,
        map_wigets.get("hidden").unwrap().value.parse::<i32>()?
    );

    // Validating cache
    {
        let form_store = FORM_CACHE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
