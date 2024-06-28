use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use syn::Path;

use crate::manifest::ProjectManifest;

mod create_attributes;
mod create_blocks;
mod generate_convertkey;

/// Get the path to the `froglight_registry` crate.
pub(crate) fn get_registry_path() -> Path { ProjectManifest::get().get_path("froglight_registry") }

/// Generate a `ConvertKey` implementation for a registry.
pub(super) fn frog_registry_convertkey(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let attrs =
        ConvertKeyAttributes::from_attributes(&input.attrs).expect("Failed to parse attributes");

    generate_convertkey::generate_convertkey(input, attrs).into()
}

/// Attributes for the registry.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct ConvertKeyAttributes {
    #[attribute(optional)]
    error: Option<Path>,
}

/// Attributes for registry variants.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct ConvertKeyVariantAttributes {
    /// The key for the registry variant.
    #[attribute(optional, conflicts = [other])]
    key: String,

    /// If no other variants match, use this variant.
    #[attribute(conflicts = [key])]
    other: bool,
}

/// Generate the block attributes.
pub(super) fn frog_create_attributes(tokens: TokenStream) -> TokenStream {
    create_attributes::generate_attributes(tokens).into()
}

/// Generate the block structs.
pub(super) fn frog_create_blocks(tokens: TokenStream) -> TokenStream {
    create_blocks::generate_blocks(tokens).into()
}
