//! # Operations with passwords.
//!
//! Trait:
//! `Password` - Operations with passwords.
//! Methods:
//! `create_password_hash` - Generate password hash and add to result document.
//!

use rand::Rng;

// Operations with passwords.
// *************************************************************************************************
pub trait Password {
    // Generate password hash and add to result document.
    // ---------------------------------------------------------------------------------------------
    fn create_password_hash(field_value: &str) -> Result<String, Box<dyn std::error::Error>> {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789@#$%^&+=*!~)(";
        const SALT_LEN: usize = 12;
        let mut rng = rand::thread_rng();
        let password: &[u8] = field_value.as_bytes();
        let salt: String = (0..SALT_LEN)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        let salt: &[u8] = salt.as_bytes();
        let config = argon2::Config::default();
        let hash: String = argon2::hash_encoded(password, salt, &config)?;
        Ok(hash)
    }
}
