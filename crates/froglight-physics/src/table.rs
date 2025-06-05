//! Precomputed sine and cosine tables.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    unused_imports
)]

use core::f64::consts::PI;

use bevy_platform::sync::LazyLock;

/// The smallest value that can be considered "zero"
/// when comparing angles using [`sin`] and [`cos`].
pub const EPSILON: f32 = 1.0E-7;

/// A precomputed sine table for angles in the range `[0, 2π)`.
pub static SIN: LazyLock<[f32; 65536]> =
    LazyLock::new(|| core::array::from_fn(|i| SINFN((i as f64 * 2.0 * PI) / 65536.0) as f32));

// -------------------------------------------------------------------------------------------------

/// The sine function from the standard library.
#[cfg(feature = "std")]
const SINFN: fn(f64) -> f64 = f64::sin;

/// The sine function from the `libm` crate.
#[cfg(all(feature = "libm", not(feature = "std")))]
const SINFN: fn(f64) -> f64 = libm::sin;

#[cfg(not(any(feature = "std", feature = "libm")))]
const SINFN: fn(f64) -> f64 = compile_error!(
    "Either the `std` or `libm` feature must be enabled to use the precomputed sine table."
);

// -------------------------------------------------------------------------------------------------

/// Calculate the sine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin(x: f32) -> f32 {
    let x = x * 10430.378;
    let x = x as i32 as usize & 65535;
    debug_assert!(x < 65535, "x must be in the range [0, 65535), got: {x}");

    SIN[x]
}

/// Calculate the cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn cos(x: f32) -> f32 {
    let x = x * 10430.378 + 16384.0;
    let x = x as i32 as usize & 65535;
    debug_assert!(x < 65535, "x must be in the range [0, 65535), got: {x}");

    SIN[x]
}

/// Calculate the sine and cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin_cos(x: f32) -> (f32, f32) {
    let (x, y) = (x * 10430.378, x * 10430.378 + 16384.0);
    let (x, y) = ((x as i32 as usize & 65535), (y as i32 as usize & 65535));
    debug_assert!(x < 65535 && y < 65535, "x and y must be in the range [0, 65535), got: {x}, {y}");

    (SIN[x], SIN[y])
}
