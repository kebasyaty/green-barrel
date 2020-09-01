/// Data types for the `value` field
#[derive(Debug, Clone)]
pub enum DataType {
    Text(String),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}
impl Default for DataType {
    fn default() -> Self {
        DataType::Text(String::new())
    }
}
impl DataType {
    pub fn get_data(&self) -> (&str, String) {
        match self {
            Self::Text(data) => ("String", data.to_string()),
            Self::I64(data) => ("I64", data.to_string()),
            Self::U64(data) => ("U64", data.to_string()),
            Self::F64(data) => ("F64", data.to_string()),
            Self::Bool(data) => ("Bool", data.to_string()),
        }
    }
}

fn main() {
    let x = DataType::F64(10.3);
    let x = x.get_data();

    let x = match x.0 {
        "String" => x.1,
        "I64" => x.1.parse().unwrap(),
        "U64" => x.1.parse().unwrap(),
        "F64" => x.1.parse().unwrap(),
        "Bool" => x.1.parse().unwrap(),
        _ => panic!("Incorrect data."),
    };

    println!("{}", x);
}
