//! Math functions that match Java's standard library.
#![allow(clippy::unreadable_literal)]

/// Convert degrees to radians.
///
/// Matches the behavior of Java's standard library.
#[must_use]
pub const fn to_radians(degrees: f64) -> f64 { degrees * 0.017453292519943295 }

/// Convert radians to degrees.
///
/// Matches the behavior of Java's standard library.
#[must_use]
pub const fn to_degrees(radians: f64) -> f64 { radians * 57.29577951308232 }
