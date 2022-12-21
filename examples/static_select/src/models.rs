use green_barrel::*;
use metamorphose::Model;
use serde::{Deserialize, Serialize};

use crate::settings::{
    accounts::SERVICE_NAME, APP_NAME, DATABASE_NAME, DB_QUERY_DOCS_LIMIT, UNIQUE_APP_KEY,
};

#[Model()]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub username: InputText,
    //
    //pub select_text: SelectText,
    pub select_text_mult: SelectTextMult,
    //
    // pub select_i32: SelectI32,
    //pub select_i32_mult: SelectI32Mult,
    //
    // pub select_u32: SelectU32,
    //pub select_u32_mult: SelectI32Mult,
    //
    //pub select_i64: SelectI64,
    //pub select_i64_mult: SelectI64Mult,
    //
    //pub select_f64: SelectF64,
    //pub select_f64_mult: SelectF64Mult,
}

impl Control for User {
    fn custom_default() -> Self {
        Self {
            username: InputText {
                maxlength: 150,
                required: true,
                unique: true,
                ..Default::default()
            },
            select_text_mult: SelectTextMult {
                required: true,
                default: Some(vec!["windows".into(), "linux".into(), "mac os".into()]),
                options: vec![
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
