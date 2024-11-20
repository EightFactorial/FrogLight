use proc_macro::TokenStream;

mod impl_generated;
mod registry_consts;

// Get the path to the `froglight_registry` crate.
// pub(crate) fn get_registry_path() -> Path {
// ProjectManifest::get().get_path("froglight_registry") }

/// Generate a `ConvertKey` implementation for a registry.
pub(super) fn impl_registry_consts(tokens: TokenStream) -> TokenStream {
    registry_consts::impl_registry_consts(syn::parse_macro_input!(tokens as syn::DeriveInput))
        .into()
}

/// Generate a `RegistryId` and `ConvertId` implementation for
/// all registries used in a specific version.
pub(super) fn impl_generated_registries(tokens: TokenStream) -> TokenStream {
    impl_generated::impl_generated_registries(tokens).into()
}
