//! # Mango-ORM
//!
//! ORM-like API MongoDB for Rust.

pub mod forms;
pub mod macros;
pub mod migration;
pub mod models;
pub mod store;
pub mod widgets;

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    use mongodb::{
        options::{ClientOptions, StreamAddress},
        Client,
    };
    use regex::RegexBuilder;

    // Testing of Client
    // *********************************************************************************************
    // cargo test test_client -- --nocapture
    #[tokio::test]
    async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
        let client_options = ClientOptions::builder()
            .hosts(vec![StreamAddress {
                hostname: "localhost".into(),
                port: Some(27017),
            }])
            .build();

        let client: Client = Client::with_options(client_options)?;

        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        Ok(())
    }

    // Regular expressions
    // *********************************************************************************************
    #[test]
    fn regex_validate_color_code() {
        let re =
            RegexBuilder::new(r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6})\b|(?:rgb|hsl)a?\([^\)]*\)$")
                .case_insensitive(true)
                .build()
                .unwrap();
        assert!(re.is_match("#f2f2f2"));
        assert!(re.is_match("#F2F2F2"));
        assert!(re.is_match("#fff"));
        assert!(re.is_match("rgb(255,0,24)"));
        assert!(re.is_match("rgb(255, 0, 24)"));
        assert!(re.is_match("rgba(255, 0, 24, .5)"));
        assert!(re.is_match("rgba(#fff, .5)"));
        assert!(re.is_match("rgba(#FFF, .5)"));
        assert!(re.is_match("hsl(120, 100%, 50%)"));
        assert!(re.is_match("hsla(170, 23%, 25%, 0.2 )"));
        assert!(re.is_match("0x00ffff"));
        assert!(re.is_match("0x00FFFF"));
        assert!(!re.is_match("#f2ewq"));
    }

    #[test]
    fn regex_validate_password() {
        let re = RegexBuilder::new(r"^[a-z0-9@#$%^&+=*!~)(]{8,}$")
            .case_insensitive(true)
            .build()
            .unwrap();
        assert!(!re.is_match("1234567"));
        assert!(!re.is_match(&"`".repeat(8)));
        assert!(!re.is_match(&"№".repeat(8)));
        assert!(!re.is_match(&" ".repeat(8)));
        assert!(!re.is_match(&"-".repeat(8)));
        assert!(!re.is_match(&"_".repeat(8)));
        assert!(!re.is_match(&":".repeat(8)));
        assert!(!re.is_match(&"'".repeat(8)));
        assert!(!re.is_match(&"\"".repeat(8)));
        assert!(!re.is_match(&",".repeat(8)));
        assert!(!re.is_match(&".".repeat(8)));
        assert!(!re.is_match(&"<".repeat(8)));
        assert!(!re.is_match(&">".repeat(8)));
        assert!(!re.is_match(&"?".repeat(8)));
        assert!(!re.is_match(&"/".repeat(8)));
        assert!(!re.is_match(&"  ".repeat(8)));
        assert!(re.is_match(&"zeDKs9LtfrB7Xm2"));
        assert!(re.is_match(&"@#$%^&+=*!~)("));
        assert!(re.is_match(&"0123456789"));
        assert!(re.is_match(&"abcdefghijklmnopqrstuvwxyz"));
        assert!(re.is_match(&"ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
    }

    #[test]
    fn regex_validate_datetime() {
        let re = RegexBuilder::new(
            r"^[\d]{4}-([0][1-9]|[1][0-2])-([0][1-9]|[1][0-9]|[2][0-9]|[3][0-1])T([0-1][0-9]|[2][0-3]):[0-5][0-9]:[0-5][0-9]$"
        ).build().unwrap();
        assert!(!re.is_match("0000-00-00T00:00:00"));
        assert!(!re.is_match("0000-13-01T00:00:00"));
        assert!(!re.is_match("0000-01-32T00:00:00"));
        assert!(!re.is_match("0000-01-01T24:00:00"));
        assert!(!re.is_match("0000-01-01T00:60:00"));
        assert!(!re.is_match("0000-01-01T00:00:60"));
        assert!(re.is_match("0000-12-31T00:00:00"));
        assert!(re.is_match("0000-01-01T00:00:00"));
        assert!(re.is_match("2020-10-15T11:17:49"));
    }
}
