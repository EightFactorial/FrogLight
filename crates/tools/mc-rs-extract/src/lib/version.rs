use std::{num::ParseIntError, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

use crate::manifest::VersionManifest;

/// A Minecraft version.
///
/// This is a best-effort representation of a Minecraft version, and is not always accurate.
///
/// There are many different version formats, and this enum attempts to cover *most* of them.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use mc_rs_extract::Version;
///
/// assert_eq!(Version::from_str("1.20"), Ok(Version::Release { major: 1, minor: 20, patch: 0 }));
/// assert_eq!(Version::from_str("1.20.1"), Ok(Version::Release { major: 1, minor: 20, patch: 1 }));
/// assert_eq!(Version::from_str("1.20.2-rc2"), Ok(Version::ReleaseCandidate { major: 1, minor: 20, patch: 2, rc: 2 }));
/// assert_eq!(Version::from_str("1.20.3-pre3"), Ok(Version::PreRelease { major: 1, minor: 20, patch: 3, pre: 3 }));
/// assert_eq!(Version::from_str("1.20.4-foo"), Ok(Version::Other("1.20.4-foo".to_string())));
/// assert_eq!(Version::from_str("23w46a"), Ok(Version::Snapshot { year: 23, week: 46, rev: 'a' }));
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Version {
    /// A release version, e.g. `1.20.1`
    Release { major: u8, minor: u8, patch: u8 },
    /// A release candidate, e.g. `1.20.2-rc2`
    ReleaseCandidate {
        major: u8,
        minor: u8,
        patch: u8,
        rc: u8,
    },
    /// A pre-release version, e.g. `1.20.3-pre3`
    PreRelease {
        major: u8,
        minor: u8,
        patch: u8,
        pre: u8,
    },
    /// A snapshot version, e.g. `23w46a`
    Snapshot { year: u8, week: u8, rev: char },
    /// A version that doesn't fit into the above categories, e.g. `1.20.4-foo`
    Other(String),
}

impl Version {
    /// Create a new [`Version::Release`]
    pub const fn new_release(major: u8, minor: u8, patch: u8) -> Self {
        Version::Release {
            major,
            minor,
            patch,
        }
    }

    /// Returns `None` if the two versions are incomparable,
    /// otherwise returns `true` if `self` is less than or equal to `other`.
    ///
    /// # Examples
    /// ```rust
    /// use std::str::FromStr;
    /// use mc_rs_extract::Version;
    ///
    /// let v1_19_4 = Version::from_str("1.19.4").unwrap();
    /// let v1_20_0 = Version::from_str("1.20.0").unwrap();
    /// let v1_20_1 = Version::from_str("1.20.1").unwrap();
    ///
    /// assert_eq!(v1_19_4.lossy_le(&v1_19_4), Some(true));
    /// assert_eq!(v1_19_4.lossy_le(&v1_20_0), Some(true));
    /// assert_eq!(v1_19_4.lossy_le(&v1_20_1), Some(true));
    ///
    /// assert_eq!(v1_20_0.lossy_le(&v1_19_4), Some(false));
    /// assert_eq!(v1_20_0.lossy_le(&v1_20_0), Some(true));
    /// assert_eq!(v1_20_0.lossy_le(&v1_20_1), Some(true));
    ///
    /// assert_eq!(v1_20_1.lossy_le(&v1_19_4), Some(false));
    /// assert_eq!(v1_20_1.lossy_le(&v1_20_0), Some(false));
    /// assert_eq!(v1_20_1.lossy_le(&v1_20_1), Some(true));
    /// ```
    pub fn lossy_le(&self, other: &Version) -> Option<bool> {
        if self == other {
            Some(true)
        } else {
            self.lossy_lt(other)
        }
    }

    /// Returns `None` if the two versions are incomparable,
    /// otherwise returns `true` if `self` is less than `other`.
    ///
    /// # Examples
    /// ```rust
    /// use std::str::FromStr;
    /// use mc_rs_extract::Version;
    ///
    /// let v1_19_4 = Version::from_str("1.19.4").unwrap();
    /// let v1_20_0 = Version::from_str("1.20.0").unwrap();
    /// let v1_20_1 = Version::from_str("1.20.1").unwrap();
    ///
    /// assert_eq!(v1_19_4.lossy_lt(&v1_19_4), Some(false));
    /// assert_eq!(v1_19_4.lossy_lt(&v1_20_0), Some(true));
    /// assert_eq!(v1_19_4.lossy_lt(&v1_20_1), Some(true));
    ///
    /// assert_eq!(v1_20_0.lossy_lt(&v1_19_4), Some(false));
    /// assert_eq!(v1_20_0.lossy_lt(&v1_20_0), Some(false));
    /// assert_eq!(v1_20_0.lossy_lt(&v1_20_1), Some(true));
    ///
    /// assert_eq!(v1_20_1.lossy_lt(&v1_19_4), Some(false));
    /// assert_eq!(v1_20_1.lossy_lt(&v1_20_0), Some(false));
    /// assert_eq!(v1_20_1.lossy_lt(&v1_20_1), Some(false));
    /// ```
    pub fn lossy_lt(&self, other: &Version) -> Option<bool> {
        match (self, other) {
            (
                Version::Release {
                    major,
                    minor,
                    patch,
                },
                Version::Release {
                    major: other_major,
                    minor: other_minor,
                    patch: other_patch,
                },
            ) => Some(
                major < other_major
                    || (major == other_major && minor < other_minor)
                    || (major == other_major && minor == other_minor && patch < other_patch),
            ),
            (
                Version::ReleaseCandidate {
                    major,
                    minor,
                    patch,
                    rc,
                },
                Version::ReleaseCandidate {
                    major: other_major,
                    minor: other_minor,
                    patch: other_patch,
                    rc: other_rc,
                },
            ) => Some(
                major < other_major
                    || (major == other_major && minor < other_minor)
                    || (major == other_major && minor == other_minor && patch < other_patch)
                    || (major == other_major
                        && minor == other_minor
                        && patch == other_patch
                        && rc < other_rc),
            ),
            (
                Version::PreRelease {
                    major,
                    minor,
                    patch,
                    pre,
                },
                Version::PreRelease {
                    major: other_major,
                    minor: other_minor,
                    patch: other_patch,
                    pre: other_pre,
                },
            ) => Some(
                major < other_major
                    || (major == other_major && minor < other_minor)
                    || (major == other_major && minor == other_minor && patch < other_patch)
                    || (major == other_major
                        && minor == other_minor
                        && patch == other_patch
                        && pre < other_pre),
            ),
            (
                Version::Snapshot { year, week, rev },
                Version::Snapshot {
                    year: other_year,
                    week: other_week,
                    rev: other_rev,
                },
            ) => Some(
                year < other_year
                    || (year == other_year && week < other_week)
                    || (year == other_year && week == other_week && rev < other_rev),
            ),
            _ => None,
        }
    }

    /// Returns `None` if either version doesn't exist in the [`VersionManifest`],
    /// otherwise returns `true` if `self` is less than `other`.
    pub fn lt(&self, other: &Version, manifest: &VersionManifest) -> Option<bool> {
        let self_id = manifest.get(self)?;
        let other_id = manifest.get(other)?;
        Some(self_id.released_before(other_id))
    }

    /// Returns `None` if either version doesn't exist in the [`VersionManifest`],
    /// otherwise returns `true` if `self` is less than or equal to `other`.
    pub fn le(&self, other: &Version, manifest: &VersionManifest) -> Option<bool> {
        if self == other {
            Some(true)
        } else {
            self.lt(other, manifest)
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum VersionParseError {
    #[error("Invalid Version: {0}")]
    InvalidInteger(#[from] ParseIntError),
}

// Parse a string into a Version
impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Handle Release formats `1.20` and `1.20.0`
        if Regex::new(r"^\d+\.\d+$").unwrap().is_match(s)
            || Regex::new(r"^\d+\.\d+\.\d+$").unwrap().is_match(s)
        {
            let mut parts = s.split('.');
            let major = parts.next().expect("Release Major").parse()?;
            let minor = parts.next().expect("Release Minor").parse()?;
            let patch = parts.next().map(|s| s.parse()).unwrap_or(Ok(0))?;
            Ok(Version::Release {
                major,
                minor,
                patch,
            })
        } else if Regex::new(r"^\d+\.\d+\.\d+-rc\d+$").unwrap().is_match(s) {
            // Handle ReleaseCandidate format `1.20.0-rc1`
            let mut parts = s.split('-');
            let version = parts.next().expect("ReleaseCandidate Version");
            let rc = parts
                .next()
                .expect("ReleaseCandidate RC")
                .strip_prefix("rc")
                .expect("ReleaseCandidate RC")
                .parse()?;

            let mut parts = version.split('.');
            let major = parts.next().expect("ReleaseCandidate Major").parse()?;
            let minor = parts.next().expect("ReleaseCandidate Minor").parse()?;
            let patch = parts.next().expect("ReleaseCandidate Patch").parse()?;
            Ok(Version::ReleaseCandidate {
                major,
                minor,
                patch,
                rc,
            })
        } else if Regex::new(r"^\d+\.\d+\.\d+-pre\d+$").unwrap().is_match(s) {
            // Handle PreRelease format `1.20.0-pre1`
            let mut parts = s.split('-');
            let version = parts.next().expect("PreRelease Version");
            let pre = parts
                .next()
                .expect("PreRelease Pre")
                .strip_prefix("pre")
                .unwrap()
                .parse()?;

            let mut parts = version.split('.');
            let major = parts.next().expect("PreRelease Major").parse()?;
            let minor = parts.next().expect("PreRelease Minor").parse()?;
            let patch = parts.next().expect("PreRelease Patch").parse()?;
            Ok(Version::PreRelease {
                major,
                minor,
                patch,
                pre,
            })
        } else if Regex::new(r"^\d+w\d+[a-z]?$").unwrap().is_match(s) {
            // Handle Snapshot format `23w46a`
            let mut parts = s.split('w');
            let year = parts.next().expect("Snapshot Year").parse()?;

            let mut week = String::new();
            let mut rev = 'a';

            for c in parts.next().expect("Snapshot Week/Rev").chars() {
                if c.is_ascii_digit() {
                    week.push(c);
                } else {
                    rev = c;
                    break;
                }
            }

            Ok(Version::Snapshot {
                year,
                week: week.parse()?,
                rev,
            })
        } else {
            // Everything else
            Ok(Version::Other(s.to_string()))
        }
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Release {
                major,
                minor,
                patch,
            } => {
                write!(f, "Release {major}.{minor}.{patch}")
            }
            Version::ReleaseCandidate {
                major,
                minor,
                patch,
                rc,
            } => write!(f, "ReleaseCandidate {major}.{minor}.{patch}-rc{rc}"),
            Version::PreRelease {
                major,
                minor,
                patch,
                pre,
            } => write!(f, "PreRelease {major}.{minor}.{patch}-pre{pre}"),
            Version::Snapshot { year, week, rev } => write!(f, "Snapshot {year}w{week}{rev}"),
            Version::Other(s) => write!(f, "Other {s}"),
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Release {
                major,
                minor,
                patch,
            } => {
                write!(f, "{major}.{minor}.{patch}")
            }
            Version::ReleaseCandidate {
                major,
                minor,
                patch,
                rc,
            } => write!(f, "{major}.{minor}.{patch}-rc{rc}"),
            Version::PreRelease {
                major,
                minor,
                patch,
                pre,
            } => write!(f, "{major}.{minor}.{patch}-pre{pre}"),
            Version::Snapshot { year, week, rev } => write!(f, "{year}w{week}{rev}"),
            Version::Other(s) => write!(f, "{s}"),
        }
    }
}

// Implement Serialize for Version
impl Serialize for Version {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

// Implement Deserialize for Version
impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Version::from_str(&String::deserialize(deserializer)?).unwrap())
    }
}

#[test]
fn parser() {
    assert_eq!(
        Version::from_str("1.20"),
        Ok(Version::Release {
            major: 1,
            minor: 20,
            patch: 0
        })
    );
    assert_eq!(
        Version::from_str("1.20.0"),
        Ok(Version::Release {
            major: 1,
            minor: 20,
            patch: 0
        })
    );
    assert_eq!(
        Version::from_str("1.20.1"),
        Ok(Version::Release {
            major: 1,
            minor: 20,
            patch: 1
        })
    );
    assert_eq!(
        Version::from_str("1.20.2-rc2"),
        Ok(Version::ReleaseCandidate {
            major: 1,
            minor: 20,
            patch: 2,
            rc: 2
        })
    );
    assert_eq!(
        Version::from_str("1.20.3-pre3"),
        Ok(Version::PreRelease {
            major: 1,
            minor: 20,
            patch: 3,
            pre: 3
        })
    );
    assert_eq!(
        Version::from_str("1.20.4-foo"),
        Ok(Version::Other("1.20.4-foo".to_string()))
    );
    assert_eq!(
        Version::from_str("23w46a"),
        Ok(Version::Snapshot {
            year: 23,
            week: 46,
            rev: 'a'
        })
    );
}

#[test]
fn lossy_le() {
    let v1_19_4 = Version::from_str("1.19.4").unwrap();
    let v1_20_0 = Version::from_str("1.20.0").unwrap();
    let v1_20_1 = Version::from_str("1.20.1").unwrap();

    assert_eq!(v1_19_4.lossy_le(&v1_19_4), Some(true));
    assert_eq!(v1_19_4.lossy_le(&v1_20_0), Some(true));
    assert_eq!(v1_19_4.lossy_le(&v1_20_1), Some(true));

    assert_eq!(v1_20_0.lossy_le(&v1_19_4), Some(false));
    assert_eq!(v1_20_0.lossy_le(&v1_20_0), Some(true));
    assert_eq!(v1_20_0.lossy_le(&v1_20_1), Some(true));

    assert_eq!(v1_20_1.lossy_le(&v1_19_4), Some(false));
    assert_eq!(v1_20_1.lossy_le(&v1_20_0), Some(false));
    assert_eq!(v1_20_1.lossy_le(&v1_20_1), Some(true));
}

#[test]
fn lossy_lt() {
    let v1_19_4 = Version::from_str("1.19.4").unwrap();
    let v1_20_0 = Version::from_str("1.20.0").unwrap();
    let v1_20_1 = Version::from_str("1.20.1").unwrap();

    assert_eq!(v1_19_4.lossy_lt(&v1_19_4), Some(false));
    assert_eq!(v1_19_4.lossy_lt(&v1_20_0), Some(true));
    assert_eq!(v1_19_4.lossy_lt(&v1_20_1), Some(true));

    assert_eq!(v1_20_0.lossy_lt(&v1_19_4), Some(false));
    assert_eq!(v1_20_0.lossy_lt(&v1_20_0), Some(false));
    assert_eq!(v1_20_0.lossy_lt(&v1_20_1), Some(true));

    assert_eq!(v1_20_1.lossy_lt(&v1_19_4), Some(false));
    assert_eq!(v1_20_1.lossy_lt(&v1_20_0), Some(false));
    assert_eq!(v1_20_1.lossy_lt(&v1_20_1), Some(false));
}
