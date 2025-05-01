use bevy_platform::hash::FixedHasher;
use froglight_common::version::V1_21_5;

use super::{ComponentMap, VersionComponents};

impl VersionComponents for V1_21_5 {
    fn components() -> ComponentMap { ComponentMap::with_hasher(FixedHasher) }
}
