#![allow(clippy::wildcard_imports, unused_imports, reason = "Automatically @generated")]

use once_cell::sync::Lazy;

use super::item::*;
use crate::{info::*, item::*, storage::*};

generate! {
    @items froglight_common::version::V1_21_8,
    PlaceholderItem, "minecraft:placeholder", ItemComponentMap(&[])
}
