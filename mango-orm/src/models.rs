//! # Models
//!
//! `Meta` - Metadata of model (database name, collection name, etc).

// MODELS ==========================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
