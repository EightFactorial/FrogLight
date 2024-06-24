use std::{
    hash::{BuildHasher, Hash},
    io::Write,
};

use super::{FrogWrite, WriteError};
use crate::protocol::FrogVarWrite;

impl<K: FrogWrite + Eq + Hash, V: FrogWrite, S: BuildHasher> FrogWrite
    for std::collections::HashMap<K, V, S>
{
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Map too large").fg_var_write(buf)?;
        for (k, v) in self {
            k.fg_write(buf)?;
            v.fg_write(buf)?;
        }
        Ok(())
    }
}

#[cfg(feature = "hashbrown")]
impl<K: FrogWrite + Eq + Hash, V: FrogWrite, S: BuildHasher> FrogWrite
    for hashbrown::HashMap<K, V, S>
{
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Map too large").fg_var_write(buf)?;
        for (k, v) in self {
            k.fg_write(buf)?;
            v.fg_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogWrite + Eq + Hash, S: BuildHasher> FrogWrite for std::collections::HashSet<T, S> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Set too large").fg_var_write(buf)?;
        for k in self {
            k.fg_write(buf)?;
        }
        Ok(())
    }
}

#[cfg(feature = "hashbrown")]
impl<T: FrogWrite + Eq + Hash, S: BuildHasher> FrogWrite for hashbrown::HashSet<T, S> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Set too large").fg_var_write(buf)?;
        for k in self {
            k.fg_write(buf)?;
        }
        Ok(())
    }
}

impl<K: FrogWrite + Ord, V: FrogWrite> FrogWrite for std::collections::BTreeMap<K, V> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Map too large").fg_var_write(buf)?;
        for (k, v) in self {
            k.fg_write(buf)?;
            v.fg_write(buf)?;
        }
        Ok(())
    }
}

impl<T: FrogWrite + Ord> FrogWrite for std::collections::BTreeSet<T> {
    #[inline]
    fn fg_write(&self, buf: &mut (impl Write + ?Sized)) -> Result<(), WriteError> {
        u32::try_from(self.len()).expect("Set too large").fg_var_write(buf)?;
        for k in self {
            k.fg_write(buf)?;
        }
        Ok(())
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_write_hashmap_u8_u8(data in proptest::collection::hash_map(
        proptest::num::u8::ANY,
        proptest::num::u8::ANY,
        0..128,
    )) {
        let mut bytes = Vec::with_capacity(data.len() * 2 + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for (k, v) in &data {
            k.fg_write(&mut bytes).unwrap();
            v.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_hashmap_string_u8(data in proptest::collection::hash_map(
        ".*",
        proptest::num::u8::ANY,
        0..128,
    )) {
        let mut bytes = Vec::with_capacity(data.len() * 2 + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for (k, v) in &data {
            k.fg_write(&mut bytes).unwrap();
            v.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_hashset_u8(data in proptest::collection::hash_set(proptest::num::u8::ANY, 0..128)) {
        let mut bytes = Vec::with_capacity(data.len() + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for k in &data {
            k.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_hashset_option_u16(data in proptest::collection::hash_set(
        proptest::option::of(proptest::num::u16::ANY),
        0..128,
    )) {
        let mut bytes = Vec::with_capacity(data.len() * 3 + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for k in &data {
            k.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_btreemap_u8_u8(data in proptest::collection::btree_map(
        proptest::num::u8::ANY,
        proptest::num::u8::ANY,
        0..128,
    )) {
        let mut bytes = Vec::with_capacity(data.len() * 2 + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for (k, v) in &data {
            k.fg_write(&mut bytes).unwrap();
            v.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_btreemap_i64_i8(data in proptest::collection::btree_map(
        proptest::num::i64::ANY,
        proptest::num::i8::ANY,
        0..128,
    )) {
        let mut bytes = Vec::with_capacity(data.len() * 9 + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for (k, v) in &data {
            k.fg_write(&mut bytes).unwrap();
            v.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_btreeset_u8(data in proptest::collection::btree_set(proptest::num::u8::ANY, 0..128)) {
        let mut bytes = Vec::with_capacity(data.len() + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for k in &data {
            k.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }

    #[test]
    fn proto_write_btreeset_string(data in proptest::collection::btree_set(".*", 0..128)) {
        let mut bytes = Vec::with_capacity(data.len() + 1);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut bytes).unwrap();
        for k in &data {
            k.fg_write(&mut bytes).unwrap();
        }

        assert_eq!(data.fg_to_bytes(), bytes);
    }
}
