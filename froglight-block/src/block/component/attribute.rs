use core::any::TypeId;

use crate::{
    attribute::BlockAttributeBundle,
    prelude::{BlockType, BlockVersion},
};

/// Data and functions that define a block's attributes.
#[derive(Clone, Copy)]
pub struct BlockAttributes {
    /// The number of states for this block type.
    ///
    /// All blocks have at least one state, even if they have no attributes.
    pub states: u16,
    /// The `(attribute_name, attribute_type)` pairs for this block type.
    pub list: &'static [(&'static str, TypeId)],
    /// A function to get the string value of an attribute.
    pub get_attr_fn: fn(state: usize, attr: &str) -> Option<&'static str>,
    /// A function to set the string value of an attribute.
    pub set_attr_fn: fn(state: usize, attr: &str, value: &str) -> Option<(usize, &'static str)>,
}

impl BlockAttributes {
    /// Create a new [`BlockAttribute`] for a given [`BlockType`].
    #[must_use]
    pub const fn new<B: BlockType<V>, V: BlockVersion>() -> Self {
        Self {
            states: B::Attributes::TOTAL,
            list: B::ATTRDATA,

            get_attr_fn: |state, name| {
                let attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTRDATA.iter().position(|&(attr, _)| attr == name)?;
                attributes.get_attr_str(index)
            },
            set_attr_fn: |state, name, value| {
                let mut attributes = B::Attributes::from_set_index(state)?;
                let index = B::ATTRDATA.iter().position(|&(attr, _)| attr == name)?;
                let old_value = attributes.set_attr_str(index, value)?;
                let new_state = attributes.to_set_index();
                Some((new_state, old_value))
            },
        }
    }
}
