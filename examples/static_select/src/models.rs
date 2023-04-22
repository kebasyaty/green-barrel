use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: Text,
    //
    //pub select_text: ChoiceText,
    pub select_text_mult: ChoiceTextMult,
    //
    //pub select_i32: ChoiceI32,
    //pub select_i32_mult: SelectI32Mult,
    //
    //pub select_u32: ChoiceU32,
    //pub select_u32_mult: ChoiceI32Mult,
    //
    //pub select_i64: ChoiceI64,
    //pub select_i64_mult: ChoiceI64Mult,
    //
    //pub select_f64: ChoiceF64,
    //pub select_f64_mult: ChoiceF64Mult,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: Text {
                maxlength: 150,
                required: true,
                unique: true,
                ..Default::default()
            },
            select_text_mult: ChoiceTextMult {
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
