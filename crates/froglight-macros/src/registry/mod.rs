use proc_macro::TokenStream;
use syn::Path;

use crate::manifest::ProjectManifest;

mod create_attributes;
mod create_blocks;
mod create_registry;
mod generate_convertkey;

/// Get the path to the `froglight_registry` crate.
pub(crate) fn get_registry_path() -> Path { ProjectManifest::get().get_path("froglight_registry") }

/// Generate a `ConvertKey` implementation for a registry.
pub(super) fn frog_registry_convertkey(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    generate_convertkey::generate_convertkey(input).into()
}

/// Generate the block attributes.
pub(super) fn frog_create_attributes(tokens: TokenStream) -> TokenStream {
    create_attributes::generate_attributes(tokens).into()
}

/// Generate the block structs.
pub(super) fn frog_create_blocks(tokens: TokenStream) -> TokenStream {
    create_blocks::generate_blocks(tokens).into()
}

/// Generate the block trait impls.
pub(super) fn frog_create_block_impls(tokens: TokenStream) -> TokenStream {
    create_blocks::generate_block_impls(tokens).into()
}

/// Generate the registry trait impls.
pub(super) fn frog_create_registry_impls(tokens: TokenStream) -> TokenStream {
    create_registry::generate_registry_impls(tokens).into()
}
