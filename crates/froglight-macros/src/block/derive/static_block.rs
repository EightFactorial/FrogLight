use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::CrateManifest;

pub(crate) fn derive_static_block(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse2(input).unwrap();
    let block = CrateManifest::froglight("froglight-block");

    quote! {
        impl #block::block::StaticBlock for #ident {
            #[inline]
            #[must_use]
            fn as_static() -> &'static Self { &Self }
        }
    }
}
