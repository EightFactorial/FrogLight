//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use alloc::vec::Vec;

use super::{Serialize, SerializeError};
use crate::format::writer::{Writer, WriterType};

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized.
#[inline(always)]
pub fn to_vec<T: Serialize<'static>>(value: &T) -> Result<Vec<u8>, SerializeError> {
    <T as Serialize>::to_vec(value)
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized
/// or if the writer encounters an error.
#[inline(always)]
pub fn to_writer<T: Serialize<'static>, W: WriterType>(
    value: &T,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as Serialize>::to_writer(value, false, Writer::new(writer))
}

/// Serialize the value to the given [`WriterType`].
///
/// This is exactly the same as [`to_writer`], but acts as if the value is
/// marked with `#[facet(mc::variable)]`.
///
/// This is only useful for scalars like `u32`, lists like `Vec<u64>`, and
/// maps like `HashMap<String, u32>`. Otherwise it will have no effect.
///
/// # Errors
///
/// Returns an error if the value cannot be serialized
/// or if the writer encounters an error.
#[inline(always)]
pub fn to_writer_variable<T: Serialize<'static>, W: WriterType>(
    value: &T,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as Serialize>::to_writer(value, true, Writer::new(writer))
}
