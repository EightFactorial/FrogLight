//! Errors that occur while using registries.

/// There is no value for the specified key.
#[derive(Debug, thiserror::Error)]
#[error("There is no value for the specified key: \"{}\"", self.0)]
pub struct InvalidKeyError(pub String);

impl From<String> for InvalidKeyError {
    fn from(key: String) -> Self { Self(key) }
}

impl<T: AsRef<str>> From<&T> for InvalidKeyError {
    fn from(key: &T) -> Self { Self(key.as_ref().to_string()) }
}

impl From<InvalidKeyError> for String {
    fn from(err: InvalidKeyError) -> Self { err.0 }
}
