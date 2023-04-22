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
    //pub select_text_dyn: ChoiceTextDyn,
    pub select_text_mult_dyn: ChoiceTextMultDyn,
    //pub select_i32_dyn: ChoiceI32Dyn,
    //pub select_i32_mult_dyn: ChoiceI32MultDyn,
    //pub select_u32_dyn: ChoiceU32Dyn,
    //pub select_u32_mult_dyn: ChoiceU32MultDyn,
    //pub select_i64_dyn: ChoiceI64Dyn,
    //pub select_i64_mult_dyn: ChoiceI64MultDyn,
    //pub select_f64_dyn: ChoiceF64Dyn,
    //pub select_f64_mult_dyn: ChoiceF64MultDyn,
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
            ..Default::default()
        }
    }
}
