use std::hash::{BuildHasher, Hash};

use crate::io::{FrogRead, FrogVarRead};

impl<K: Eq + Hash + FrogRead, V: FrogRead, S: Default + BuildHasher> FrogRead
    for std::collections::HashMap<K, V, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<K: Eq + Hash + FrogRead, V: FrogRead, S: Default + BuildHasher> FrogRead
    for hashbrown::HashMap<K, V, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<T: Eq + Hash + FrogRead, S: Default + BuildHasher> FrogRead
    for std::collections::HashSet<T, S>
{
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(set)
    }
}

impl<T: Eq + Hash + FrogRead, S: Default + BuildHasher> FrogRead for hashbrown::HashSet<T, S> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(set)
    }
}

impl<K: Ord + FrogRead, V: FrogRead> FrogRead for std::collections::BTreeMap<K, V> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
                }
            }
        }

        Ok(map)
    }
}

impl<T: Ord + FrogRead> FrogRead for std::collections::BTreeSet<T> {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
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
                    return Err(crate::io::ReadError::ListError(len, i, Box::new(err)));
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
    fn proto_read_hashmap(data in proptest::collection::hash_map(0u8..=255u8, 0u8..=255u8, 0..64)) {
        use crate::io::var_write::FrogVarWrite;

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
    fn proto_read_hashset(data in proptest::collection::hash_set(0u8..=255u8, 0..64)) {
        use crate::io::var_write::FrogVarWrite;

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
    fn proto_read_btreemap(data in proptest::collection::btree_map(0u8..=255u8, 0u8..=255u8, 0..64)) {
        use crate::io::var_write::FrogVarWrite;

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
    fn proto_read_btreeset(data in proptest::collection::btree_set(0u8..=255u8, 0..64)) {
        use crate::io::var_write::FrogVarWrite;

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
    fn proto_read_hashmap_hashbrown(data in proptest::collection::hash_map(0u8..=255u8, 0u8..=255u8, 0..64)) {
        use crate::io::var_write::FrogVarWrite;

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

}
