use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::CrateManifest;

pub(crate) fn derive_static_item(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse2(input).unwrap();
    let item = CrateManifest::froglight("froglight-item");

    quote! {
        #[automatically_derived]
        impl #item::item::StaticItem for #ident {
            #[inline] #[must_use] fn as_static() -> &'static Self { &Self }
        }
    }
}
