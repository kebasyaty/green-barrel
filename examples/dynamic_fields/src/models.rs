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
    //pub select_text_dyn: ChoiceTextDyn,
    pub select_text_mult_dyn: ChoiceTextMultDynField,
    //pub select_i32_dyn: ChoiceI32DynField,
    //pub select_i32_mult_dyn: ChoiceI32MultDynField,
    //pub select_u32_dyn: ChoiceU32DynField,
    //pub select_u32_mult_dyn: ChoiceU32MultDynField,
    //pub select_i64_dyn: ChoiceI64DynField,
    //pub select_i64_mult_dyn: ChoiceI64MultDynField,
    //pub select_f64_dyn: ChoiceF64DynField,
    //pub select_f64_mult_dyn: ChoiceF64MultDynField,
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
            ..Default::default()
        }
    }
}
