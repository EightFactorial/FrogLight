//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use alloc::vec::Vec;

use super::{Serialize, SerializeError};
use crate::format::writer::{Writer, WriterType};

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn to_vec<T: Serialize<'static>>(value: &T) -> Result<Vec<u8>, SerializeError> {
    <T as Serialize>::to_vec(value)
}

/// TODO
///
/// # Errors
///
/// TODO
#[inline(always)]
pub fn to_writer<T: Serialize<'static>, W: WriterType>(
    value: &T,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as Serialize>::to_writer(value, Writer::new(writer))
}
