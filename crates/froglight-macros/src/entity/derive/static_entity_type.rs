use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::CrateManifest;

pub(crate) fn derive_static_entity_type(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = syn::parse2(input).unwrap();
    let block = CrateManifest::froglight("froglight-entity");

    quote! {
        #[automatically_derived]
        impl #block::entity_type::StaticEntityType for #ident {
            #[inline] #[must_use] fn as_static() -> &'static Self { &Self }
        }
    }
}
