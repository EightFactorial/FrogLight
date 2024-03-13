#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// --- Asset Macros ---

#[cfg(feature = "assets")]
mod assets;

// --- Protocol Macros ---

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

// --- World Macros ---

#[cfg(feature = "world")]
mod world;

#[cfg(feature = "world")]
macro_rules! world_macro {
    ($name:ident, $fn:ident) => {
        #[allow(missing_docs)]
        #[cfg(feature = "world")]
        #[proc_macro]
        pub fn $fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream { world::$fn(input) }
    };
}

#[cfg(feature = "world")]
world_macro!(frog_blocks, frog_blocks);
#[cfg(feature = "world")]
world_macro!(frog_biomes, frog_biomes);
#[cfg(feature = "world")]
world_macro!(frog_version_blocks, frog_version_blocks);
#[cfg(feature = "world")]
world_macro!(frog_version_biomes, frog_version_biomes);
#[cfg(feature = "world")]
world_macro!(frog_block_attributes, frog_block_attributes);

/// A macro for counting the number of states a block attribute has
#[cfg(feature = "world")]
#[proc_macro_derive(BlockAttribute)]
pub fn frog_attribute_states(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    world::frog_attribute_states(input)
}
