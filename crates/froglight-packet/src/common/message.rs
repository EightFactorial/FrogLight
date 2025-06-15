#![expect(missing_docs)]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_utils::bitset::FixedBitSet;
use smol_str::SmolStr;

/// A message signature.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct MessageSignature([u8; 256]);

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct SignedCommandArgument {
    pub name: SmolStr,
    pub signature: MessageSignature,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf))]
pub struct SeenMessagesUpdate {
    pub message_count: u32,
    /// A bitmask of the last 20 messages used to sign this message.
    pub seen_messages: FixedBitSet<20>,
}
