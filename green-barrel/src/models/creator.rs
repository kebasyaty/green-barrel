//! For describe a model with user defaults.

pub trait Creator {
    fn custom_default() -> Self;
}
