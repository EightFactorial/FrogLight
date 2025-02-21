//! Implementations for `froglight-io`'s
//! [`FrogRead`](froglight_io::standard::FrogRead) and
//! [`FrogWrite`](froglight_io::standard::FrogWrite) traits.

use std::io::{Read, Write};

use froglight_io::prelude::*;

use crate::mutf8::{Mutf8Str, Mutf8String};
#[allow(clippy::wildcard_imports)]
use crate::nbt::*;

impl FrogRead for NamedNbt {
    // Tag + Name + Payload
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        let byte = u8::frog_read(buffer)?;
        if byte == NbtTag::END {
            Ok(Self::new_from(Mutf8String::from_bytes(Vec::new()), None))
        } else {
            let name = Mutf8String::frog_read(buffer)?;

            #[cfg(feature = "debug")]
            tracing_log::log::trace!("Reading NamedNbt: \"{}\"", name.to_str_lossy());

            UnnamedNbt::frog_read_inner(byte, buffer).map(|unnamed| unnamed.into_named(name))
        }
    }
}
impl FrogWrite for NamedNbt {
    // Tag + Name + Payload
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        match self.compound() {
            None => NbtTag::END.frog_write(buffer),
            Some(compound) => {
                NbtTag::COMPOUND.frog_write(buffer)?;
                self.name().frog_write(buffer)?;
                compound.frog_write(buffer)
            }
        }
    }

    fn frog_len(&self) -> usize {
        self.compound().map_or(std::mem::size_of::<u8>(), |nbt| {
            std::mem::size_of::<u8>() + self.name().frog_len() + nbt.frog_len()
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for UnnamedNbt {
    // Tag + Payload
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        UnnamedNbt::frog_read_inner(u8::frog_read(buffer)?, buffer)
    }
}
impl FrogWrite for UnnamedNbt {
    // Tag + Payload
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        match self.compound() {
            None => NbtTag::END.frog_write(buffer),
            Some(compound) => {
                NbtTag::COMPOUND.frog_write(buffer)?;
                compound.frog_write(buffer)
            }
        }
    }

    #[inline]
    fn frog_len(&self) -> usize {
        self.compound()
            .map_or(std::mem::size_of::<u8>(), |nbt| std::mem::size_of::<u8>() + nbt.frog_len())
    }
}

impl UnnamedNbt {
    fn frog_read_inner(tag: u8, buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "debug")]
        tracing_log::log::trace!("UnnamedNbt: Tag -> {tag}");

        match tag {
            NbtTag::END => Ok(Self::new_from(None)),
            NbtTag::COMPOUND => NbtCompound::frog_read(buffer).map(UnnamedNbt::new),
            unk => Err(ReadError::InvalidEnum(std::any::type_name::<Self>(), unk.into())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for NbtCompound {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "debug")]
        tracing_log::log::trace!("NbtCompound: ! Start !");

        let mut tag = u8::frog_read(buffer)?;
        let mut compound = Self::new();

        while NbtTag::END != tag {
            let string = Mutf8String::frog_read(buffer)?;
            let data = NbtTag::frog_read_inner(tag, buffer)?;
            tag = u8::frog_read(buffer)?;

            #[cfg(feature = "debug")]
            tracing_log::log::trace!("NbtCompound: \"{}\": {data:?}", string.to_str_lossy());

            compound.insert(string, data);
        }

        #[cfg(feature = "debug")]
        tracing_log::log::trace!("NbtCompound: ! Finish !");

        Ok(compound)
    }
}
impl FrogWrite for NbtCompound {
    fn frog_write(&self, _buffer: &mut impl Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize {
        self.iter().fold(std::mem::size_of::<u8>(), |acc, (key, value)| {
            acc + key.frog_len() + value.frog_len()
        })
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for NbtTag {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        Self::frog_read_inner(u8::frog_read(buffer)?, buffer)
    }
}
impl FrogWrite for NbtTag {
    fn frog_write(&self, _buffer: &mut impl Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize {
        // Tag + Payload
        std::mem::size_of::<u8>()
            + match self {
                NbtTag::Byte(_) => std::mem::size_of::<i8>(),
                NbtTag::Short(_) => std::mem::size_of::<i16>(),
                NbtTag::Int(_) => std::mem::size_of::<i32>(),
                NbtTag::Long(_) => std::mem::size_of::<i64>(),
                NbtTag::Float(_) => std::mem::size_of::<f32>(),
                NbtTag::Double(_) => std::mem::size_of::<f64>(),
                NbtTag::String(string) => string.frog_len(),
                NbtTag::List(list) => list.frog_len(),
                NbtTag::Compound(compound) => compound.frog_len(),
                NbtTag::ByteArray(items) => {
                    std::mem::size_of::<u32>() + (items.len() * std::mem::size_of::<i8>())
                }
                NbtTag::IntArray(items) => {
                    std::mem::size_of::<u32>() + (items.len() * std::mem::size_of::<i32>())
                }
                NbtTag::LongArray(items) => {
                    std::mem::size_of::<u32>() + (items.len() * std::mem::size_of::<i64>())
                }
            }
    }
}

impl NbtTag {
    fn frog_read_inner(tag: u8, buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "debug")]
        tracing_log::log::trace!("NbtTag: Tag -> {tag}");

        match tag {
            NbtTag::BYTE => i8::frog_read(buffer).map(Self::Byte),
            NbtTag::SHORT => i16::frog_read(buffer).map(Self::Short),
            NbtTag::INT => i32::frog_read(buffer).map(Self::Int),
            NbtTag::LONG => i64::frog_read(buffer).map(Self::Long),
            NbtTag::FLOAT => f32::frog_read(buffer).map(Self::Float),
            NbtTag::DOUBLE => f64::frog_read(buffer).map(Self::Double),
            NbtTag::STRING => Mutf8String::frog_read(buffer).map(Self::String),
            NbtTag::LIST => NbtListTag::frog_read(buffer).map(Self::List),
            NbtTag::COMPOUND => NbtCompound::frog_read(buffer).map(Self::Compound),
            NbtTag::BYTE_ARRAY => NbtListTag::frog_read_array(buffer).map(Self::ByteArray),
            NbtTag::INT_ARRAY => NbtListTag::frog_read_array(buffer).map(Self::IntArray),
            NbtTag::LONG_ARRAY => NbtListTag::frog_read_array(buffer).map(Self::LongArray),
            unk => Err(ReadError::InvalidEnum(std::any::type_name::<Self>(), unk.into())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for NbtListTag {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        NbtListTag::frog_read_inner(u8::frog_read(buffer)?, buffer)
    }
}
impl FrogWrite for NbtListTag {
    fn frog_write(&self, _buffer: &mut impl Write) -> Result<usize, WriteError> { todo!() }

    fn frog_len(&self) -> usize {
        // Tag + Length + Payload
        std::mem::size_of::<u8>()
            + std::mem::size_of::<u32>()
            + match self {
                NbtListTag::Empty => 0,
                NbtListTag::Byte(items) => items.len() * std::mem::size_of::<i8>(),
                NbtListTag::Short(items) => items.len() * std::mem::size_of::<i16>(),
                NbtListTag::Int(items) => items.len() * std::mem::size_of::<i32>(),
                NbtListTag::Long(items) => items.len() * std::mem::size_of::<i64>(),
                NbtListTag::Float(items) => items.len() * std::mem::size_of::<f32>(),
                NbtListTag::Double(items) => items.len() * std::mem::size_of::<f64>(),
                NbtListTag::String(items) => items.iter().map(|i| i.frog_len()).sum(),
                NbtListTag::List(items) => items.iter().map(|i| i.frog_len()).sum(),
                NbtListTag::Compound(items) => items.iter().map(|i| i.frog_len()).sum(),
                NbtListTag::ByteArray(items) => items
                    .iter()
                    .map(|i| std::mem::size_of::<u32>() + (i.len() * std::mem::size_of::<i8>()))
                    .sum(),
                NbtListTag::IntArray(items) => items
                    .iter()
                    .map(|i| std::mem::size_of::<u32>() + (i.len() * std::mem::size_of::<i32>()))
                    .sum(),
                NbtListTag::LongArray(items) => items
                    .iter()
                    .map(|i| std::mem::size_of::<u32>() + (i.len() * std::mem::size_of::<i64>()))
                    .sum(),
            }
    }
}

impl NbtListTag {
    fn frog_read_inner(tag: u8, buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "debug")]
        tracing_log::log::trace!("NbtListTag: Tag -> {tag}");

        match tag {
            NbtTag::END => u32::frog_read(buffer).map(|_| Self::Empty),
            NbtTag::BYTE => Self::frog_read_array(buffer).map(Self::Byte),
            NbtTag::SHORT => Self::frog_read_array(buffer).map(Self::Short),
            NbtTag::INT => Self::frog_read_array(buffer).map(Self::Int),
            NbtTag::LONG => Self::frog_read_array(buffer).map(Self::Long),
            NbtTag::FLOAT => Self::frog_read_array(buffer).map(Self::Float),
            NbtTag::DOUBLE => Self::frog_read_array(buffer).map(Self::Double),
            NbtTag::STRING => Self::frog_read_array(buffer).map(Self::String),
            NbtTag::LIST => Self::frog_read_array(buffer).map(Self::List),
            NbtTag::COMPOUND => Self::frog_read_array(buffer).map(Self::Compound),
            NbtTag::BYTE_ARRAY => Self::frog_read_array_array(buffer).map(Self::ByteArray),
            NbtTag::INT_ARRAY => Self::frog_read_array_array(buffer).map(Self::IntArray),
            NbtTag::LONG_ARRAY => Self::frog_read_array_array(buffer).map(Self::LongArray),
            unk => Err(ReadError::InvalidEnum(std::any::type_name::<Self>(), unk.into())),
        }
    }

    // NBT uses a plain `u32` for the length instead to the usual variable encoding
    fn frog_read_array<T: FrogRead>(buffer: &mut impl Read) -> Result<Vec<T>, ReadError> {
        (0..u32::frog_read(buffer)? as usize).map(|_| T::frog_read(buffer)).collect()
    }

    fn frog_read_array_array<T: FrogRead>(
        buffer: &mut impl Read,
    ) -> Result<Vec<Vec<T>>, ReadError> {
        (0..u32::frog_read(buffer)? as usize).map(|_| Self::frog_read_array::<T>(buffer)).collect()
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for Mutf8String {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        let mut bytes = vec![0; usize::from(u16::frog_read(buffer)?)];
        buffer.read_exact(&mut bytes)?;

        Ok(Self::from_bytes(bytes))
    }
}
impl FrogWrite for Mutf8String {
    #[inline]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        self.as_mutf8_str().frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { self.as_mutf8_str().frog_len() }
}
impl FrogWrite for Mutf8Str {
    #[expect(clippy::cast_possible_truncation)]
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        (self.len() as u16).frog_write(buffer)?;
        buffer.write_all(self.as_bytes())?;

        Ok(self.frog_len())
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn frog_len(&self) -> usize { std::mem::size_of::<u16>() + self.as_bytes().len() }
}
