//! Generated type-state versions
//!
//! # Example
//! ```ignore
//! use froglight_common::version::Version;
//!
//! #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Version)]
//! #[version(protocol = 767, resource = 34, feature = "v1_21_1")]
//! pub struct V1_21_1;
//!
//! // |
//! // V
//!
//! #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Version)]
//! pub struct V1_21_1;
//!
//! #[cfg(feature = "v1_21_1")]
//! impl super::Version for V1_21_1 {
//!     const PROTOCOL_ID: u32 = 767;
//!     const RESOURCE_VERSION: u32 = 34;
//! }
//! ```

/// Minecraft 1.21.4
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V1_21_4;

#[cfg(feature = "v1_21_4")]
impl super::Version for V1_21_4 {
    const PROTOCOL_ID: u32 = 769;
    const RESOURCE_VERSION: u32 = 46;
}

/// Minecraft 1.21.5
///
/// TODO: Update values when 1.21.5 is released
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct V1_21_5;

#[cfg(feature = "v1_21_5")]
impl super::Version for V1_21_5 {
    const PROTOCOL_ID: u32 = 770;
    const RESOURCE_VERSION: u32 = 47;
}
