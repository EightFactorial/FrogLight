#![allow(unit_bindings, clippy::let_unit_value, reason = "Placeholder")]

use froglight_common::prelude::Identifier;

use crate::version::ItemVersion;

/// Placeholder type for NBT data.
pub type Nbt = ();

/// Data about an [`Item`](crate::item::Item).
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ComponentData {
    raw: Nbt,
}

impl ComponentData {
    /// Create a new [`ComponentData`] from raw NBT data.
    #[inline]
    #[must_use]
    pub const fn new(raw: Nbt) -> Self { ComponentData { raw } }

    /// Get the component of the given type from this data.
    ///
    /// Returns `Ok(None)` if the component is not present.
    ///
    /// # Errors
    ///
    /// Returns an error if the component could not be read.
    pub fn get<C: ComponentType<V>, V: ItemVersion>(&self) -> Result<Option<C>, C::Error> {
        let component = self.raw; // .get(C::IDENTIFIER)?;
        C::from_nbt_data(&component).map(Some)
    }

    /// Set the component of the given type in this data.
    #[inline]
    pub fn set<C: ComponentType<V>, V: ItemVersion>(&mut self, _component: &C) {
        // self.nbt.insert(C::IDENTIFIER, component.to_nbt_data());
    }

    /// Get a reference to the raw NBT data.
    #[inline]
    #[must_use]
    pub const fn as_raw(&self) -> &Nbt { &self.raw }

    /// Get a mutable reference to the raw NBT data.
    #[inline]
    #[must_use]
    pub const fn as_raw_mut(&mut self) -> &mut Nbt { &mut self.raw }
}

// -------------------------------------------------------------------------------------------------

/// A trait implemented by all item component types.
pub trait ComponentType<V: ItemVersion>: Sized {
    /// The error type returned when trying to read [`Nbt`] as this type.
    type Error: Sized;
    /// The [`Identifier`] of this attribute type.
    const IDENTIFIER: Identifier<'static>;

    /// Try to convert [`Nbt`] into this type.
    ///
    /// # Errors
    ///
    /// Returns an error if the conversion fails.
    fn from_nbt_data(data: &Nbt) -> Result<Self, Self::Error>;

    /// Convert this type into [`Nbt`].
    fn to_nbt_data(&self) -> Nbt;
}
