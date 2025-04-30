use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

pub(crate) fn blocks(input: TokenStream) -> TokenStream {
    let MacroInput { path, blocks } = syn::parse2(input).unwrap();
    let block = path.unwrap_or_else(|| CrateManifest::froglight("froglight-block"));

    blocks.iter().fold(
        quote! {
            #[cfg(feature = "reflect")]
            use bevy_reflect::prelude::*;
        },
        |mut acc, item| {
            acc.extend(MacroInput::as_tokens(item, &block));
            acc
        },
    )
}

struct MacroInput {
    path: Option<Path>,
    blocks: Vec<ItemStruct>,
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

        Ok(Self { path, blocks })
    }
}
impl MacroInput {
    fn as_tokens(
        ItemStruct { vis, struct_token, ident, semi_token, .. }: &ItemStruct,
        path: &Path,
    ) -> TokenStream {
        quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, #path::prelude::StaticBlock)]
            #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
            #vis #struct_token #ident #semi_token
        }
    }
}
