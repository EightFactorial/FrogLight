use std::{marker::PhantomData, num::NonZeroU8};

use froglight_common::version::V1_21_4;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_item::storage::GlobalItemId;
use froglight_nbt::nbt::{NbtCompound, UnnamedNbt};

use super::{RawInventorySlot, RawInventorySlotRef};
use crate::slot::component::InventoryComponents;

impl FrogReadVersion<V1_21_4> for RawInventorySlot<V1_21_4> {
    fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, ReadError> {
        match u32::frog_var_read(buffer)? {
            0 => Ok(Self(None, PhantomData)),
            count => {
                #[expect(clippy::cast_possible_truncation)]
                let count = NonZeroU8::new(count as u8).unwrap();
                let global = GlobalItemId::new_unchecked(u32::frog_var_read(buffer)?);

                let add_len = u32::frog_var_read(buffer)? as usize;
                let rem_len = u32::frog_var_read(buffer)? as usize;

                let mut nbt = UnnamedNbt::new_empty();

                // Read the added component data.
                {
                    let components = &*InventoryComponents::read::<V1_21_4>();
                    for _ in 0..add_len {
                        let index = u32::frog_var_read(buffer)? as usize;
                        // Get the component's reader by index.
                        match components.get_index(index) {
                            None => panic!("Unknown `InventoryComponent` ID: {index}"),
                            Some((ident, fns)) => {
                                // Read the component data and insert it into `nbt`.
                                match fns.frog_read(buffer) {
                                    Ok(tag) => nbt.insert(ident.as_str(), tag),
                                    Err(err) => {
                                        panic!(
                                            "Failed to read `InventoryComponent` \"{ident}\": {err}"
                                        )
                                    }
                                };
                            }
                        }
                    }
                }

                // Read the removed component ids.
                let mut removed = Vec::with_capacity(rem_len);
                for _ in 0..rem_len {
                    removed.push(u32::frog_var_read(buffer)?);
                }

                Ok(Self(Some((count, global, nbt, removed)), PhantomData))
            }
        }
    }
}

impl FrogWriteVersion<V1_21_4> for RawInventorySlot<V1_21_4> {
    #[inline]
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        RawInventorySlotRef::from_raw(self).frog_write(buffer)
    }

    #[inline]
    fn frog_len(&self) -> usize { RawInventorySlotRef::from_raw(self).frog_len() }
}

impl FrogWriteVersion<V1_21_4> for RawInventorySlotRef<'_, V1_21_4> {
    fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, WriteError> {
        match &self.0 {
            Some((count, global, data, removed)) => {
                let mut written = 0;

                written += u32::from(count.get()).frog_var_write(buffer)?;
                written += u32::from(*global).frog_var_write(buffer)?;

                #[expect(clippy::cast_possible_truncation)]
                {
                    let count = data.compound().map_or(0, NbtCompound::len);
                    written += u32::frog_var_write(&(count as u32), buffer)?;
                    written += u32::frog_var_write(&(removed.len() as u32), buffer)?;
                }

                // Write the added component data.
                {
                    let components = &*InventoryComponents::read::<V1_21_4>();
                    for (ident, tag) in data.compound().into_iter().flatten() {
                        let ident = ident.to_str_lossy();
                        // Get the component's index by identifier.
                        match components.get_full(ident.as_ref()) {
                            None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                            #[expect(clippy::cast_possible_truncation)]
                            Some((index, _, fns)) => {
                                written += u32::frog_var_write(&(index as u32), buffer)?;
                                written += fns.frog_write(tag, buffer)?;
                            }
                        }
                    }
                }

                // Write the removed component ids.
                {
                    for removed in removed.as_ref() {
                        written += u32::frog_var_write(removed, buffer)?;
                    }
                }

                Ok(written)
            }
            None => u32::frog_var_write(&0, buffer),
        }
    }

    fn frog_len(&self) -> usize {
        match &self.0 {
            Some((count, global, data, removed)) => {
                let mut length = 0;

                length += u32::from(count.get()).frog_var_len();
                length += u32::from(*global).frog_var_len();

                #[expect(clippy::cast_possible_truncation)]
                {
                    let count = data.compound().map_or(0, NbtCompound::len);
                    length += u32::frog_var_len(&(count as u32));
                    length += u32::frog_var_len(&(removed.len() as u32));
                }

                // Add the added component data lengths
                {
                    let components = &*InventoryComponents::read::<V1_21_4>();
                    for (ident, tag) in data.compound().into_iter().flatten() {
                        let ident = ident.to_str_lossy();
                        // Get the component's index by identifier.
                        match components.get_full(ident.as_ref()) {
                            None => panic!("Unknown `InventoryComponent` \"{ident}\""),
                            #[expect(clippy::cast_possible_truncation)]
                            Some((index, _, fns)) => {
                                length += u32::frog_var_len(&(index as u32));
                                length += fns.frog_len(tag);
                            }
                        }
                    }
                }

                // Add the removed component ids lengths
                {
                    for removed in removed.as_ref() {
                        length += u32::frog_var_len(removed);
                    }
                }

                length
            }
            None => u32::frog_var_len(&0),
        }
    }
}
