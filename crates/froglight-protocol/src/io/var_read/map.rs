use std::hash::{BuildHasher, Hash};

use crate::io::{FrogRead, FrogVarRead};

impl<K: Eq + Hash + FrogRead, V: FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for std::collections::HashMap<K, V, S>
{
    fn frog_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut map = std::collections::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::frog_read(buf), V::frog_var_read(buf)) {
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

impl<K: Eq + Hash + FrogRead, V: FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for hashbrown::HashMap<K, V, S>
{
    fn frog_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut map = hashbrown::HashMap::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match (K::frog_read(buf), V::frog_var_read(buf)) {
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

impl<T: Eq + Hash + FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for std::collections::HashSet<T, S>
{
    fn frog_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut set = std::collections::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::frog_var_read(buf) {
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

impl<T: Eq + Hash + FrogVarRead, S: Default + BuildHasher> FrogVarRead
    for hashbrown::HashSet<T, S>
{
    fn frog_var_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
    where
        Self: Sized,
    {
        let len = usize::try_from(u32::frog_var_read(buf)?).expect("Map too large");
        let mut set = hashbrown::HashSet::with_capacity_and_hasher(len, S::default());

        for i in 0..len {
            match T::frog_var_read(buf) {
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
