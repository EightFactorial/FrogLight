use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

pub(crate) fn items(input: TokenStream) -> TokenStream {
    let MacroInput { path, items } = syn::parse2(input).unwrap();
    let block = path.unwrap_or_else(|| CrateManifest::froglight("froglight-item"));

    items.iter().fold(TokenStream::new(), |mut acc, item| {
        acc.extend(MacroInput::as_tokens(item, &block));
        acc
    })
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
        ItemStruct { vis, struct_token, ident, semi_token, .. }: &ItemStruct,
        path: &Path,
    ) -> TokenStream {
        quote! {
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, #path::prelude::StaticItem)]
            #[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect), reflect(Debug, PartialEq, Hash))]
            #vis #struct_token #ident #semi_token
        }
    }
}
