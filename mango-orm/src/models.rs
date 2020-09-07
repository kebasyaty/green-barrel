//! # Models
//!
//! Abstract Model methods for creating collections and interacting with the database.

// MODELS ==========================================================================================
/// Metadata
pub struct Meta {
    pub collection: &'static str,
}
/// Abstract Model ---------------------------------------------------------------------------------
pub trait Model {
    fn meta() -> Meta;
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
