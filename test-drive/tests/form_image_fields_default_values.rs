use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_SR7QfAW1L_jGCjtV";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(
            widget = "inputImage",
            default = r#"{
                "path":"./media/no-image-found.png",
                "url":"/media/no-image-found.png"
            }"#
        )]
        pub image: Option<String>,
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
    // image
    let map_wigets = result.wig();
    assert!(map_wigets.get("image").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/no-image-found.png","url":"/media/no-image-found.png"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("image").unwrap().value.as_str()
        )?
    );

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // image
    let map_wigets = result.wig();
    assert!(map_wigets.get("image").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/no-image-found.png","url":"/media/no-image-found.png"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("image").unwrap().value.as_str()
        )?
    );

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
