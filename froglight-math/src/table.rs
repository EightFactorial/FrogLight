//! Precomputed sine and cosine tables.
#![allow(clippy::cast_precision_loss, reason = "Desired behavior")]
#![allow(clippy::cast_possible_truncation, reason = "Desired behavior")]
#![allow(clippy::cast_sign_loss, reason = "Desired behavior")]

use core::f64::consts::PI;
#[cfg(feature = "std")]
use std::sync::LazyLock;

#[cfg(not(feature = "std"))]
use once_cell::sync::Lazy as LazyLock;

/// The smallest value that can be considered "zero"
/// when comparing angles using [`sin`] and [`cos`].
pub const EPSILON: f32 = 1.0E-4;

// -------------------------------------------------------------------------------------------------

/// A precomputed sine table for angles in the range `[0, 2π)`.
///
/// Used by the [`sin`], [`cos`], and [`sin_cos`] functions.
#[cfg(not(feature = "nightly"))]
pub static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
    /// The sine function from the standard library.
    #[cfg(feature = "std")]
    const SINFN: fn(f64) -> f64 = f64::sin;

    /// The sine function from the `libm` crate.
    #[cfg(not(feature = "std"))]
    const SINFN: fn(f64) -> f64 = libm::sin;

    // Normally we'd use `core::array::from_fn`,
    // but it seems to be causing a stack overflow in some cases.
    let mut array = [0.0f64; 65536];
    array.iter_mut().enumerate().for_each(|(i, f)| {
        *f = SINFN((i as f64 * 2.0 * PI) / 65536.0);
    });

    // Normally we'd use `array.map` and to convert to `f32`s directly,
    // but it seems to be causing a stack overflow in some cases.
    let mut output = [0.0f32; 65536];
    array.iter().zip(output.iter_mut()).for_each(|(i, o)| {
        *o = *i as f32;
    });
    output
});

// -------------------------------------------------------------------------------------------------

/// A precomputed sine table for angles in the range `[0, 2π)`.
///
/// Used by the [`sin`], [`cos`], and [`sin_cos`] functions.
///
/// Uses SIMD intrinsics for better performance (generates roughly 64x faster).
#[cfg(feature = "nightly")]
pub static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
    const BATCH: usize = 64;

    use core::simd::Simd;
    use std::simd::StdFloat;

    // Normally we'd use `core::array::from_fn`,
    // but it seems to be causing a stack overflow in some cases.
    let mut array = [0.0f64; 65536];
    array.iter_mut().enumerate().for_each(|(i, f)| {
        *f = (i as f64 * 2.0 * PI) / 65536.0;
    });

    let (chunks, remainder) = array.as_chunks_mut::<BATCH>();
    assert!(remainder.is_empty(), "Array length must be a multiple of BATCH!");
    for chunk in chunks {
        *chunk = Simd::<f64, BATCH>::from_array(*chunk).sin().to_array();
    }

    // Normally we'd use `array.map` and to convert to `f32`s directly,
    // but it seems to be causing a stack overflow in some cases.
    let mut output = [0.0f32; 65536];
    array.iter().zip(output.iter_mut()).for_each(|(i, o)| {
        *o = *i as f32;
    });
    output
});

// -------------------------------------------------------------------------------------------------

/// Calculate the sine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin(rad: f32) -> f32 {
    let x = rad * 10430.378;
    let x = x as i32 as usize & 65535;
    debug_assert!(x <= 65535, "x must be in the range [0, 65535], got: {x}");

    SIN[x]
}

/// Calculate the cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn cos(rad: f32) -> f32 {
    let x = rad * 10430.378 + 16384.0;
    let x = x as i32 as usize & 65535;
    debug_assert!(x <= 65535, "x must be in the range [0, 65535], got: {x}");

    SIN[x]
}

/// Calculate the sine and cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin_cos(rad: f32) -> (f32, f32) {
    let (x, y) = (rad * 10430.378, rad * 10430.378 + 16384.0);
    let (x, y) = ((x as i32 as usize & 65535), (y as i32 as usize & 65535));
    debug_assert!(x <= 65535 && y <= 65535, "x, y must be in the range [0, 65535], got: {x}, {y}");

    (SIN[x], SIN[y])
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use core::f32::consts::PI;

    #[cfg(feature = "std")]
    use proptest::prelude::*;

    use super::{EPSILON, sin};

    /// Tests for the sine and cosine functions for common angles.
    #[test]
    fn common() {
        use core::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8};
        fn assert(input: f32, expected: f32) {
            let sin = sin(input);
            let diff = (sin - expected).abs();
            assert!(diff < EPSILON, "{sin} != {expected} (input: {input}, diff: {diff})");
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

    #[cfg(feature = "std")]
    proptest::proptest! {
        #![proptest_config(ProptestConfig::with_cases(81920))]


        /// Test the sine and cosine functions against the standard library.
        #[test]
        fn arbitrary(data in (-PI * 200.0f32)..(PI * 200.0f32)) {
            // Note: Input is in radians, not degrees.
            let (std_sin, std_cos) = f32::sin_cos(data);
            let (tbl_sin, tbl_cos) = crate::table::sin_cos(data);

            let diff = (std_sin - tbl_sin).abs();
            assert!(diff < EPSILON, "{tbl_sin} != {std_sin} (diff: {diff})");

            let diff = (std_cos - tbl_cos).abs();
            assert!(diff < EPSILON, "{tbl_cos} != {std_cos} (diff: {diff})");
        }
    }
}
