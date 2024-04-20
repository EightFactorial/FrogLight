use compact_str::CompactString;
use md5::{Digest, Md5};
use uuid::{Builder, Uuid};

/// Information about the account being used.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountInformation {
    /// The account's username.
    pub username: CompactString,
    /// The account's UUID.
    pub uuid: Uuid,
}

impl Default for AccountInformation {
    fn default() -> Self { Self::new_offline("froglight") }
}

impl AccountInformation {
    /// Creates a new account with an offline UUID.
    ///
    /// # Example
    /// ```rust
    /// use froglight_network::connection::AccountInformation;
    ///
    /// let account = AccountInformation::new_offline("froglight");
    /// assert_eq!(account.username.as_str(), "froglight");
    /// assert_eq!(account.uuid.to_string(), "8ee7f9a9-5c09-3373-8aeb-8aba0d9adeaa");
    /// ```
    pub fn new_offline(username: &(impl AsRef<str> + ?Sized)) -> Self {
        Self { username: CompactString::new(username.as_ref()), uuid: Self::offline_uuid(username) }
    }

    /// Creates an offline UUID from a username.
    #[must_use]
    pub fn offline_uuid(username: &(impl AsRef<str> + ?Sized)) -> Uuid {
        let mut hash = Md5::new();
        hash.update(format!("OfflinePlayer:{}", username.as_ref()).as_bytes());

        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&hash.finalize()[..16]);

        Builder::from_md5_bytes(bytes).into_uuid()
    }
}

/// TODO: Add more examples
#[test]
fn test_uuids() {
    assert_eq!(
        AccountInformation::offline_uuid("froglight").to_string(),
        "8ee7f9a9-5c09-3373-8aeb-8aba0d9adeaa"
    );
}
