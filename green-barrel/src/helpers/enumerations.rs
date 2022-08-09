//! Enumerations

use core::fmt::Debug;

/// To optimize the update_dyn_wig method.
// -------------------------------------------------------------------------------------------------
pub enum ControlArr<'a> {
    Text(Vec<&'a str>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    F64(Vec<f64>),
}
impl<'a> ControlArr<'a> {
    pub fn control_arr_str(&self) -> &Vec<&'a str> {
        match self {
            Self::Text(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_i32(&self) -> &Vec<i32> {
        match self {
            Self::I32(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_i64(&self) -> &Vec<i64> {
        match self {
            Self::I64(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
    pub fn control_arr_f64(&self) -> &Vec<f64> {
        match self {
            Self::F64(data) => data,
            _ => panic!("Invalid data type."),
        }
    }
}

/// The HTTP method to submit the form with. Possible (case insensitive) values: GET and POST.
/// Default -> HttpMethod::GET
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
}

impl Default for HttpMethod {
    fn default() -> Self {
        HttpMethod::GET
    }
}

impl HttpMethod {
    pub fn value(&self) -> String {
        match self {
            HttpMethod::GET => String::from("get"),
            HttpMethod::POST => String::from("post"),
        }
    }
}

/// If the value of the method attribute is post, enctype is the MIME type of the form submission.
/// Possible values: application/x-www-form-urlencoded | multipart/form-data | text/plain.
/// Default -> Enctype::Application
// -------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum Enctype {
    Application,
    Multipart,
    Text,
}

impl Default for Enctype {
    fn default() -> Self {
        Enctype::Application
    }
}

impl Enctype {
    pub fn value(&self) -> String {
        match self {
            Enctype::Application => String::from("application/x-www-form-urlencoded"),
            Enctype::Multipart => String::from("multipart/form-data"),
            Enctype::Text => String::from("text/plain"),
        }
    }
}
