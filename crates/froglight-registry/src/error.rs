//! Errors that occur while using registries.

/// There is no value for the specified key.
#[derive(Debug, thiserror::Error)]
#[error("There is no value for the specified key: \"{}\"", self.0)]
pub struct InvalidKeyError(pub String);

impl From<String> for InvalidKeyError {
    fn from(key: String) -> Self { Self(key) }
}

impl From<&str> for InvalidKeyError {
    fn from(key: &str) -> Self { Self(key.to_string()) }
}

impl From<InvalidKeyError> for String {
    fn from(err: InvalidKeyError) -> Self { err.0 }
}
