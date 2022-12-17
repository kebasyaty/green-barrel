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
    //pub select_text_dyn: SelectTextDyn,
    pub select_text_mult_dyn: SelectTextMultDyn,
    //pub select_i32_dyn: SelectI32Dyn,
    //pub select_i32_mult_dyn: SelectI32MultDyn,
    //pub select_u32_dyn: SelectU32Dyn,
    //pub select_u32_mult_dyn: SelectU32MultDyn,
    //pub select_i64_dyn: SelectI64Dyn,
    //pub select_i64_mult_dyn: SelectI64MultDyn,
    //pub select_f64_dyn: SelectF64Dyn,
    //pub select_f64_mult_dyn: SelectF64MultDyn,
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
            ..Default::default()
        }
    }
}
