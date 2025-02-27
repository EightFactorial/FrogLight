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
            Ok(Self::new_empty())
        } else {
            let name = Mutf8String::frog_read(buffer)?;

            #[cfg(feature = "debug")]
            tracing::trace!("Reading NamedNbt: \"{}\"", name.to_str_lossy());

            UnnamedNbt::frog_read_inner(byte, buffer).map(|unnamed| match unnamed.into_inner() {
                None => Self::new_empty(),
                Some(compound) => Self::new(name, compound),
            })
        }
    }
}
impl FrogWrite for NamedNbt {
    // Tag + Name + Payload
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        match self.compound() {
            None => NbtTag::END.frog_write(buffer),
            Some(compound) => {
                let tag = NbtTag::COMPOUND.frog_write(buffer)?;
                let name = self.name().unwrap().frog_write(buffer)?;
                compound.frog_write(buffer).map(|payload| tag + name + payload)
            }
        }
    }

    fn frog_len(&self) -> usize {
        self.compound().map_or(std::mem::size_of::<u8>(), |nbt| {
            std::mem::size_of::<u8>() + self.name().unwrap().frog_len() + nbt.frog_len()
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
                let tag = NbtTag::COMPOUND.frog_write(buffer)?;
                compound.frog_write(buffer).map(|payload| tag + payload)
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
        tracing::trace!("UnnamedNbt: Tag -> {tag}");

        match tag {
            NbtTag::END => Ok(Self::new_empty()),
            NbtTag::COMPOUND => NbtCompound::frog_read(buffer).map(UnnamedNbt::new),
            unk => Err(ReadError::InvalidEnum(std::any::type_name::<Self>(), unk.into())),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for NbtCompound {
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        #[cfg(feature = "debug")]
        tracing::trace!("NbtCompound: ! Start !");

        let mut tag = u8::frog_read(buffer)?;
        let mut compound = Self::new();

        while NbtTag::END != tag {
            let string = Mutf8String::frog_read(buffer)?;
            let data = NbtTag::frog_read_inner(tag, buffer)?;
            tag = u8::frog_read(buffer)?;

            #[cfg(feature = "debug")]
            tracing::trace!("NbtCompound: \"{}\": {data:?}", string.to_str_lossy());

            compound.insert(string, data);
        }

        #[cfg(feature = "debug")]
        tracing::trace!("NbtCompound: ! Finish !");

        Ok(compound)
    }
}
impl FrogWrite for NbtCompound {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        let payload =
            self.iter().try_fold::<_, _, Result<usize, WriteError>>(0, |acc, (key, value)| {
                Ok(acc
                    + value.tag_id().frog_write(buffer)?
                    + key.frog_write(buffer)?
                    + value.frog_write_inner(buffer)?)
            })?;
        NbtTag::END.frog_write(buffer).map(|tag| payload + tag)
    }

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
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        let tag = self.tag_id().frog_write(buffer)?;
        self.frog_write_inner(buffer).map(|payload| tag + payload)
    }

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
        tracing::trace!("NbtTag: Tag -> {tag}");

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

    fn frog_write_inner(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        match self {
            NbtTag::Byte(byte) => byte.frog_write(buffer),
            NbtTag::Short(short) => short.frog_write(buffer),
            NbtTag::Int(int) => int.frog_write(buffer),
            NbtTag::Long(long) => long.frog_write(buffer),
            NbtTag::Float(float) => float.frog_write(buffer),
            NbtTag::Double(double) => double.frog_write(buffer),
            NbtTag::String(string) => string.frog_write(buffer),
            NbtTag::List(list) => list.frog_write(buffer),
            NbtTag::Compound(compound) => compound.frog_write(buffer),
            NbtTag::ByteArray(items) => NbtListTag::frog_write_array(items, buffer),
            NbtTag::IntArray(items) => NbtListTag::frog_write_array(items, buffer),
            NbtTag::LongArray(items) => NbtListTag::frog_write_array(items, buffer),
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FrogRead for NbtListTag {
    #[inline]
    fn frog_read(buffer: &mut impl Read) -> Result<Self, ReadError> {
        match u8::frog_read(buffer)? {
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
}
impl FrogWrite for NbtListTag {
    fn frog_write(&self, buffer: &mut impl Write) -> Result<usize, WriteError> {
        let tag = self.tag_id().frog_write(buffer)?;
        match self {
            NbtListTag::Empty => 0u32.frog_write(buffer).map(|payload| tag + payload),
            NbtListTag::Byte(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Short(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Int(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Long(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Float(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Double(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::String(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::Compound(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::ByteArray(items) => {
                NbtListTag::frog_write_array_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::IntArray(items) => {
                NbtListTag::frog_write_array_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::LongArray(items) => {
                NbtListTag::frog_write_array_array(items, buffer).map(|payload| tag + payload)
            }
            NbtListTag::List(items) => {
                NbtListTag::frog_write_array(items, buffer).map(|payload| tag + payload)
            }
        }
    }

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
                NbtListTag::String(items) => items.iter().map(FrogWrite::frog_len).sum(),
                NbtListTag::List(items) => items.iter().map(FrogWrite::frog_len).sum(),
                NbtListTag::Compound(items) => items.iter().map(FrogWrite::frog_len).sum(),
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
    // NBT uses a plain `u32` for the length instead of the usual variable encoding
    fn frog_read_array<T: FrogRead>(buffer: &mut impl Read) -> Result<Vec<T>, ReadError> {
        (0..u32::frog_read(buffer)? as usize).map(|_| T::frog_read(buffer)).collect()
    }

    fn frog_read_array_array<T: FrogRead>(
        buffer: &mut impl Read,
    ) -> Result<Vec<Vec<T>>, ReadError> {
        (0..u32::frog_read(buffer)? as usize).map(|_| Self::frog_read_array::<T>(buffer)).collect()
    }

    // NBT uses a plain `u32` for the length instead of the usual variable encoding
    #[expect(clippy::cast_possible_truncation)]
    fn frog_write_array<T: FrogWrite>(
        items: &[T],
        buffer: &mut impl Write,
    ) -> Result<usize, WriteError> {
        items.iter().try_fold((items.len() as u32).frog_write(buffer)?, |acc, item| {
            item.frog_write(buffer).map(|item| acc + item)
        })
    }

    #[expect(clippy::cast_possible_truncation)]
    fn frog_write_array_array<T: FrogWrite>(
        items: &[Vec<T>],
        buffer: &mut impl Write,
    ) -> Result<usize, WriteError> {
        items.iter().try_fold((items.len() as u32).frog_write(buffer)?, |acc, item| {
            Self::frog_write_array(item, buffer).map(|item| acc + item)
        })
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
    fn frog_len(&self) -> usize { std::mem::size_of::<u16>() + self.as_bytes().len() }
}
