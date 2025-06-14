#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

use super::ClientboundPlayPackets;
use crate::common::UnsizedVec;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(no_field_bounds, Debug, Clone, PartialEq))]
#[cfg_attr(feature = "io", derive(froglight_macros::FrogBuf), frog(version))]
pub struct BundleDelimiterS2CPacket(UnsizedVec<ClientboundPlayPackets>);
