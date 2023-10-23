#![allow(dead_code)]

use crate::{error::Result, Serializable};
use bitflags::bitflags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

impl Serializable for ClassAccessFlags {
    fn parse<R: Read>(rdr: &mut R) -> Result<Self> {
        let bits = rdr.read_u16::<BigEndian>()?;
        Ok(ClassAccessFlags::from_bits_truncate(bits))
    }

    fn write<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FieldAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const VOLATILE = 0x0040;
        const TRANSIENT = 0x0080;
        const SYNTHETIC = 0x1000;
        const ENUM = 0x4000;
    }
}

impl Serializable for FieldAccessFlags {
    fn parse<R: Read>(rdr: &mut R) -> Result<Self> {
        let bits = rdr.read_u16::<BigEndian>()?;
        Ok(FieldAccessFlags::from_bits_truncate(bits))
    }

    fn write<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MethodAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }
}

impl Serializable for MethodAccessFlags {
    fn parse<R: Read>(rdr: &mut R) -> Result<Self> {
        let bits = rdr.read_u16::<BigEndian>()?;
        Ok(MethodAccessFlags::from_bits_truncate(bits))
    }

    fn write<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}

bitflags! {
    pub struct InnerClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

impl Serializable for InnerClassAccessFlags {
    fn parse<R: Read>(rdr: &mut R) -> Result<Self> {
        let bits = rdr.read_u16::<BigEndian>()?;
        Ok(InnerClassAccessFlags::from_bits_truncate(bits))
    }

    fn write<W: Write>(&self, wtr: &mut W) -> Result<()> {
        wtr.write_u16::<BigEndian>(self.bits())?;
        Ok(())
    }
}
