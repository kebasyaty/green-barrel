//! English

use serde_json::{json, Value};

pub fn message() -> Value {
    json!({
        1: "Invalid Color code",
        2: "Allowed chars",
        3: "Invalid Phone number",
        4: "Invalid email address",
        5: "Invalid Url",
        6: "Invalid IP address",
        7: "Invalid IPv4 address",
        8: "Invalid IPv6 address",
    })
}
