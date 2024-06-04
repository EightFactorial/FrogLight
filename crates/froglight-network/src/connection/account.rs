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
    // TODO: Auth token
    // pub auth_token: Option<CompactString>,
}

impl Default for AccountInformation {
    fn default() -> Self { Self::new_offline("froglight") }
}

impl AccountInformation {
    /// Creates a new account with the given username and UUID.
    ///
    /// If the account will be used offline, use
    /// [`AccountInformation::new_offline`].
    #[must_use]
    pub fn new(username: impl Into<CompactString>, uuid: Uuid) -> Self {
        Self { username: username.into(), uuid }
    }

    /// Set the auth token for the account.
    #[must_use]
    pub fn with_token(self, _token: impl Into<CompactString>) -> Self {
        // self.auth_token = Some(CompactString::new(token.as_ref()));
        self
    }

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
    #[must_use]
    #[inline]
    pub fn new_offline(username: &(impl AsRef<str> + ?Sized)) -> Self {
        Self::new(username.as_ref(), Self::offline_uuid(username))
    }

    /// The prefix for offline player UUIDs.
    pub const OFFLINE_PREFIX: &'static str = "OfflinePlayer:";

    /// Creates an offline UUID from a username.
    #[must_use]
    pub fn offline_uuid(username: &(impl AsRef<str> + ?Sized)) -> Uuid {
        let mut hash = Md5::new();
        hash.update(Self::OFFLINE_PREFIX.as_bytes());
        hash.update(username.as_ref().as_bytes());

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
