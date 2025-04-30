use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

pub(crate) fn items(input: TokenStream) -> TokenStream {
    let MacroInput { path, items } = syn::parse2(input).unwrap();
    let item = path.unwrap_or_else(|| CrateManifest::froglight("froglight-item"));
    let block = CrateManifest::froglight("froglight-block");

    items.iter().fold(
        quote! {
            #[cfg(feature = "reflect")]
            use bevy_reflect::prelude::*;
        },
        |mut acc, input| {
            acc.extend(MacroInput::as_tokens(input, &item, &block));
            acc
        },
    )
}

struct MacroInput {
    path: Option<Path>,
    items: Vec<ItemStruct>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>().map_or(None, |v| {
            let _ = input.parse::<Token![,]>();
            Some(v)
        });

        let mut blocks = Vec::new();
        while !input.is_empty() {
            blocks.push(input.parse::<ItemStruct>()?);
        }

        Ok(Self { path, items: blocks })
    }
}
impl MacroInput {
    fn as_tokens(
        ItemStruct { attrs, vis, struct_token, ident, semi_token, .. }: &ItemStruct,
        path: &Path,
        block: &Path,
    ) -> TokenStream {
        if attrs.iter().any(|attr| attr.path().is_ident("block")) {
            quote! {
                #[cfg(not(feature = "block"))]
                #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, #path::prelude::StaticItem)]
                #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
                #vis #struct_token #ident #semi_token

                #[cfg(feature = "block")]
                pub use #block::generated::block::#ident;
                #[cfg(feature = "block")]
                #[automatically_derived]
                impl #path::prelude::StaticItem for #ident {
                    #[inline] #[must_use] fn as_static() -> &'static Self { &Self }
                }
            }
        } else {
            quote! {
                #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, #path::prelude::StaticItem)]
                #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
                #vis #struct_token #ident #semi_token
            }
        }
    }
}
