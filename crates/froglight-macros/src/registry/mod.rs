use proc_macro::TokenStream;
use syn::Path;

use crate::manifest::ProjectManifest;

mod generate_convertid;
mod generate_convertkey;

/// Get the path to the `froglight_registry` crate.
pub(crate) fn get_registry_path() -> Path { ProjectManifest::get().get_path("froglight_registry") }

/// Generate a `ConvertKey` implementation for a registry.
pub(super) fn frog_registry_convertkey(tokens: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    generate_convertkey::generate_convertkey(input).into()
}

/// Generate a `ConvertId` implementation for a registry.
pub(super) fn frog_create_registry_impls(tokens: TokenStream) -> TokenStream {
    generate_convertid::generate_convertid(tokens).into()
}
