#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod manifest;
#[allow(unused_imports)]
pub(crate) use manifest::CrateManifest;

// ------------------- `froglight-block` -------------------

#[cfg(feature = "block")]
mod block;

/// Derive `frogligt_block::block::StaticBlock` for a struct.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::StaticBlock;
///
/// #[derive(StaticBlock)]
/// struct MyBlock;
///
/// // |
/// // V
///
/// impl froglight_block::block::StaticBlock for MyBlock {
///    fn as_static() -> &'static Self { &Self }
/// }
/// ```
#[cfg(feature = "block")]
#[proc_macro_derive(StaticBlock)]
pub fn derive_static_block(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    block::derive_static_block(input.into()).into()
}

/// Derive the appropriate traits on blocks.
#[cfg(feature = "block")]
#[proc_macro]
pub fn blocks(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    block::blocks(input.into()).into()
}

/// Derive the appropriate traits on block attributes.
#[cfg(feature = "block")]
#[proc_macro]
pub fn block_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    block::block_attributes(input.into()).into()
}

/// Derive the appropriate traits on block properties.
#[cfg(feature = "block")]
#[proc_macro]
pub fn block_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    block::block_properties(input.into()).into()
}

// ------------------- `froglight-common` -------------------

#[cfg(feature = "common")]
mod common;

/// Derive `froglight_common::version::Version` for a struct.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::Version;
///
/// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Version)]
/// #[version(protocol = 769, resource = 46)]
/// pub struct V1_21_4;
///
/// // |
/// // V
///
/// impl froglight_common::version::Version for V1_21_4 {
///     const PROTOCOL_ID: u32 = 769;
///     const RESOURCE_VERSION: u32 = 46;
/// }
#[cfg(feature = "common")]
#[proc_macro_derive(Version, attributes(version))]
pub fn derive_version(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    common::derive_version(input.into()).into()
}

// ------------------- `froglight-entity` -------------------

#[cfg(feature = "entity")]
mod entity;

/// Derive the appropriate traits on entity types.
#[proc_macro]
#[cfg(feature = "entity")]
pub fn entity_types(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::entity_types(input.into()).into()
}

/// Derive the appropriate traits on entity types.
#[proc_macro]
#[cfg(feature = "entity")]
pub fn entity_type_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::entity_type_properties(input.into()).into()
}

/// Derive `froglight_entity::entity_type::StaticEntityType` for a struct.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::StaticEntityType;
///
/// #[derive(StaticEntityType)]
/// struct MyItem;
///
/// // |
/// // V
///
/// impl froglight_entity::entity_type::StaticEntityType for MyItem {
///    fn as_static() -> &'static Self { &Self }
/// }
/// ```
#[cfg(feature = "entity")]
#[proc_macro_derive(StaticEntityType)]
pub fn derive_static_entity_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::derive_static_entity_type(input.into()).into()
}

/// Derive the appropriate traits on entity types.
#[proc_macro]
#[cfg(feature = "entity")]
pub fn status_effects(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::status_effects(input.into()).into()
}

/// Derive the appropriate traits on entity types.
#[proc_macro]
#[cfg(feature = "entity")]
pub fn status_effect_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::status_effect_properties(input.into()).into()
}

/// Derive `froglight_entity::status_effect::StaticStatusEffect` for a struct.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::StaticStatusEffect;
///
/// #[derive(StaticStatusEffect)]
/// struct MyItem;
///
/// // |
/// // V
///
/// impl froglight_entity::status_effect::StaticStatusEffect for MyItem {
///    fn as_static() -> &'static Self { &Self }
/// }
/// ```
#[cfg(feature = "entity")]
#[proc_macro_derive(StaticStatusEffect)]
pub fn derive_static_status_effect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    entity::derive_static_status_effect(input.into()).into()
}

// ------------------- `froglight-io` -------------------

#[cfg(feature = "io")]
mod io;

/// Derive `FrogRead`/`FrogVarRead` and `FrogWrite`/`FrogVarWrite` for an item.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::FrogBuf;
///
/// #[derive(FrogBuf)]
/// struct MyStruct {
///     a: u8,
///     b: u16,
/// }
///
/// #[derive(FrogBuf)]
/// enum MyEnum {
///     A(u8),
///     B(u16),
/// }
///
/// // |
/// // V
///
/// impl froglight_io::prelude::FrogRead for MyStruct {
///     fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, froglight_io::prelude::ReadError> {
///        Ok(Self {
///            a: froglight_io::prelude::FrogRead::frog_read(buffer)?,
///            b: froglight_io::prelude::FrogRead::frog_read(buffer)?,
///        })
///     }
/// }
/// impl froglight_io::prelude::FrogWrite for MyStruct {
///     fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, froglight_io::prelude::WriteError> {
///         let mut written = 0;
///         written += froglight_io::prelude::FrogWrite::frog_write(&self.a, buffer)?;
///         written += froglight_io::prelude::FrogWrite::frog_write(&self.b, buffer)?;
///         Ok(written)
///     }
///     fn frog_len(&self) -> usize {
///         froglight_io::prelude::FrogWrite::frog_len(&self.a) +
///         froglight_io::prelude::FrogWrite::frog_len(&self.b)
///     }
/// }
///
/// impl froglight_io::prelude::FrogRead for MyEnum {
///     fn frog_read(buffer: &mut impl std::io::Read) -> Result<Self, froglight_io::prelude::ReadError> {
///        match <u32 as froglight_io::prelude::FrogVarRead>::frog_var_read(buffer)? {
///            0 => Ok(Self::A(froglight_io::prelude::FrogRead::frog_read(buffer)?)),
///            1 => Ok(Self::B(froglight_io::prelude::FrogRead::frog_read(buffer)?)),
///            other => Err(froglight_io::prelude::ReadError::InvalidEnum(std::any::type_name::<Self>(), other)),
///        }
///     }
/// }
/// impl froglight_io::prelude::FrogWrite for MyEnum {
///     fn frog_write(&self, buffer: &mut impl std::io::Write) -> Result<usize, froglight_io::prelude::WriteError> {
///         let mut written = 0;
///         match self {
///             Self::A(a) => {
///                 written += froglight_io::prelude::FrogVarWrite::frog_var_write(&0u32, buffer)?;
///                 written += froglight_io::prelude::FrogWrite::frog_write(a, buffer)?;
///             },
///             Self::B(b) => {
///                 written += froglight_io::prelude::FrogVarWrite::frog_var_write(&1u32, buffer)?;
///                 written += froglight_io::prelude::FrogWrite::frog_write(b, buffer)?;
///             }
///         }
///         Ok(written)
///     }
///     fn frog_len(&self) -> usize {
///         match self {
///             Self::A(a) => {
///                 froglight_io::prelude::FrogVarWrite::frog_var_len(&0u32) +
///                 froglight_io::prelude::FrogWrite::frog_len(a)
///             }
///             Self::B(b) => {
///                 froglight_io::prelude::FrogVarWrite::frog_var_len(&1u32) +
///                 froglight_io::prelude::FrogWrite::frog_len(b)
///             },
///         }
///     }
/// }
/// ```
#[cfg(feature = "io")]
#[proc_macro_derive(FrogBuf, attributes(frog))]
pub fn derive_frogbuf(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    io::derive_frogbuf(input.into()).into()
}

/// Derive `FrogReadVersion`/`FrogWriteVersion` for a packet enum.
#[cfg(feature = "io")]
#[proc_macro_derive(FrogPackets, attributes(frog))]
pub fn derive_frogpackets(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    io::derive_frogpackets(input.into()).into()
}

// ------------------- `froglight-item` -------------------

#[cfg(feature = "item")]
mod item;

/// Derive `frogligt_item::item::StaticItem` for a struct.
///
/// # Example
/// ```rust,ignore
/// use froglight_macros::StaticItem;
///
/// #[derive(StaticItem)]
/// struct MyItem;
///
/// // |
/// // V
///
/// impl froglight_item::item::StaticItem for MyItem {
///    fn as_static() -> &'static Self { &Self }
/// }
/// ```
#[cfg(feature = "item")]
#[proc_macro_derive(StaticItem)]
pub fn derive_static_item(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item::derive_static_item(input.into()).into()
}

/// Derive the appropriate traits on items.
#[cfg(feature = "item")]
#[proc_macro]
pub fn items(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item::items(input.into()).into()
}

/// Derive the appropriate traits on item properties.
#[cfg(feature = "item")]
#[proc_macro]
pub fn item_properties(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item::item_properties(input.into()).into()
}

// --------------------- `froglight-nbt` --------------------

#[cfg(feature = "nbt")]
mod nbt;

/// Derive `ConvertNbt` for an item.
#[cfg(feature = "nbt")]
#[proc_macro_derive(FrogNbt, attributes(frog))]
pub fn derive_frognbt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    nbt::derive_frognbt(input.into()).into()
}

// ------------------- `froglight-network` -------------------

#[cfg(feature = "network")]
mod network;

// ------------------- `froglight-utils` -------------------

#[cfg(feature = "utils")]
mod utils;

/// Create an `AppStorage` struct.
#[cfg(feature = "utils")]
#[proc_macro_derive(AppStorage, attributes(storage))]
pub fn derive_appstorage(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    utils::derive_app_storage(input.into()).into()
}
