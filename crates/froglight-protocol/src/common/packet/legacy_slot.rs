use froglight_macros::FrogReadWrite;
use simdnbt::owned::Nbt;

/// An item slot.
///
/// This is used to represent an item slot in the inventory.
///
/// # Note
/// This is used in versions before `1.20.4 (TODO)`.
#[derive(Debug, Default, Clone, PartialEq, FrogReadWrite)]
#[frog(tests = ["read_verify", "write_verify"], bytes = [0])]
pub enum LegacyItemSlot {
    /// An empty item slot.
    #[default]
    Empty,
    /// An item slot with an item in it.
    Item(LegacyItemSlotData),
}

impl LegacyItemSlot {
    /// Returns the number of items in the slot.
    #[must_use]
    #[inline]
    pub fn count(&self) -> i8 {
        match self {
            LegacyItemSlot::Empty => 0,
            LegacyItemSlot::Item(slot) => slot.count,
        }
    }

    /// Returns `true` if the item slot is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            LegacyItemSlot::Empty => true,
            LegacyItemSlot::Item(slot) => slot.is_empty(),
        }
    }

    /// Updates the slot to be empty if it is.
    #[inline]
    pub fn update_slot(&mut self) {
        if self.is_empty() {
            *self = LegacyItemSlot::Empty;
        }
    }
}

/// The data of an item slot.
///
/// # Note
/// This is used in versions before `1.20.4 (TODO)`.
#[derive(Debug, Clone, PartialEq, FrogReadWrite)]
// TODO: #[frog(tests = ["read_example"], bytes = [])]
pub struct LegacyItemSlotData {
    /// The kind of item in the slot.
    #[frog(var)]
    pub kind: u32,
    /// The number of items in the slot.
    pub count: i8,
    /// The item's data.
    pub data: Nbt,
}

impl LegacyItemSlotData {
    /// Creates a new item slot.
    #[must_use]
    #[inline]
    pub fn new(kind: u32, count: i8, data: Nbt) -> Self { Self { kind, count, data } }

    /// Returns `true` if the item slot is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool { self.count == 0 }
}
