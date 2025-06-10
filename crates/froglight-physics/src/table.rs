//! Precomputed sine and cosine tables.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use core::f64::consts::PI;

use bevy_platform::sync::LazyLock;

/// The smallest value that can be considered "zero"
/// when comparing angles using [`sin`] and [`cos`].
pub const TABLE_EPSILON: f32 = 1.0E-4;

// -------------------------------------------------------------------------------------------------

/// A precomputed sine table for angles in the range `[0, 2π)`.
#[cfg(not(all(feature = "std", feature = "nightly")))]
pub static SIN: LazyLock<[f32; 65536]> =
    LazyLock::new(|| core::array::from_fn(|i| SINFN((i as f64 * 2.0 * PI) / 65536.0) as f32));

/// The sine function from the standard library.
#[cfg(all(not(all(feature = "std", feature = "nightly")), feature = "std"))]
const SINFN: fn(f64) -> f64 = f64::sin;

/// The sine function from the `libm` crate.
#[cfg(all(not(all(feature = "std", feature = "nightly")), not(feature = "std"), feature = "libm"))]
const SINFN: fn(f64) -> f64 = libm::sin;

/// Fail to compile if neither the `std` nor `libm` features are enabled.
#[cfg(all(
    not(all(feature = "std", feature = "nightly")),
    not(feature = "std"),
    not(feature = "libm")
))]
const SINFN: fn(f64) -> f64 = compile_error!(
    "Either the `std` or `libm` feature must be enabled to use the precomputed sine table."
);

// -------------------------------------------------------------------------------------------------

/// A precomputed sine table for angles in the range `[0, 2π)`.
///
/// Uses SIMD intrinsics to compute the sine values for better performance.
#[cfg(all(feature = "std", feature = "nightly"))]
pub static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
    use std::simd::{Simd, StdFloat};
    const BATCH: usize = 64;

    let mut array = core::array::from_fn::<f64, 65536, _>(|i| (i as f64 * 2.0 * PI) / 65536.0);
    for chunk in array.array_chunks_mut::<BATCH>() {
        *chunk = Simd::<f64, BATCH>::from_array(*chunk).sin().to_array();
    }
    array.map(|f| f as f32)
});

// -------------------------------------------------------------------------------------------------

/// Calculate the sine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin(x: f32) -> f32 {
    let x = x * 10430.378;
    let x = x as i32 as usize & 65535;
    debug_assert!(x <= 65535, "x must be in the range [0, 65535], got: {x}");

    SIN[x]
}

/// Calculate the cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn cos(x: f32) -> f32 {
    let x = x * 10430.378 + 16384.0;
    let x = x as i32 as usize & 65535;
    debug_assert!(x <= 65535, "x must be in the range [0, 65535], got: {x}");

    SIN[x]
}

/// Calculate the sine and cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin_cos(x: f32) -> (f32, f32) {
    let (x, y) = (x * 10430.378, x * 10430.378 + 16384.0);
    let (x, y) = ((x as i32 as usize & 65535), (y as i32 as usize & 65535));
    debug_assert!(
        x <= 65535 && y <= 65535,
        "x and y must be in the range [0, 65535], got: {x}, {y}"
    );

    (SIN[x], SIN[y])
}

// -------------------------------------------------------------------------------------------------

#[test]
fn test() {
    use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8};
    fn assert(input: f32, expected: f32) {
        let sin = sin(input);
        let diff = (sin - expected).abs();
        assert!(diff < TABLE_EPSILON, "{sin} != {expected} (input: {input}, diff: {diff})");
    }

    // 0, 90, 180, 270, 360 degrees
    assert(0.0 * FRAC_PI_2, 0.0); // sin(0 degrees)
    assert(1.0 * FRAC_PI_2, 1.0); // sin(90 degrees)
    assert(2.0 * FRAC_PI_2, 0.0); // sin(180 degrees)
    assert(3.0 * FRAC_PI_2, -1.0); // sin(270 degrees)
    assert(4.0 * FRAC_PI_2, 0.0); // sin(360 degrees)

    // 45, 135, 225, 315 degrees
    assert(1.0 * FRAC_PI_4, 0.70710677); // sin(45 degrees)
    assert(3.0 * FRAC_PI_4, 0.70710677); // sin(135 degrees)
    assert(5.0 * FRAC_PI_4, -0.70710677); // sin(225 degrees)
    assert(7.0 * FRAC_PI_4, -0.70710677); // sin(315 degrees)

    // 22.5, 67.5, 112.5, 157.5, 202.5, 247.5, 292.5, 337.5 degrees
    assert(1.0 * FRAC_PI_8, 0.38268343); // sin(22.5 degrees)
    assert(3.0 * FRAC_PI_8, 0.9238795); // sin(67.5 degrees)
    assert(5.0 * FRAC_PI_8, 0.9238795); // sin(112.5 degrees)
    assert(7.0 * FRAC_PI_8, 0.38268343); // sin(157.5 degrees)
    assert(9.0 * FRAC_PI_8, -0.38268343); // sin(202.5 degrees)
    assert(11.0 * FRAC_PI_8, -0.9238795); // sin(247.5 degrees)
    assert(13.0 * FRAC_PI_8, -0.9238795); // sin(292.5 degrees)
    assert(15.0 * FRAC_PI_8, -0.38268343); // sin(337.5 degrees)
}

#[cfg(test)]
proptest::proptest! {
    #[test]
    fn arbitrary(data in -720.0f32..720.0f32) {
        // Note: Input is in radians, not degrees.
        let (std_sin, std_cos) = data.sin_cos();
        let (tbl_sin, tbl_cos) = sin_cos(data);

        let diff = (std_sin - tbl_sin).abs();
        assert!(diff < TABLE_EPSILON, "{tbl_sin} != {std_sin} (diff: {diff})");

        let diff = (std_cos - tbl_cos).abs();
        assert!(diff < TABLE_EPSILON, "{tbl_cos} != {std_cos} (diff: {diff})");
    }
}
