//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]
#![expect(clippy::result_unit_err, reason = "WIP")]

use alloc::vec::Vec;

use super::Serialize;

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn to_vec<T: Serialize<'static>>(value: &T) -> Result<Vec<u8>, ()> {
    <T as Serialize>::to_vec(value)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn to_buffer<T: Serialize<'static>>(value: &T, buffer: &mut ()) -> Result<usize, ()> {
    <T as Serialize>::to_buffer(value, buffer)
}
