//! TODO
#![expect(clippy::inline_always, reason = "Performance")]

#[cfg(all(target_arch = "aarch64", not(feature = "simd_fallback")))]
pub mod aarch64;
#[cfg(all(target_arch = "aarch64", not(feature = "simd_fallback")))]
pub use aarch64::*;
/// The SIMD module currently being used.
#[cfg(all(target_arch = "aarch64", not(feature = "simd_fallback")))]
pub const ARCH: &str = "aarch64";

#[cfg(all(target_arch = "x86_64", not(feature = "simd_fallback")))]
pub mod x86_64;
#[cfg(all(target_arch = "x86_64", not(feature = "simd_fallback")))]
pub use x86_64::*;
/// The SIMD module currently being used.
#[cfg(all(target_arch = "x86_64", not(feature = "simd_fallback")))]
pub const ARCH: &str = "x86_64";

pub mod fallback;
#[cfg(any(
    not(any(target_arch = "aarch64", target_arch = "x86_64")),
    feature = "simd_fallback"
))]
pub use fallback::*;
/// The SIMD module currently being used.
#[cfg(any(not(any(target_arch = "aarch64", target_arch = "x86_64")), feature = "simd_fallback"))]
pub const ARCH: &str = "fallback";

mod traits;
pub use traits::VarIntType;

/// Convert a 4-byte array to a 32-byte array by padding zeros to the end.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr4_to_31(arr: [u8; 4]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Convert a 5-byte array to a 32-byte array by padding zeros to the end.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr5_to_31(arr: [u8; 5]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], arr[4], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Convert an 8-byte array to a 32-byte array by padding zeros to the end.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr8_to_31(arr: [u8; 8]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Convert a 16-byte array to a 32-byte array by padding zeros to the end.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr16_to_31(arr: [u8; 16]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], arr[8], arr[9], arr[10],
        arr[11], arr[12], arr[13], arr[14], arr[15], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Convert a 24-byte array to a 32-byte array by padding zeros to the end.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr24_to_31(arr: [u8; 24]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], arr[8], arr[9], arr[10],
        arr[11], arr[12], arr[13], arr[14], arr[15], arr[16], arr[17], arr[18], arr[19], arr[20],
        arr[21], arr[22], arr[23], 0, 0, 0, 0, 0, 0, 0,
    ]
}

/// Convert a 32-byte array to a 31-byte array by truncating the last byte.
#[inline(always)]
#[allow(dead_code, reason = "May not be used depending on the platform")]
const fn arr32_to_31(arr: [u8; 32]) -> [u8; 31] {
    [
        arr[0], arr[1], arr[2], arr[3], arr[4], arr[5], arr[6], arr[7], arr[8], arr[9], arr[10],
        arr[11], arr[12], arr[13], arr[14], arr[15], arr[16], arr[17], arr[18], arr[19], arr[20],
        arr[21], arr[22], arr[23], arr[24], arr[25], arr[26], arr[27], arr[28], arr[29], arr[30],
    ]
}
