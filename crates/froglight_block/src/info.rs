//! TODO

#[cfg(feature = "nightly")]
use core::any::TypeId;
use core::{
    fmt::Debug,
    ops::{Deref, Index},
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

use froglight_common::{atomic::AtomicF32, identifier::Identifier, version::Version};

use crate::{attribute::BlockAttributes, block::BlockType};

/// Information about a [`BlockType`].
#[derive(Debug)]
pub struct BlockInfo {
    /// The block's identifier.
    identifier: Identifier,
    /// The block's definition.
    definition: BlockDefinitionMap,
    /// The block's settings.
    settings: BlockSettings,

    /// The [`TypeId`] of the block, if available.
    #[cfg(feature = "nightly")]
    block_type: TypeId,
    /// The [`TypeId`] of the version, if available.
    #[cfg(feature = "nightly")]
    version_type: TypeId,

    /// The default state of the block.
    default_state: u16,
    /// The base registered id of the block.
    registered_id: AtomicU32,
    /// The total number of states this block has.
    total_states: usize,

    attr_names: &'static [&'static str],
    get_attr_fn: fn(u16, usize) -> Option<&'static str>,
    set_attr_fn: fn(u16, usize, &'static str) -> Option<(u16, &'static str)>,
}

impl BlockInfo {
    /// Create a new [`BlockInfo`] for a [`BlockType`].
    #[must_use]
    pub const fn new<B: BlockType<V>, V: Version>(
        default_state: u16,
        identifier: &'static str,
        settings: BlockSettings,
        definition: BlockDefinitionMap,
    ) -> Self {
        Self {
            identifier: Identifier::new_static(identifier),
            definition,
            settings,

            #[cfg(feature = "nightly")]
            block_type: TypeId::of::<B>(),
            #[cfg(feature = "nightly")]
            version_type: TypeId::of::<V>(),

            default_state,
            registered_id: AtomicU32::new(u32::MAX),
            total_states: <B as BlockType<V>>::Attributes::COUNT,

            attr_names: <B as BlockType<V>>::ATTRIBUTE_NAMES,
            get_attr_fn: |block_state: u16, attr_index: usize| {
                <B as BlockType<V>>::Attributes::from_state(usize::from(block_state))
                    .and_then(|attrs| attrs.get_attr_str(attr_index))
            },
            #[expect(
                clippy::cast_possible_truncation,
                reason = "A block will never have 4,294,967,295 states"
            )]
            set_attr_fn: |block_state: u16, attr_index: usize, attr_value: &'static str| {
                <B as BlockType<V>>::Attributes::from_state(usize::from(block_state)).and_then(
                    |mut attrs| {
                        let previous = attrs.set_attr_str(attr_index, attr_value)?;
                        attrs.into_index().map(|index| (index as u16, previous))
                    },
                )
            },
        }
    }

    /// Get the block's [`Identifier`].
    #[inline]
    #[must_use]
    pub const fn identifier(&self) -> &Identifier { &self.identifier }

    /// Get the block's [`BlockDefinition`].
    #[inline]
    #[must_use]
    pub const fn definition(&self) -> &BlockDefinitionMap { &self.definition }

    /// Get the block's [`BlockSettings`].
    #[inline]
    #[must_use]
    pub const fn settings(&self) -> &BlockSettings { &self.settings }

    /// Returns `true` if this block is of the given type.
    #[inline]
    #[must_use]
    #[cfg(feature = "nightly")]
    pub fn is_block<B: BlockType<V>, V: Version>(&self) -> bool {
        TypeId::of::<B>() == self.block_type && TypeId::of::<V>() == self.version_type
    }

    /// Get the default state of the block.
    #[inline]
    #[must_use]
    pub fn default_state(&self) -> u16 { self.default_state }

    /// Get the base registered id of the block.
    ///
    /// The first id assigned to the block after it has been
    /// registered, not the default id.
    ///
    /// # Panics
    ///
    /// Panics if the [`BlockInfo`] has not been registered yet.
    #[must_use]
    pub fn base_id(&self) -> u32 {
        let base = self.registered_id.load(Ordering::Relaxed);
        if base == u32::MAX {
            panic!("BlockInfo has not been registered yet!");
        } else {
            base
        }
    }

    /// Get the total number of states this block can have.
    #[inline]
    #[must_use]
    pub const fn states(&self) -> usize { self.total_states }

    /// Get the names of the block's attributes, in order.
    #[inline]
    #[must_use]
    pub const fn attributes(&self) -> &'static [&'static str] { self.attr_names }

    /// Set the block's registered id.
    ///
    /// # Panics
    ///
    /// Panics if the [`BlockInfo`] already has a registered id.
    pub(super) fn set_registered_id(&self, block_id: u32) {
        if self.registered_id.load(Ordering::Relaxed) == u32::MAX {
            self.registered_id.store(block_id, Ordering::Relaxed);
        } else {
            panic!("BlockInfo already has a registered id!");
        }
    }

    /// Get the string attribute value for a given attribute name.
    #[must_use]
    pub fn get_attr_name(&self, block_state: u16, attr_name: &str) -> Option<&'static str> {
        let index = self.attr_names.iter().position(|&name| name == attr_name)?;
        self.get_attr_index(block_state, index)
    }

    /// Get the string attribute value for a given attribute index.
    #[inline]
    #[must_use]
    pub fn get_attr_index(&self, block_state: u16, attr_index: usize) -> Option<&'static str> {
        (self.get_attr_fn)(block_state, attr_index)
    }

    /// Set the string attribute value for a given attribute name.
    ///
    /// Returns a new block state and the previous attribute value,
    /// or `None` if the attribute could not be set.
    #[must_use]
    pub fn set_attr_name(
        &self,
        block_state: u16,
        attr_name: &str,
        attr_value: &'static str,
    ) -> Option<(u16, &'static str)> {
        let index = self.attr_names.iter().position(|&name| name == attr_name)?;
        self.set_attr_index(block_state, index, attr_value)
    }

    /// Set the string attribute value for a given index.
    ///
    /// Returns a new block state and the previous attribute value,
    /// or `None` if the attribute could not be set.
    #[inline]
    #[must_use]
    pub fn set_attr_index(
        &self,
        block_state: u16,
        attr_index: usize,
        attr_value: &'static str,
    ) -> Option<(u16, &'static str)> {
        (self.set_attr_fn)(block_state, attr_index, attr_value)
    }
}

impl PartialEq for BlockInfo {
    #[cfg(feature = "nightly")]
    fn eq(&self, other: &Self) -> bool {
        self.block_type == other.block_type && self.version_type == other.version_type
    }

    #[cfg(not(feature = "nightly"))]
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
            && self.total_states == other.total_states
            && self.base_id() == other.base_id()
    }
}
impl Eq for BlockInfo {}

// -------------------------------------------------------------------------------------------------

/// A block definition object.
///
/// Used to define block properties in a static way.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockDefinition {
    /// A map of strings to definition objects
    Map(BlockDefinitionMap),
    /// A list of definition objects
    List(&'static [BlockDefinition]),
    /// A string value
    String(&'static str),
    /// An integer value
    Integer(i64),
    /// A float value
    Float(f64),
}

impl BlockDefinition {
    /// If the values is a [`BlockDefinition::Map`], returns the map.
    #[must_use]
    pub const fn as_map(&self) -> Option<&BlockDefinitionMap> {
        if let BlockDefinition::Map(map) = self { Some(map) } else { None }
    }

    /// If the value is a [`BlockDefinition::Map`],
    /// use the string key and get the corresponding value.
    #[must_use]
    pub fn get_key(&self, key: &str) -> Option<&BlockDefinition> {
        self.as_map().and_then(|map| map.get(key))
    }

    /// If the value is a [`BlockDefinition::List`], returns the list.
    #[must_use]
    pub const fn as_list(&self) -> Option<&'static [BlockDefinition]> {
        if let BlockDefinition::List(list) = self { Some(list) } else { None }
    }

    /// If the value is a [`BlockDefinition::List`],
    /// use the index to get the corresponding value.
    #[must_use]
    pub fn get_index(&self, index: usize) -> Option<&BlockDefinition> {
        self.as_list().and_then(|list| list.get(index))
    }

    /// If the value is a [`BlockDefinition::String`], returns the string.
    #[must_use]
    pub const fn as_string(&self) -> Option<&'static str> {
        if let BlockDefinition::String(value) = self { Some(value) } else { None }
    }

    /// If the value is a [`BlockDefinition::Integer`], returns the integer.
    #[must_use]
    pub const fn as_integer(&self) -> Option<i64> {
        if let BlockDefinition::Integer(value) = self { Some(*value) } else { None }
    }

    /// If the value is a [`BlockDefinition::Float`], returns the float.
    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        if let BlockDefinition::Float(value) = self { Some(*value) } else { None }
    }
}

impl<'a> Index<&'a str> for &BlockDefinition {
    type Output = BlockDefinition;

    #[inline]
    fn index(&self, index: &'a str) -> &Self::Output {
        <BlockDefinition as Index<&'a str>>::index(self, index)
    }
}
impl<'a> Index<&'a str> for BlockDefinition {
    type Output = BlockDefinition;

    fn index(&self, index: &'a str) -> &Self::Output {
        if let BlockDefinition::Map(map) = self {
            <BlockDefinitionMap as Index<&'a str>>::index(map, index)
        } else {
            panic!("Cannot index into non-map block definition objects")
        }
    }
}

impl Index<usize> for &BlockDefinition {
    type Output = BlockDefinition;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        <BlockDefinition as Index<usize>>::index(self, index)
    }
}
impl Index<usize> for BlockDefinition {
    type Output = BlockDefinition;

    fn index(&self, index: usize) -> &Self::Output {
        if let BlockDefinition::List(list) = self {
            list.get(index).expect("Block definition list index out of bounds")
        } else {
            panic!("Cannot usize index into non-list block definition objects")
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A map of block definition objects.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockDefinitionMap(pub &'static [(&'static str, BlockDefinition)]);

impl BlockDefinitionMap {
    /// Use the string key and get the corresponding value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&BlockDefinition> {
        self.iter().find_map(|(k, v)| if k == &key { Some(v) } else { None })
    }
}

impl Deref for BlockDefinitionMap {
    type Target = [(&'static str, BlockDefinition)];

    #[inline]
    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'a> Index<&'a str> for &BlockDefinitionMap {
    type Output = BlockDefinition;

    #[inline]
    fn index(&self, index: &'a str) -> &Self::Output {
        <BlockDefinitionMap as Index<&'a str>>::index(self, index)
    }
}
impl<'a> Index<&'a str> for BlockDefinitionMap {
    type Output = BlockDefinition;

    fn index(&self, index: &'a str) -> &Self::Output {
        if let Some((_, result)) = self.0.iter().find(|(key, _)| *key == index) {
            result
        } else {
            panic!("No definition value found for key: \"{index}\"")
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Modifiable block settings.
#[derive(Debug)]
pub struct BlockSettings {
    /// Whether this block is air.
    pub is_air: AtomicBool,
    /// Whether this block is solid.
    pub is_solid: AtomicBool,
    /// Whether this block is transparent.
    pub is_transparent: AtomicBool,

    /// The block's friction.
    pub friction: AtomicF32,
    /// The block's jump factor.
    pub jump_factor: AtomicF32,

    /// The time it takes to destroy this block.
    pub break_speed: AtomicF32,
    /// The explosion resistance of this block.
    pub explosion_resistance: AtomicF32,
}

impl BlockSettings {
    /// Create a new [`BlockSettings`].
    #[must_use]
    pub const fn new(
        is_air: bool,
        is_solid: bool,
        is_transparent: bool,

        friction: f32,
        jump_factor: f32,

        break_speed: f32,
        explosion_resistance: f32,
    ) -> Self {
        Self {
            is_air: AtomicBool::new(is_air),
            is_solid: AtomicBool::new(is_solid),
            is_transparent: AtomicBool::new(is_transparent),

            friction: AtomicF32::new(friction),
            jump_factor: AtomicF32::new(jump_factor),

            break_speed: AtomicF32::new(break_speed),
            explosion_resistance: AtomicF32::new(explosion_resistance),
        }
    }

    /// Get whether this block is air.
    #[inline]
    #[must_use]
    pub fn is_air(&self) -> bool { self.is_air.load(Ordering::Relaxed) }

    /// Set whether this block is air.
    #[inline]
    pub fn set_air(&self, is_air: bool) { self.is_air.store(is_air, Ordering::Relaxed); }

    /// Get whether this block is solid.
    #[inline]
    #[must_use]
    pub fn is_solid(&self) -> bool { self.is_solid.load(Ordering::Relaxed) }

    /// Set whether this block is solid.
    #[inline]
    pub fn set_solid(&self, is_solid: bool) { self.is_solid.store(is_solid, Ordering::Relaxed); }

    /// Get whether this block is transparent.
    #[inline]
    #[must_use]
    pub fn is_transparent(&self) -> bool { self.is_transparent.load(Ordering::Relaxed) }

    /// Set whether this block is transparent.
    #[inline]
    pub fn set_transparent(&self, is_transparent: bool) {
        self.is_transparent.store(is_transparent, Ordering::Relaxed);
    }
}

impl Default for BlockSettings {
    fn default() -> Self { Self::new(false, true, false, 0.6, 1.0, 0.0, 0.0) }
}
