use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "JGq8Q6vtg7ChUR_e";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            default = "Lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "hiddenText",
            default = "Lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub hidden: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "radioText", default = "Lorem ipsum")]
        pub radio: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputColor", default = "#000000")]
        pub color: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputEmail", maxlength = 74)]
        pub email: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPassword", minlength = 8)]
        pub password: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPhone")]
        pub phone: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputUrl")]
        pub url: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIP", default = "127.0.0.1")]
        pub ip: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv4", default = "127.0.0.1")]
        pub ipv4: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv6", default = "::ffff:7f00:1")]
        pub ipv6: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "textArea", default = "Lorem ipsum")]
        pub textarea: Option<String>,
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
    // text
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("text").unwrap().value);
    // hidden
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("hidden").unwrap().value);
    // radio
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("radio").unwrap().value);
    // color
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("color").unwrap().value);
    // email
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv4").unwrap().value);
    // ipv6
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv6").unwrap().value);
    // textarea
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("textarea").unwrap().value);

    // Validating values in database
    {
        let form_store = FORM_CACHE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // text
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("text").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // hidden
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("hidden").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    // radio
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("radio").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("color").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ip").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv4").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("ipv6").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("textarea").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_CACHE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
