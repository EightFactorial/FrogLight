use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Manifest;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Version {
    Release {
        major: u8,
        minor: u8,
        patch: u8,
    },
    ReleaseCanidate {
        major: u8,
        minor: u8,
        patch: u8,
        rc: u8,
    },
    PreRelease {
        major: u8,
        minor: u8,
        patch: u8,
        pre: u8,
    },
    Snapshot {
        year: u8,
        week: u8,
        release: String,
    },
    Other(String),
}

impl Version {
    pub const fn new_release(major: u8, minor: u8, patch: u8) -> Self {
        Self::Release {
            major,
            minor,
            patch,
        }
    }

    pub const fn new_release_canidate(major: u8, minor: u8, patch: u8, rc: u8) -> Self {
        Self::ReleaseCanidate {
            major,
            minor,
            patch,
            rc,
        }
    }

    pub const fn new_pre_release(major: u8, minor: u8, patch: u8, pre: u8) -> Self {
        Self::PreRelease {
            major,
            minor,
            patch,
            pre,
        }
    }

    pub const fn new_snapshot(year: u8, week: u8, release: String) -> Self {
        Self::Snapshot {
            year,
            week,
            release,
        }
    }

    pub fn is_stable(&self) -> bool { matches!(self, Self::Release { .. }) }

    pub fn is_newer(&self, other: &Version, manifest: &Manifest) -> bool {
        let a = manifest.versions.iter().find(|v| &v.id == self).unwrap();
        let b = manifest.versions.iter().find(|v| &v.id == other).unwrap();
        a.release_time > b.release_time
    }

    pub fn is_same_or_newer(&self, other: &Version, manifest: &Manifest) -> bool {
        let a = manifest.versions.iter().find(|v| &v.id == self).unwrap();
        let b = manifest.versions.iter().find(|v| &v.id == other).unwrap();
        a.release_time >= b.release_time
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VersionError {
    #[error("Invalid version")]
    InvalidVersion,
    #[error("Snapshot version must be in the format: <year>w<week>.<release>")]
    ErrorSnapshot,
    #[error("Release Canidate version must be in the format: <major>.<minor>-rc<rc>")]
    ErrorReleaseCanidate,
    #[error("Pre Release version must be in the format: <major>.<minor>-pre<pre>")]
    ErrorPreRelease,
    #[error("Release version must be in the format: <major>.<minor>.<patch>")]
    ErrorRelease,
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(' ') || s.contains("RV") {
            return Ok(Version::Other(s.to_owned()));
        } else if let Some(c) = s.chars().next() {
            if !c.is_ascii_digit() {
                return Ok(Version::Other(s.to_owned()));
            }
        }

        match (s.find('w'), s.find("-rc"), s.find("-pre")) {
            (Some(1), None, None) | (Some(2), None, None) => {
                // Snapshot

                let mut split = s.split('w');
                let year = split
                    .next()
                    .ok_or(VersionError::ErrorSnapshot)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                let week_release = split.next().ok_or(VersionError::ErrorSnapshot)?;

                let mut chars = week_release.chars();

                let mut week = String::new();
                for c in chars.by_ref() {
                    if !c.is_ascii_digit() {
                        break;
                    } else {
                        week.push(c);
                    }
                }

                let week = week.parse().unwrap();
                let release = chars.collect();

                Ok(Self::Snapshot {
                    year,
                    week,
                    release,
                })
            }
            (None, Some(_), None) => {
                // Release Canidate

                let mut split = s.split("-rc");

                let major_minor: String = split
                    .next()
                    .ok_or(VersionError::ErrorReleaseCanidate)?
                    .to_string();
                let rc = split
                    .next()
                    .ok_or(VersionError::ErrorReleaseCanidate)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                let mut split = major_minor.split('.');
                let major = split
                    .next()
                    .ok_or(VersionError::ErrorReleaseCanidate)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let minor = split
                    .next()
                    .ok_or(VersionError::ErrorReleaseCanidate)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let patch = split
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                if split.next().is_some() {
                    return Err(VersionError::InvalidVersion);
                }

                Ok(Self::ReleaseCanidate {
                    major,
                    minor,
                    patch,
                    rc,
                })
            }
            (None, None, Some(_)) => {
                // Pre Release

                let mut split: std::str::Split<'_, &str> = s.split("-pre");

                let major_minor: String = split
                    .next()
                    .ok_or(VersionError::ErrorPreRelease)?
                    .to_string();
                let pre = split
                    .next()
                    .ok_or(VersionError::ErrorPreRelease)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                let mut split = major_minor.split('.');
                let major = split
                    .next()
                    .ok_or(VersionError::ErrorPreRelease)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let minor = split
                    .next()
                    .ok_or(VersionError::ErrorPreRelease)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let patch = split
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                Ok(Self::PreRelease {
                    major,
                    minor,
                    patch,
                    pre,
                })
            }
            (None, None, None) => {
                // Release

                let mut split = s.split('.');
                let major = split
                    .next()
                    .ok_or(VersionError::ErrorRelease)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let minor = split
                    .next()
                    .ok_or(VersionError::ErrorRelease)?
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;
                let patch = split
                    .next()
                    .unwrap_or("0")
                    .parse()
                    .map_err(|_| VersionError::InvalidVersion)?;

                if split.next().is_some() {
                    return Err(VersionError::InvalidVersion);
                }

                Ok(Self::Release {
                    major,
                    minor,
                    patch,
                })
            }
            _ => Ok(Self::Other(s.to_owned())),
        }
    }
}

impl TryFrom<String> for Version {
    type Error = VersionError;

    fn try_from(value: String) -> Result<Self, Self::Error> { Version::from_str(&value) }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::Release {
                major,
                minor,
                patch,
            } => write!(f, "{}.{}.{}", major, minor, patch),
            Version::ReleaseCanidate {
                major,
                minor,
                patch,
                rc,
            } => {
                write!(f, "{}.{}.{}-rc{}", major, minor, patch, rc)
            }
            Version::PreRelease {
                major,
                minor,
                patch,
                pre,
            } => {
                write!(f, "{}.{}.{}-pre{}", major, minor, patch, pre)
            }
            Version::Snapshot {
                year,
                week,
                release,
            } => write!(f, "{}w{:02}{}", year, week, release),
            Version::Other(other) => write!(f, "{}", other),
        }
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind: &'static str = match self {
            Version::Release { .. } => "Release",
            Version::ReleaseCanidate { .. } => "Release Canidate",
            Version::PreRelease { .. } => "Pre Release",
            Version::Snapshot { .. } => "Snapshot",
            Version::Other(_) => "Other",
        };

        write!(f, "{}({})", kind, self)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
