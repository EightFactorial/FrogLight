use serde_json::Value;
use simdnbt::owned::{self, Nbt, NbtCompound, NbtTag};
use uuid::Uuid;

use super::{FrogRead, ReadError};

impl FrogRead for () {
    #[inline]
    fn fg_read(_: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError> { Ok(()) }
}

impl FrogRead for Nbt {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        owned::read(buf).map_err(ReadError::from)
    }
}

impl FrogRead for NbtCompound {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        NbtTag::fg_read(buf)?.into_compound().ok_or(ReadError::NbtCompoundError)
    }
}

impl FrogRead for NbtTag {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        owned::read_tag(buf).map_err(|e| ReadError::NbtError(e.into()))
    }
}

impl FrogRead for Uuid {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        Ok(Uuid::from_u128(u128::fg_read(buf)?))
    }
}

impl FrogRead for Value {
    #[inline]
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let content: String = String::fg_read(buf)?;
        Ok(serde_json::from_str(&content).unwrap_or(Value::Null))
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    // TODO: Test NBT read
    // #[test]
    // fn proto_read_nbt() {}

    /// All combinations of 16 bytes are valid UUIDs, so just test that the read
    /// function returns the same UUID as the one created from the bytes.
    #[test]
    fn proto_read_uuid(data in proptest::array::uniform16(proptest::num::u8::ANY)) {
        let mut cursor = std::io::Cursor::new(data.as_slice());

        let uuid = Uuid::from_slice(&data).unwrap();
        let read = Uuid::fg_read(&mut cursor).unwrap();

        assert_eq!(uuid, read);
        assert_eq!(cursor.position(), 16);
    }
}
