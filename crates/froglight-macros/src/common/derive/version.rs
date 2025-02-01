use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitInt};

use crate::CrateManifest;

#[derive(FromDeriveInput)]
#[darling(attributes(version))]
struct VersionMacro {
    protocol: LitInt,
    resource: LitInt,
    #[darling(default)]
    feature: Option<String>,
}

pub(crate) fn derive_version(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let VersionMacro { protocol, resource, feature } =
        VersionMacro::from_derive_input(&input).unwrap();
    let DeriveInput { ident, .. } = input;

    let common = CrateManifest::froglight("froglight-common");
    let feature_tokens = feature.map_or(TokenStream::new(), |feature| {
        quote! {
            #[cfg(feature = #feature)]
        }
    });

    quote! {
        #feature_tokens
        impl #common::version::Version for #ident {
            const PROTOCOL_ID: u32 = #protocol;
            const RESOURCE_VERSION: u32 = #resource;
        }
    }
}
