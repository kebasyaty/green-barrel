use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST__qFbE9pfgAMD9fDd";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(
            widget = "inputDate",
            default = "1970-02-28",
            min = "1970-01-01",
            max = "1970-03-01",
            unique = true
        )]
        pub date: Option<String>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_with_filling_values() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        date: Some("1970-02-27".to_string()),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "1970-02-28".to_string(),
        map_wigets.get("date").unwrap().value
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
    // Validating
    assert!(result.bool());
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "1970-02-28".to_string(),
        map_wigets.get("date").unwrap().value
    );

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::form_key()[..]).unwrap();
    }

    Ok(())
}
