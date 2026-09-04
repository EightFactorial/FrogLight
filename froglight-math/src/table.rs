//! Precomputed sine and cosine tables.
#![allow(clippy::cast_precision_loss, reason = "Desired behavior")]
#![allow(clippy::cast_possible_truncation, reason = "Desired behavior")]
#![allow(clippy::cast_sign_loss, reason = "Desired behavior")]

use core::f64::consts::PI;

use froglight_common::types::LazyLock;

cfg_select! {
    all(feature = "nightly", feature = "std") => {
        /// A precomputed sine table for angles in the range `[0, 2π)`.
        ///
        /// Used by the [`sin`], [`cos`], and [`sin_cos`] functions.
        ///
        /// Uses SIMD intrinsics for better performance (generates roughly 64x
        /// faster).
        static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
            const BATCH: usize = 8;

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
    }
    _ => {
        /// A precomputed sine table for angles in the range `[0, 2π)`.
        ///
        /// Used by the [`sin`], [`cos`], and [`sin_cos`] functions.
        static SIN: LazyLock<[f32; 65536]> = LazyLock::new(|| {
            // Normally we'd use `core::array::from_fn`,
            // but it seems to be causing a stack overflow in some cases.
            let mut array = [0.0f64; 65536];
            array.iter_mut().enumerate().for_each(|(i, f)| {
                #[cfg(feature = "std")]
                {
                    *f = f64::sin((i as f64 * 2.0 * PI) / 65536.0);
                }
                #[cfg(all(not(feature = "std"), feature = "libm"))]
                {
                    *f = froglight_common::crates::libm::sin((i as f64 * 2.0 * PI) / 65536.0);
                }
            });

            // Normally we'd use `array.map` and to convert to `f32`s directly,
            // but it seems to be causing a stack overflow in some cases.
            let mut output = [0.0f32; 65536];
            array.iter().zip(output.iter_mut()).for_each(|(i, o)| {
                *o = *i as f32;
            });

            output
        });
    }
}

// -------------------------------------------------------------------------------------------------

const SCALE: f64 = 10_430.378_350_470_453;
const WRAP: f64 = 16_384.0;
const MASK: usize = 0xFFFF;

/// Calculate the sine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin(rad: f64) -> f32 {
    let x = rad * SCALE;
    let index = (x as u32 as usize) & MASK;

    SIN[index]
}

/// Calculate the cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn cos(rad: f64) -> f32 {
    let x = (rad * SCALE) + WRAP;
    let index = (x as u32 as usize) & MASK;

    SIN[index]
}

/// Calculate the sine and cosine of an angle using the [`SIN`] table.
#[must_use]
pub fn sin_cos(rad: f64) -> (f32, f32) {
    let x = rad * SCALE;
    let x_index = (x as u32 as usize) & MASK;

    let y = (rad * SCALE) + WRAP;
    let y_index = (y as u32 as usize) & MASK;

    (SIN[x_index], SIN[y_index])
}

#[cfg(test)]
mod tests {

    /// The acceptable error margin for table and non-table calculations.
    const EPSILON: f32 = 1.0e-15;

    /// Tests for the sine and cosine functions for common angles.
    #[test]
    #[allow(clippy::unreadable_literal, reason = "Ignore")]
    fn common() {
        use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8};

        fn assert(input: f64, expected: f32) {
            let sin = super::sin(input);
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
}
