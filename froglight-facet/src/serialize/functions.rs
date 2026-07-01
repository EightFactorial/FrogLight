//! TODO
#![expect(clippy::inline_always, reason = "Wrapper Functions")]

use alloc::vec::Vec;

use froglight_facet_iter::{Writer, WriterType};

use crate::serialize::{Serialize, SerializeError, future::SerializeAsync};

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized.
#[inline(always)]
pub fn to_vec<T: Serialize<'static>>(value: &T, protocol: u32) -> Result<Vec<u8>, SerializeError> {
    <T as Serialize>::to_vec(value, protocol)
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized.
#[inline(always)]
pub async fn to_vec_async<T: SerializeAsync<'static>>(
    value: &T,
    protocol: u32,
) -> Result<Vec<u8>, SerializeError> {
    <T as SerializeAsync>::to_vec_async(value, protocol).await
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
    protocol: u32,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as Serialize>::to_writer(value, false, protocol, Writer::new(writer))
}

/// TODO
///
/// # Errors
///
/// Returns an error if the value cannot be serialized
/// or if the writer encounters an error.
#[inline(always)]
pub async fn to_writer_async<T: SerializeAsync<'static>, W: WriterType>(
    value: &T,
    protocol: u32,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as SerializeAsync>::to_writer_async(value, false, protocol, Writer::new(writer)).await
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
    protocol: u32,
    writer: &mut W,
) -> Result<usize, SerializeError> {
    <T as Serialize>::to_writer(value, true, protocol, Writer::new(writer))
}
