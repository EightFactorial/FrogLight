use std::hash::{BuildHasher, Hash};

use crate::io::{FrogRead, FrogVarRead};

impl<K: Eq + Hash + FrogRead, V: FrogRead, S: Default + BuildHasher> FrogRead
    for std::collections::HashMap<K, V, S>
{
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::frog_read(buf), V::frog_read(buf)) {
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
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut map = hashbrown::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::frog_read(buf), V::frog_read(buf)) {
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
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::frog_read(buf) {
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
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut set = hashbrown::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::frog_read(buf) {
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
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::BTreeMap::new();

        for i in 0..len {
            match (K::frog_read(buf), V::frog_read(buf)) {
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
    fn frog_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::BTreeSet::new();

        for i in 0..len {
            match T::frog_read(buf) {
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

#[test]
fn proto_read_hashmap() {
    let mut cursor = std::io::Cursor::new([0].as_slice());
    let map: std::collections::HashMap<u8, u8> =
        std::collections::HashMap::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(map.len(), 0);

    let mut cursor = std::io::Cursor::new([1, 0, 0].as_slice());
    let map: std::collections::HashMap<u8, u8> =
        std::collections::HashMap::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&0).unwrap(), &0);

    let mut cursor = std::io::Cursor::new([1, 0, 0, 0, 0, 0, 0, 0, 1].as_slice());
    let map: std::collections::HashMap<u32, u32> =
        std::collections::HashMap::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&0).unwrap(), &1);

    let mut cursor =
        std::io::Cursor::new([2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0].as_slice());
    let map: std::collections::HashMap<u32, u32> =
        std::collections::HashMap::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&0).unwrap(), &0);
    assert_eq!(map.get(&1).unwrap(), &0);
}

#[test]
fn proto_read_hashset() {
    let mut cursor = std::io::Cursor::new([0].as_slice());
    let set: std::collections::HashSet<u8> =
        std::collections::HashSet::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(set.len(), 0);

    let mut cursor = std::io::Cursor::new([1, 0].as_slice());
    let set: std::collections::HashSet<u8> =
        std::collections::HashSet::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(set.len(), 1);
    assert!(set.contains(&0));

    let mut cursor = std::io::Cursor::new([1, 0, 0, 0, 0].as_slice());
    let set: std::collections::HashSet<u32> =
        std::collections::HashSet::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(set.len(), 1);
    assert!(set.contains(&0));

    let mut cursor = std::io::Cursor::new([2, 0, 0, 0, 0, 0, 0, 0, 1].as_slice());
    let set: std::collections::HashSet<u32> =
        std::collections::HashSet::frog_read(&mut cursor).unwrap();
    assert_eq!(cursor.position(), cursor.get_ref().len() as u64);

    assert_eq!(set.len(), 2);
    assert!(set.contains(&0));
    assert!(set.contains(&1));
}
