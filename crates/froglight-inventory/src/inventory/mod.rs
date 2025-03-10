//! TODO

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use froglight_common::version::Version;
use froglight_item::item::UntypedItem;

mod equipment;
pub use equipment::EquipmentInventory;

mod player;
pub use player::{PlayerInventory, PlayerInventoryMenu};

/// A slot in an inventory.
///
/// May contain an item or be empty.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, AsRef, AsMut)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Default, PartialEq))]
#[cfg_attr(feature = "bevy", derive(Component), reflect(Component))]
pub struct InventorySlot<V: Version>(Option<UntypedItem<V>>);

impl<V: Version> InventorySlot<V> {
    /// Create a new empty [`InventorySlot`].
    #[must_use]
    pub const fn new() -> Self { Self(None) }
}
