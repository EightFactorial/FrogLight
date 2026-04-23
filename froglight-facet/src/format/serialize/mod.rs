//! TODO

use alloc::{borrow::Cow, string::String, vec::Vec};

use facet::{Facet, Peek};

mod error;
pub use error::SerializeError;

pub mod functions;

pub(crate) mod iterator;
pub use iterator::IteratorStack;

pub(crate) mod logic;
pub use logic::{Item, Serializer};

pub mod varint;

use crate::{
    facet::WithFnAttr,
    format::{
        serialize::iterator::StackItem,
        writer::{Writer, WriterError},
    },
};

/// A trait for types that can be deserialized.
#[expect(clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Serialize<'facet> {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::new();
        <Self as Serialize>::to_writer(value, false, Writer::new(&mut buffer)).map(|_| buffer)
    }

    fn to_writer(value: &Self, variable: bool, writer: Writer<'_>)
    -> Result<usize, SerializeError>;
}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::with_capacity(8); // TODO: Size hint
        <Self as Serialize>::to_writer(value, false, Writer::new(&mut buffer)).map(|_| buffer)
    }

    #[inline]
    fn to_writer(
        value: &Self,
        variable: bool,
        writer: Writer<'_>,
    ) -> Result<usize, SerializeError> {
        serialize(Peek::new(value), variable, writer)
    }
}

// -------------------------------------------------------------------------------------------------

macro_rules! get_as {
    ($peek:expr => $($ty:ty),*: $closure:expr) => {
        $(
            if let Ok(value) = $peek.get::<$ty>() { return ($closure)(value); }
        )*
    };
}

#[expect(clippy::too_many_lines, reason = "TODO: Compact integer handling")]
fn serialize(
    peek: Peek<'_, '_>,
    variable: bool,
    mut writer: Writer<'_>,
) -> Result<usize, SerializeError> {
    let core = |item| {
        let item @ StackItem { peek, variable, field, .. } = match item {
            Item::Item(item) => item,
            Item::Size(size) => {
                let (bytes, len) = varint::encode_u32(size);
                writer.write_bytes(&bytes[..len as usize])?;
                return Ok(());
            }
        };

        if let Some(field) = field {
            // Handle field attributes

            // Run the custom serializer.
            if let Some(with) = field.get_attr(Some("mc"), "with")
                && let Some(with) = with.get_as::<WithFnAttr>()
            {
                return with.serialize(item, &mut writer);
            }
        }

        if let Ok(()) = peek.get::<()>() {
            return Ok(());
        }

        if let Ok(bool) = peek.get::<bool>() {
            return writer.write_byte(u8::from(*bool));
        }

        if let Ok(u8) = peek.get::<u8>() {
            return if variable {
                let (bytes, len) = varint::encode_u8(*u8);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_byte(*u8)
            };
        } else if let Ok(u16) = peek.get::<u16>() {
            return if variable {
                let (bytes, len) = varint::encode_u16(*u16);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u16.to_le_bytes())
            };
        } else if let Ok(u32) = peek.get::<u32>() {
            return if variable {
                let (bytes, len) = varint::encode_u32(*u32);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u32.to_le_bytes())
            };
        } else if let Ok(u64) = peek.get::<u64>() {
            return if variable {
                let (bytes, len) = varint::encode_u64(*u64);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u64.to_le_bytes())
            };
        } else if let Ok(u128) = peek.get::<u128>() {
            return if variable {
                let (bytes, len) = varint::encode_u128(*u128);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u128.to_le_bytes())
            };
        }

        #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
        if let Ok(i8) = peek.get::<i8>() {
            let u8 = *i8 as u8;
            return if variable {
                let (bytes, len) = varint::encode_u8(u8);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_byte(u8)
            };
        } else if let Ok(i16) = peek.get::<i16>() {
            let u16 = *i16 as u16;
            return if variable {
                let (bytes, len) = varint::encode_u16(u16);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u16.to_le_bytes())
            };
        } else if let Ok(i32) = peek.get::<i32>() {
            let u32 = *i32 as u32;
            return if variable {
                let (bytes, len) = varint::encode_u32(u32);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u32.to_le_bytes())
            };
        } else if let Ok(i64) = peek.get::<i64>() {
            let u64 = *i64 as u64;
            return if variable {
                let (bytes, len) = varint::encode_u64(u64);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u64.to_le_bytes())
            };
        } else if let Ok(i128) = peek.get::<i128>() {
            let u128 = *i128 as u128;
            return if variable {
                let (bytes, len) = varint::encode_u128(u128);
                writer.write_bytes(&bytes[..len as usize])
            } else {
                writer.write_bytes(&u128.to_le_bytes())
            };
        }

        if let Ok(f32) = peek.get::<f32>() {
            return writer.write_bytes(&f32.to_le_bytes());
        } else if let Ok(f64) = peek.get::<f64>() {
            return writer.write_bytes(&f64.to_le_bytes());
        }

        // Handle strings
        get_as!(peek => String, Cow<'_, str>, str: |value: &str| -> Result<(), WriterError> {
            let (bytes, len) = varint::encode_u32(value.len().try_into()?);
            writer.write_bytes(&bytes[..len as usize])?;
            writer.write_bytes(value.as_bytes())
        });

        todo!("Unhandled type `{}`: {peek:?}", peek.shape().type_name());
    };

    let mut ser = Serializer::new(peek, variable, core);
    while let Some(result) = Iterator::next(&mut ser) {
        result?;
    }

    drop(ser);
    Ok(writer.total_written())
}
