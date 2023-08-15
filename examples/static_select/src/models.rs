use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: TextField,
    //
    //pub select_text: ChoiceTextField,
    pub select_text_mult: ChoiceTextMultField,
    //
    //pub select_i32: ChoiceI32Field,
    //pub select_i32_mult: SelectI32MultField,
    //
    //pub select_u32: ChoiceU32Field,
    //pub select_u32_mult: ChoiceI32MultField,
    //
    //pub select_i64: ChoiceI64Field,
    //pub select_i64_mult: ChoiceI64MultField,
    //
    //pub select_f64: ChoiceF64Field,
    //pub select_f64_mult: ChoiceF64MultField,
}

impl Control for User {
    fn custom() -> Self {
        Self {
            username: TextField {
                maxlength: 150,
                required: true,
                unique: true,
                ..Default::default()
            },
            select_text_mult: ChoiceTextMultField {
                required: true,
                default: Some(vec!["windows".into(), "linux".into(), "mac os".into()]),
                choices: vec![
                    ("windows".into(), "Windows".into()),
                    ("linux".into(), "Linux".into()),
                    ("mac os".into(), "Mac OS".into()),
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
