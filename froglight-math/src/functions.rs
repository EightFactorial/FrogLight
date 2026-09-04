//! Common mathematical functions.
#![allow(clippy::unreadable_literal, reason = "It's actually more readable this way")]

use froglight_common::crates::glam::{DMat2, DVec2, DVec3, DVec4};

/// Convert degrees to radians.
///
/// Matches Java's standard library.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(0f64.to_radians(), to_radians(0f64));
/// assert_eq!(1f64.to_radians(), to_radians(1f64));
/// assert_eq!(2f64.to_radians(), to_radians(2f64));
/// ```
#[inline]
#[must_use]
pub const fn to_radians(deg: f64) -> f64 { deg * 0.017453292519943295f64 }

/// Convert radians to degrees.
///
/// Matches Java's standard library.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(0f64.to_degrees(), to_degrees(0f64));
/// assert_eq!(1f64.to_degrees(), to_degrees(1f64));
/// assert_eq!(2f64.to_degrees(), to_degrees(2f64));
/// ```
#[inline]
#[must_use]
pub const fn to_degrees(rad: f64) -> f64 { rad * 57.29577951308232f64 }

/// Returns the fractional part of a floating point number.
#[inline]
#[must_use]
#[expect(clippy::cast_precision_loss, reason = "Desired behavior")]
#[expect(clippy::cast_possible_truncation, reason = "Desired behavior")]
pub const fn fract(x: f64) -> f64 {
    let int = x as i64 as f64;
    let floor = if x < int { int - 1.0 } else { int };
    x - floor
}

// -------------------------------------------------------------------------------------------------

/// Returns the sign of a floating point number or zero.
///
/// Similar to [`f64::signum`], but returns `0.0` for `0.0`.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(signum(-1.0), -1.0);
/// assert_eq!(signum(0.0), 0.0);
/// assert_eq!(signum(1.0), 1.0);
/// ```
#[inline]
#[must_use]
pub const fn signum(x: f64) -> f64 { if x == 0.0 { 0.0 } else { x.signum() } }

/// Linearly interpolate between two values.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(lerp(-1.0, 0.0, 10.0), -10.0);
/// assert_eq!(lerp(-0.5, 0.0, 10.0), -5.0);
/// assert_eq!(lerp(0.0, 0.0, 10.0), 0.0);
/// assert_eq!(lerp(0.5, 0.0, 10.0), 5.0);
/// assert_eq!(lerp(1.0, 0.0, 10.0), 10.0);
/// assert_eq!(lerp(1.5, 0.0, 10.0), 15.0);
/// assert_eq!(lerp(2.0, 0.0, 10.0), 20.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp(val: f64, start: f64, end: f64) -> f64 { start + val * (end - start) }

/// Linearly interpolate between two values.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let range = DVec2::new(0.0, 10.0);
///
/// assert_eq!(lerp_vec2(-1.0, range), -10.0);
/// assert_eq!(lerp_vec2(-0.5, range), -5.0);
/// assert_eq!(lerp_vec2(0.0, range), 0.0);
/// assert_eq!(lerp_vec2(0.5, range), 5.0);
/// assert_eq!(lerp_vec2(1.0, range), 10.0);
/// assert_eq!(lerp_vec2(1.5, range), 15.0);
/// assert_eq!(lerp_vec2(2.0, range), 20.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_vec2(val: f64, range: DVec2) -> f64 { range.x + val * (range.y - range.x) }

/// Linearly interpolate between two values,
/// clamping the interpolation factor to the range [0, 1].
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(lerp_clamped(-1.0, 0.0, 10.0), 0.0);
/// assert_eq!(lerp_clamped(-0.5, 0.0, 10.0), 0.0);
/// assert_eq!(lerp_clamped(0.0, 0.0, 10.0), 0.0);
/// assert_eq!(lerp_clamped(0.5, 0.0, 10.0), 5.0);
/// assert_eq!(lerp_clamped(1.0, 0.0, 10.0), 10.0);
/// assert_eq!(lerp_clamped(1.5, 0.0, 10.0), 10.0);
/// assert_eq!(lerp_clamped(2.0, 0.0, 10.0), 10.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_clamped(val: f64, start: f64, end: f64) -> f64 {
    if val < 0.0 {
        start
    } else if val > 1.0 {
        end
    } else {
        lerp(val, start, end)
    }
}

/// Linearly interpolate between two values,
/// clamping the interpolation factor to the range [0, 1].
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let range = DVec2::new(0.0, 10.0);
///
/// assert_eq!(lerp_vec2_clamped(-1.0, range), 0.0);
/// assert_eq!(lerp_vec2_clamped(-0.5, range), 0.0);
/// assert_eq!(lerp_vec2_clamped(0.0, range), 0.0);
/// assert_eq!(lerp_vec2_clamped(0.5, range), 5.0);
/// assert_eq!(lerp_vec2_clamped(1.0, range), 10.0);
/// assert_eq!(lerp_vec2_clamped(1.5, range), 10.0);
/// assert_eq!(lerp_vec2_clamped(2.0, range), 10.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_vec2_clamped(val: f64, range: DVec2) -> f64 {
    if val < 0.0 {
        range.x
    } else if val > 1.0 {
        range.y
    } else {
        lerp_vec2(val, range)
    }
}

/// Returns the interpolation factor that would produce a given value.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(lerp_inverse(-10.0, 0.0, 10.0), -1.0);
/// assert_eq!(lerp_inverse(-5.0, 0.0, 10.0), -0.5);
/// assert_eq!(lerp_inverse(0.0, 0.0, 10.0), 0.0);
/// assert_eq!(lerp_inverse(5.0, 0.0, 10.0), 0.5);
/// assert_eq!(lerp_inverse(10.0, 0.0, 10.0), 1.0);
/// assert_eq!(lerp_inverse(15.0, 0.0, 10.0), 1.5);
/// assert_eq!(lerp_inverse(20.0, 0.0, 10.0), 2.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_inverse(val: f64, start: f64, end: f64) -> f64 { (val - start) / (end - start) }

/// Returns the interpolation factor that would produce a given value.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let range = DVec2::new(0.0, 10.0);
///
/// assert_eq!(lerp_vec2_inverse(-10.0, range), -1.0);
/// assert_eq!(lerp_vec2_inverse(-5.0, range), -0.5);
/// assert_eq!(lerp_vec2_inverse(0.0, range), 0.0);
/// assert_eq!(lerp_vec2_inverse(5.0, range), 0.5);
/// assert_eq!(lerp_vec2_inverse(10.0, range), 1.0);
/// assert_eq!(lerp_vec2_inverse(15.0, range), 1.5);
/// assert_eq!(lerp_vec2_inverse(20.0, range), 2.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_vec2_inverse(val: f64, range: DVec2) -> f64 {
    lerp_inverse(val, range.x, range.y)
}

/// Bilinearly interpolate between four values in 2D space.
///
/// TODO: Check that this is correct.
///
/// # Examples
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let start = DVec2::new(0.0, 0.0);
/// let end = DVec2::new(10.0, 10.0);
///
/// let val = DVec2::new(0.0, 0.0);
/// assert_eq!(lerp_bilinear(val, start, end), 0.0);
///
/// let val = DVec2::new(0.5, 0.5);
/// assert_eq!(lerp_bilinear(val, start, end), 5.0);
///
/// let val = DVec2::new(0.0, 1.0);
/// assert_eq!(lerp_bilinear(val, start, end), 10.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_bilinear(val: DVec2, start: DVec2, end: DVec2) -> f64 {
    lerp(val.y, lerp_vec2(val.x, start), lerp_vec2(val.x, end))
}

/// Bilinearly interpolate between four values in 2D space.
///
/// TODO: Check that this is correct.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::{DMat2, DVec2};
///
/// let x = DVec2::new(0.0, 0.0);
/// let y = DVec2::new(10.0, 10.0);
/// let mat = DMat2::from_cols(x, y);
///
/// let val = DVec2::new(0.0, 0.0);
/// assert_eq!(lerp_bilinear_mat2(val, mat), 0.0);
///
/// let val = DVec2::new(0.5, 0.5);
/// assert_eq!(lerp_bilinear_mat2(val, mat), 5.0);
///
/// let val = DVec2::new(0.0, 1.0);
/// assert_eq!(lerp_bilinear_mat2(val, mat), 10.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_bilinear_mat2(val: DVec2, mat: DMat2) -> f64 {
    lerp_bilinear(val, mat.x_axis, mat.y_axis)
}

/// Trilinearly interpolate between eight values in 3D space.
///
/// TODO: Check that this is correct.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::{DVec3, DVec4};
///
/// let start = DVec4::new(0.0, 0.0, 0.0, 0.0);
/// let end = DVec4::new(10.0, 10.0, 10.0, 10.0);
///
/// let val = DVec3::new(0.0, 0.0, 0.0);
/// assert_eq!(lerp_trilinear(val, start, end), 0.0);
///
/// let val = DVec3::new(0.5, 0.5, 0.5);
/// assert_eq!(lerp_trilinear(val, start, end), 6.25);
///
/// let val = DVec3::new(0.0, 1.0, 1.0);
/// assert_eq!(lerp_trilinear(val, start, end), 10.0);
/// ```
#[inline]
#[must_use]
pub const fn lerp_trilinear(val: DVec3, start: DVec4, end: DVec4) -> f64 {
    let a = DVec2::new(start.x, start.y);
    let b = DVec2::new(start.z, end.w);
    let c = DVec2::new(end.x, end.y);
    let d = DVec2::new(end.z, end.w);
    let bi_val = DVec2::new(val.x, val.y);

    lerp(val.z, lerp_bilinear(bi_val, a, b), lerp_bilinear(bi_val, c, d))
}

// -------------------------------------------------------------------------------------------------

/// Map a value from one range to another.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let from = DVec2::new(0.0, 10.0);
/// let to = DVec2::new(0.0, 100.0);
///
/// assert_eq!(map_range(-1.0, from, to), -10.0);
/// assert_eq!(map_range(-0.5, from, to), -5.0);
/// assert_eq!(map_range(0.0, from, to), 0.0);
/// assert_eq!(map_range(0.5, from, to), 5.0);
/// assert_eq!(map_range(1.0, from, to), 10.0);
/// assert_eq!(map_range(1.5, from, to), 15.0);
/// assert_eq!(map_range(2.0, from, to), 20.0);
/// ```
#[inline]
#[must_use]
pub const fn map_range(val: f64, from: DVec2, to: DVec2) -> f64 {
    lerp_vec2(lerp_vec2_inverse(val, from), to)
}

/// Map a value from one range to another,
/// clamping the result to the target range.
///
/// # Examples
/// ```rust
/// use froglight_math::prelude::*;
/// use glam::DVec2;
///
/// let from = DVec2::new(0.0, 10.0);
/// let to = DVec2::new(0.0, 100.0);
///
/// assert_eq!(map_range_clamped(-1.0, from, to), 0.0);
/// assert_eq!(map_range_clamped(-0.5, from, to), 0.0);
/// assert_eq!(map_range_clamped(0.0, from, to), 0.0);
/// assert_eq!(map_range_clamped(0.5, from, to), 5.0);
/// assert_eq!(map_range_clamped(1.0, from, to), 10.0);
/// assert_eq!(map_range_clamped(1.5, from, to), 15.0);
/// assert_eq!(map_range_clamped(2.0, from, to), 20.0);
/// ```
#[inline]
#[must_use]
pub const fn map_range_clamped(val: f64, from: DVec2, to: DVec2) -> f64 {
    lerp_vec2_clamped(lerp_vec2_inverse(val, from), to)
}

// -------------------------------------------------------------------------------------------------

/// Square a value.
#[inline]
#[must_use]
pub const fn square(x: f64) -> f64 { x * x }

/// Cube a value.
#[inline]
#[must_use]
pub const fn cube(x: f64) -> f64 { x * x * x }

// -------------------------------------------------------------------------------------------------

/// Returns the base 2 logarithm of the number, rounded up.
///
/// Similar to [`u32::ilog2`], but rounds up instead of down.
#[inline]
#[must_use]
pub const fn ilog2_ceil(x: u32) -> u32 { x.saturating_sub(1).bit_width() }

/// Return the greatest common divisor of two numbers.
///
/// ## Note
///
/// If you need both the GCD and LCM, consider using [`gcd_lcm`] instead.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(gcd(0, 0), 0);
/// assert_eq!(gcd(0, 1), 1);
/// assert_eq!(gcd(1, 0), 1);
/// assert_eq!(gcd(1, 1), 1);
/// assert_eq!(gcd(12, 15), 3);
/// assert_eq!(gcd(15, 12), 3);
/// assert_eq!(gcd(54, 24), 6);
/// assert_eq!(gcd(24, 54), 6);
/// assert_eq!(gcd(17, 13), 1);
/// assert_eq!(gcd(13, 17), 1);
/// ```
#[must_use]
pub const fn gcd(mut x: u32, mut y: u32) -> u32 {
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

/// Return the least common multiple of two numbers.
///
/// ## Note
///
/// If you need both the GCD and LCM, consider using [`gcd_lcm`] instead.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(lcm(0, 0), 0);
/// assert_eq!(lcm(0, 1), 0);
/// assert_eq!(lcm(1, 0), 0);
/// assert_eq!(lcm(1, 1), 1);
/// assert_eq!(lcm(12, 15), 60);
/// assert_eq!(lcm(15, 12), 60);
/// assert_eq!(lcm(54, 24), 216);
/// assert_eq!(lcm(24, 54), 216);
/// assert_eq!(lcm(17, 13), 221);
/// assert_eq!(lcm(13, 17), 221);
/// ```
#[inline]
#[must_use]
pub const fn lcm(x: u32, y: u32) -> u32 { gcd_lcm(x, y).1 }

/// Return the greatest common divisor and least common multiple of two numbers.
///
/// # Examples
///
/// ```rust
/// use froglight_math::prelude::*;
///
/// assert_eq!(gcd_lcm(0, 0), (0, 0));
/// assert_eq!(gcd_lcm(0, 1), (1, 0));
/// assert_eq!(gcd_lcm(1, 0), (1, 0));
/// assert_eq!(gcd_lcm(1, 1), (1, 1));
/// assert_eq!(gcd_lcm(12, 15), (3, 60));
/// assert_eq!(gcd_lcm(15, 12), (3, 60));
/// assert_eq!(gcd_lcm(54, 24), (6, 216));
/// assert_eq!(gcd_lcm(24, 54), (6, 216));
/// assert_eq!(gcd_lcm(17, 13), (1, 221));
/// assert_eq!(gcd_lcm(13, 17), (1, 221));
/// ```
#[inline]
#[must_use]
pub const fn gcd_lcm(x: u32, y: u32) -> (u32, u32) {
    if x == 0 && y == 0 {
        (0, 0)
    } else {
        let gcd = gcd(x, y);
        (gcd, x * (y / gcd))
    }
}
