//! For describe a model with user defaults.

pub trait Creator {
    fn create() -> Self;
}
