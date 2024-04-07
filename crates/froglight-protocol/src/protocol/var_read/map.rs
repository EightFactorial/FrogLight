use std::hash::{BuildHasher, Hash};

use crate::protocol::{FrogRead, FrogVarRead};

impl<K: Eq + Hash + FrogRead, V: FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for std::collections::HashMap<K, V, S>
{
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::fg_read(buf), V::fg_var_read(buf)) {
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

impl<K: Eq + Hash + FrogRead, V: FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for hashbrown::HashMap<K, V, S>
{
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut map = hashbrown::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::fg_read(buf), V::fg_var_read(buf)) {
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

impl<T: Eq + Hash + FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for std::collections::HashSet<T, S>
{
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::fg_var_read(buf) {
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

impl<T: Eq + Hash + FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for hashbrown::HashSet<T, S>
{
    fn fg_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::protocol::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::fg_var_read(buf)?).expect("Map too large");
        let mut set = hashbrown::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::fg_var_read(buf) {
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

// TODO: Create tests for the above implementations, ideally using proptest
