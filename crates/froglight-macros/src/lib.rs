#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "assets")]
mod assets;

#[cfg(feature = "protocol")]
mod protocol;

#[cfg(feature = "protocol")]
macro_rules! protocol_macro {
    ($name:ident, $fn:ident) => {
        #[allow(missing_docs)]
        #[cfg(feature = "protocol")]
        #[proc_macro_derive($name, attributes(frog))]
        pub fn $fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            protocol::$fn(input, true)
        }
    };
}

#[cfg(feature = "protocol")]
protocol_macro!(FrogRead, frog_read);
#[cfg(feature = "protocol")]
protocol_macro!(FrogWrite, frog_write);
#[cfg(feature = "protocol")]
protocol_macro!(FrogReadWrite, frog_read_write);

/// A macro for generating tests for `FrogRead` and `FrogWrite` implementations.
#[cfg(feature = "protocol")]
#[proc_macro_derive(FrogTest, attributes(frog))]
pub fn frog_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_test(input)
}

/// A macro for generating a version state implementation.
#[cfg(feature = "protocol")]
#[proc_macro]
pub fn frog_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_state(input)
}

#[cfg(feature = "world")]
mod world;

/// A macro for generating a block structs and a block enum.
#[cfg(feature = "world")]
#[proc_macro]
pub fn frog_blocks(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::block_list::frog_blocks(input)
}

/// A macro for generating a biome structs and a biome enum.
#[cfg(feature = "world")]
#[proc_macro]
pub fn frog_biomes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::biome_list::frog_biomes(input)
}

/// A macro for generating a version specific block enum.
#[cfg(feature = "world")]
#[proc_macro]
pub fn frog_version_blocks(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::block_enums::frog_version_blocks(input)
}

/// A macro for generating a version specific biome enum.
#[cfg(feature = "world")]
#[proc_macro]
pub fn frog_version_biomes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::biome_enums::frog_version_biomes(input)
}

/// A macro for generating block attributes.
#[cfg(feature = "world")]
#[proc_macro]
pub fn frog_block_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::block_attributes::frog_block_attributes(input)
}
