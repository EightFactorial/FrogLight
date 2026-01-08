//! Common mathematical functions.
#![allow(clippy::unreadable_literal, reason = "It's actually more readable this way")]

/// Convert degrees to radians.
///
/// Matches Java's standard library.
#[must_use]
pub const fn to_radians(deg: f64) -> f64 { deg * 0.017453292519943295f64 }

/// Convert radians to degrees.
///
/// Matches Java's standard library.
#[must_use]
pub const fn to_degrees(rad: f64) -> f64 { rad * 57.29577951308232f64 }

/// Returns the sign of a floating point number or zero.
///
/// Similar to [`f64::signum`], but returns `0.0` for `0.0`.
#[must_use]
pub const fn signum(x: f64) -> f64 { if x == 0.0 { 0.0 } else { x.signum() } }

/// Linearly interpolate between two values.
#[must_use]
pub const fn lerp(start: f64, end: f64, t: f64) -> f64 { start + t * (end - start) }

/// Returns the fractional part of a floating point number.
#[must_use]
#[expect(clippy::cast_precision_loss, reason = "Desired behavior")]
#[expect(clippy::cast_possible_truncation, reason = "Desired behavior")]
pub const fn fract(x: f64) -> f64 {
    let int = x as i64 as f64;
    let floor = if x < int { int - 1.0 } else { int };
    x - floor
}

// -------------------------------------------------------------------------------------------------

/// Returns the base 2 logarithm of the number, rounded up.
///
/// Similar to [`u32::ilog2`], but rounds up instead of down.
#[must_use]
pub const fn ilog2_ceil(x: u32) -> u32 { u32::BITS - x.saturating_sub(1).leading_zeros() }

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
#[must_use]
pub const fn gcd_lcm(x: u32, y: u32) -> (u32, u32) {
    if x == 0 && y == 0 {
        (0, 0)
    } else {
        let gcd = gcd(x, y);
        (gcd, x * (y / gcd))
    }
}
