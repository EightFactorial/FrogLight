#![expect(dead_code)]

use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::CrateManifest;

pub(crate) fn derive_frogbuf(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogBufMacro { .. } = FrogBufMacro::from_derive_input(&input).unwrap();
    let DeriveInput { ident: _ident, generics: _generics, .. } = input;

    let _path = CrateManifest::try_find("froglight-network", "froglight")
        .unwrap_or_else(|| CrateManifest::froglight("froglight-io"));

    quote! {}
}

#[derive(FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogBufMacro {
    #[darling(default, flatten)]
    arguments: MacroType,
}

#[derive(PartialEq, Eq, FromMeta)]
enum MacroType {
    ReadWrite { read: bool, write: bool },
    Json { json: bool },
}
impl Default for MacroType {
    fn default() -> Self { Self::ReadWrite { read: true, write: true } }
}
