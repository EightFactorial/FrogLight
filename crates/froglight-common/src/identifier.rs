use smol_str::SmolStr;

/// A namespaced identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(SmolStr);

impl Identifier {
    const DEFAULT_NAMESPACE: &'static str = "minecraft";

    /// Create a new [`Identifier`].
    ///
    /// # Panics
    /// Panics if the string is not a valid identifier.
    ///
    /// To handle potential invalid identifiers,
    /// use [`Identifier::try_new`] instead.
    #[must_use]
    pub fn new(content: &(impl AsRef<str> + ?Sized)) -> Self {
        Self::try_new(content).expect("Invalid identifier")
    }

    /// Try to create a new [`Identifier`].
    ///
    /// Returns `None` if the string is not a valid identifier.
    pub fn try_new(content: &(impl AsRef<str> + ?Sized)) -> Option<Self> {
        let content = content.as_ref().trim();

        // Check if the string is empty or contains non-ASCII characters
        if content.is_empty() || !content.is_ascii() {
            return None;
        }
        // Check if the string starts or ends with a non-alphanumeric character
        if !content.as_bytes()[0].is_ascii_alphanumeric()
            || !content.as_bytes()[content.len() - 1].is_ascii_alphanumeric()
        {
            return None;
        }

        match content.chars().filter(|c| c == &':').count() {
            // Append the default namespace
            0 => Some(Self(SmolStr::new(format!("{}:{content}", Self::DEFAULT_NAMESPACE)))),
            // Use the provided namespace
            1 => Some(Self(SmolStr::new(content))),
            _ => None,
        }
    }

    /// Create a new [`Identifier`] from a static string.
    ///
    /// # Panics
    /// Panics if the string is not a valid identifier.
    #[must_use]
    pub const fn const_new(content: &'static str) -> Self {
        assert!(!content.is_empty() && content.is_ascii(), "Empty or non-ASCII identifier");

        // Check if the string starts or ends with a colon
        assert!(
            content.as_bytes()[0] != b':' && content.as_bytes()[content.len() - 1] != b':',
            "Identifier cannot start or end with a colon"
        );
        // Check if the string starts or ends with a non-alphanumeric character
        assert!(
            content.as_bytes()[0].is_ascii_alphanumeric()
                && content.as_bytes()[content.len() - 1].is_ascii_alphanumeric(),
            "Identifier cannot start or end with a non-alphanumeric character"
        );

        // Check if the string contains more than one colon
        let mut found_separator = false;
        let mut index = 0;
        while index < content.len() {
            if content.as_bytes()[index] == b':' {
                if found_separator {
                    panic!("Identifier cannot contain more than one colon");
                } else {
                    found_separator = true;
                }
            }
            index += 1;
        }

        Self(SmolStr::new_static(content))
    }

    /// Get the namespace of the [`Identifier`].
    ///
    /// If you also need the path, consider [`Identifier::split`].
    #[inline]
    #[must_use]
    pub fn namespace(&self) -> &str { self.split().0 }

    /// Get the path of the [`Identifier`].
    ///
    /// If you also need the namespace, consider [`Identifier::split`].
    #[inline]
    #[must_use]
    pub fn path(&self) -> &str { self.split().1 }

    /// Split the [`Identifier`] into its `namespace` and `path`.
    #[must_use]
    #[expect(clippy::missing_panics_doc)]
    pub fn split(&self) -> (&str, &str) { self.0.split_once(':').unwrap() }
}

impl AsRef<SmolStr> for Identifier {
    fn as_ref(&self) -> &SmolStr { &self.0 }
}
impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str { &self.0 }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}
impl std::ops::Deref for Identifier {
    type Target = SmolStr;
    fn deref(&self) -> &Self::Target { &self.0 }
}
