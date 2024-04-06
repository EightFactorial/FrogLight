use std::hash::{BuildHasher, Hash};

use bevy_utils::hashbrown;

use crate::protocol::{FrogRead, FrogVarRead};

impl<K: Eq + Hash + FrogRead, V: FrogRead, S: Default + BuildHasher> FrogRead
    for std::collections::HashMap<K, V, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::fg_read(buf), V::fg_read(buf)) {
                (Ok(key), Ok(value)) => {
                    map.insert(key, value);
                }
                (Err(err), _) | (_, Err(err)) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<K: Eq + Hash + FrogRead, V: FrogRead, S: Default + BuildHasher> FrogRead
    for hashbrown::HashMap<K, V, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut map = hashbrown::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::fg_read(buf), V::fg_read(buf)) {
                (Ok(key), Ok(value)) => {
                    map.insert(key, value);
                }
                (Err(err), _) | (_, Err(err)) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<T: Eq + Hash + FrogRead, S: Default + BuildHasher> FrogRead
    for std::collections::HashSet<T, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => {
                    set.insert(value);
                }
                Err(err) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(set)
    }
}

impl<T: Eq + Hash + FrogRead, S: Default + BuildHasher> FrogRead for hashbrown::HashSet<T, S> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut set = hashbrown::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => {
                    set.insert(value);
                }
                Err(err) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(set)
    }
}

impl<K: Ord + FrogRead, V: FrogRead> FrogRead for std::collections::BTreeMap<K, V> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::BTreeMap::new();

        for i in 0..len {
            match (K::fg_read(buf), V::fg_read(buf)) {
                (Ok(key), Ok(value)) => {
                    map.insert(key, value);
                }
                (Err(err), _) | (_, Err(err)) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<T: Ord + FrogRead> FrogRead for std::collections::BTreeSet<T> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::BTreeSet::new();

        for i in 0..len {
            match T::fg_read(buf) {
                Ok(value) => {
                    set.insert(value);
                }
                Err(err) => {
                    return Err(crate::protocol::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(set)
    }
}

#[cfg(test)]
proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(128))]

    #[test]
    fn proto_read_hashmap_u8_u8(data in proptest::collection::hash_map(proptest::bits::u8::ANY, proptest::bits::u8::ANY, 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len() * 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for (key, val) in &data {
            vec.extend_from_slice(&key.to_be_bytes());
            vec.extend_from_slice(&val.to_be_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let map = std::collections::HashMap::fg_read(&mut cursor).unwrap();

        assert_eq!(map, data);
    }

    #[test]
    fn proto_read_hashmap_u32_string(data in proptest::collection::hash_map(proptest::bits::u32::ANY, ".*", 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len() * 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for (key, val) in &data {
            vec.extend_from_slice(&key.to_be_bytes());

            u32::try_from(val.len()).unwrap().fg_var_write(&mut vec).unwrap();
            vec.extend_from_slice(val.as_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let map = std::collections::HashMap::fg_read(&mut cursor).unwrap();

        assert_eq!(map, data);
    }

    #[test]
    fn proto_read_hashset_u16(data in proptest::collection::hash_set(proptest::bits::u16::ANY, 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len());
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for val in &data {
            vec.extend_from_slice(&val.to_be_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let set = std::collections::HashSet::fg_read(&mut cursor).unwrap();

        assert_eq!(set, data);
    }

    #[test]
    fn proto_read_hashset_string(data in proptest::collection::hash_set(".*", 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len());
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for val in &data {
            u32::try_from(val.len()).unwrap().fg_var_write(&mut vec).unwrap();
            vec.extend_from_slice(val.as_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let set = std::collections::HashSet::fg_read(&mut cursor).unwrap();

        assert_eq!(set, data);
    }

    #[test]
    fn proto_read_btreemap_u8_u8(data in proptest::collection::btree_map(proptest::bits::u8::ANY, proptest::bits::u8::ANY, 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len() * 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for (key, val) in &data {
            vec.extend_from_slice(&key.to_be_bytes());
            vec.extend_from_slice(&val.to_be_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let map = std::collections::BTreeMap::fg_read(&mut cursor).unwrap();

        assert_eq!(map, data);
    }

    #[test]
    fn proto_read_btreeset_u8(data in proptest::collection::btree_set(proptest::bits::u8::ANY, 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len());
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for val in &data {
            vec.extend_from_slice(&val.to_be_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let set = std::collections::BTreeSet::fg_read(&mut cursor).unwrap();

        assert_eq!(set, data);
    }

    #[test]
    fn proto_read_hashmap_hashbrown_u8_u8(data in proptest::collection::hash_map(proptest::bits::u8::ANY, proptest::bits::u8::ANY, 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len() * 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for (key, val) in &data {
            vec.extend_from_slice(&key.to_be_bytes());
            vec.extend_from_slice(&val.to_be_bytes());
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let map = hashbrown::HashMap::<u8, u8>::fg_read(&mut cursor).unwrap();

        // Assert the map length and contents
        assert_eq!(map.len(), data.len());
        for (key, val) in &data {
            assert_eq!(map.get(key).unwrap(), val);
        }
    }

    #[test]
    fn proto_read_hashmap_hashbrown_string_op_u32(data in proptest::collection::hash_map(".*", proptest::option::of(proptest::bits::u32::ANY), 0..64)) {
        use crate::protocol::var_write::FrogVarWrite;

        // Prefix the data with the length
        let mut vec = Vec::with_capacity(data.len() * 2);
        u32::try_from(data.len()).unwrap().fg_var_write(&mut vec).unwrap();

        // Write the data
        for (key, val) in &data {
            u32::try_from(key.len()).unwrap().fg_var_write(&mut vec).unwrap();
            vec.extend_from_slice(key.as_bytes());

            match val {
                Some(val) => {
                    vec.push(1);
                    vec.extend_from_slice(&val.to_be_bytes());
                }
                None => {
                    vec.push(0);
                }
            }
        }

        let mut cursor = std::io::Cursor::new(vec.as_slice());
        let map = hashbrown::HashMap::<String, Option<u32>>::fg_read(&mut cursor).unwrap();

        // Assert the map length and contents
        assert_eq!(map.len(), data.len());
        for (key, val) in &data {
            assert_eq!(map.get(key).unwrap(), val);
        }
    }
}
