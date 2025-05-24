//! Precomputed sine and cosine tables.

use core::f64::consts::PI;

use bevy_platform::sync::LazyLock;

/// The smallest value that can be considered "zero"
/// when comparing precomputed angles using [`sin`] and [`cos`].
pub const EPSILON: f32 = 1.0E-7;

/// A precomputed sine table for angles in the range `[0, 2π)`
/// with a resolution of `2π / 65536`.
#[cfg(feature = "std")]
#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub static SIN: LazyLock<[f32; 65536]> =
    LazyLock::new(|| core::array::from_fn(|i| f64::sin((i as f64 * 2.0 * PI) / 65536.0) as f32));

/// A precomputed sine table for angles in the range `[0, 2π)`
/// with a resolution of `2π / 65536`.
#[cfg(all(feature = "libm", not(feature = "std")))]
#[expect(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub static SIN: LazyLock<[f32; 65536]> =
    LazyLock::new(|| core::array::from_fn(|i| libm::sin((i as f64 * 2.0 * PI) / 65536.0) as f32));

// -------------------------------------------------------------------------------------------------

/// Calculate the sine of an angle in radians using the [`SIN`] table.
#[must_use]
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn sin(x: f32) -> f32 {
    let x = x * 10430.378;
    let x = x as i32 as usize & 65535;
    debug_assert!(x < 65535, "x must be in the range [0, 65535), got: {x}");

    SIN[x]
}

/// Calculate the cosine of an angle in radians using the [`SIN`] table.
#[must_use]
#[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn cos(x: f32) -> f32 {
    let x = x * 10430.378 + 16384.0;
    let x = x as i32 as usize & 65535;
    debug_assert!(x < 65535, "x must be in the range [0, 65535), got: {x}");

    SIN[x]
}
