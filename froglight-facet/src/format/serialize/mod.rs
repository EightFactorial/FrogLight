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

use crate::format::writer::{Writer, WriterError};

/// A trait for types that can be deserialized.
#[expect(clippy::missing_errors_doc, missing_docs, reason = "WIP")]
pub trait Serialize<'facet> {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::new();
        <Self as Serialize>::to_writer(value, Writer::new(&mut buffer)).map(|_| buffer)
    }

    fn to_writer(value: &Self, writer: Writer<'_>) -> Result<usize, SerializeError>;
}

impl<'facet, T: Facet<'facet>> Serialize<'facet> for T {
    #[inline]
    fn to_vec(value: &Self) -> Result<Vec<u8>, SerializeError> {
        let mut buffer = Vec::with_capacity(64); // TODO: Size hint
        <Self as Serialize>::to_writer(value, Writer::new(&mut buffer)).map(|_| buffer)
    }

    #[inline]
    fn to_writer(value: &Self, writer: Writer<'_>) -> Result<usize, SerializeError> {
        serialize(Peek::new(value), writer)
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

fn serialize(peek: Peek<'_, '_>, mut writer: Writer<'_>) -> Result<usize, SerializeError> {
    let core = |item| {
        let (peek, var) = match item {
            Item::Size(_size) => todo!("Variable-length encode `size`"),
            Item::Peek(peek, var) => (peek, var),
        };

        if let Ok(()) = peek.get::<()>() {
            return Ok(());
        }

        if let Ok(bool) = peek.get::<bool>() {
            return writer.write_byte(u8::from(*bool));
        }

        if let Ok(u8) = peek.get::<u8>() {
            if var {
                todo!("Variable-length encode `u8`");
            } else {
                return writer.write_byte(*u8);
            }
        } else if let Ok(u16) = peek.get::<u16>() {
            if var {
                todo!("Variable-length encode `u16`");
            } else {
                return writer.write_bytes(&u16.to_le_bytes());
            }
        } else if let Ok(u32) = peek.get::<u32>() {
            if var {
                todo!("Variable-length encode `u32`");
            } else {
                return writer.write_bytes(&u32.to_le_bytes());
            }
        } else if let Ok(u64) = peek.get::<u64>() {
            if var {
                todo!("Variable-length encode `u64`");
            } else {
                return writer.write_bytes(&u64.to_le_bytes());
            }
        } else if let Ok(u128) = peek.get::<u128>() {
            if var {
                todo!("Variable-length encode `u128`");
            } else {
                return writer.write_bytes(&u128.to_le_bytes());
            }
        }

        if let Ok(i8) = peek.get::<i8>() {
            if var {
                todo!("Variable-length encode `i8`");
            } else {
                #[expect(clippy::cast_sign_loss, reason = "Desired behavior")]
                return writer.write_byte(*i8 as u8);
            }
        } else if let Ok(i16) = peek.get::<i16>() {
            if var {
                todo!("Variable-length encode `i16`");
            } else {
                return writer.write_bytes(&i16.to_le_bytes());
            }
        } else if let Ok(i32) = peek.get::<i32>() {
            if var {
                todo!("Variable-length encode `i32`");
            } else {
                return writer.write_bytes(&i32.to_le_bytes());
            }
        } else if let Ok(i64) = peek.get::<i64>() {
            if var {
                todo!("Variable-length encode `i64`");
            } else {
                return writer.write_bytes(&i64.to_le_bytes());
            }
        } else if let Ok(i128) = peek.get::<i128>() {
            if var {
                todo!("Variable-length encode `i128`");
            } else {
                return writer.write_bytes(&i128.to_le_bytes());
            }
        }

        if let Ok(f32) = peek.get::<f32>() {
            return writer.write_bytes(&f32.to_le_bytes());
        } else if let Ok(f64) = peek.get::<f64>() {
            return writer.write_bytes(&f64.to_le_bytes());
        }

        // Handle strings
        get_as!(peek => String, Cow<'_, str>, str: |value: &str| -> Result<(), WriterError> {
            // todo!("Variable-length encode `value.len()`");
            writer.write_bytes(value.as_bytes())
        });

        todo!("Unhandled type `{}`: {peek:?}", peek.shape().type_name());
    };

    let mut ser = Serializer::new(peek, core);
    while let Some(result) = Iterator::next(&mut ser) {
        result?;
    }

    drop(ser);
    Ok(writer.total_written())
}
