use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

pub(crate) fn entity_attributes(input: TokenStream) -> TokenStream {
    let MacroInput { path, entities } = syn::parse2(input).unwrap();
    let path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-entity"));

    let mut struct_tokens = quote! {
        #[cfg(feature = "bevy")]
        use bevy_ecs::prelude::*;
        #[cfg(feature = "reflect")]
        use bevy_reflect::prelude::*;
    };

    let mut enum_tokens = TokenStream::new();
    let mut fn_tokens = TokenStream::new();
    let mut impl_tokens = TokenStream::new();

    for entity in entities {
        let ident = &entity.ident;
        enum_tokens.extend(quote!(#ident(#ident),));
        fn_tokens.extend(quote! {
            Self::#ident(item) => { entity.insert(item); },
        });
        impl_tokens.extend(quote! {
            impl From<#ident> for EntityAttribute {
                #[inline]
                fn from(item: #ident) -> Self { Self::#ident(item) }
            }
        });

        struct_tokens.extend(MacroInput::as_tokens(&entity, &path));
    }

    quote! {
        #struct_tokens

        #[derive(Debug, Clone, Copy, PartialEq)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Clone, PartialEq))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(all(feature = "serde", feature = "reflect"), reflect(Serialize, Deserialize))]
        pub enum EntityAttribute {
            #enum_tokens
        }

        #[cfg(feature = "bevy")]
        impl EntityAttribute {
            pub fn apply_to(self, entity: &mut bevy_ecs::world::EntityWorldMut) {
                match self {
                    #fn_tokens
                }
            }
        }
    }
}

struct MacroInput {
    path: Option<Path>,
    entities: Vec<ItemStruct>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse::<Path>().map_or(None, |v| {
            let _ = input.parse::<Token![,]>();
            Some(v)
        });

        let mut entities = Vec::new();
        while !input.is_empty() {
            entities.push(input.parse::<ItemStruct>()?);
        }

        Ok(Self { path, entities })
    }
}
impl MacroInput {
    fn as_tokens(
        ItemStruct { vis, struct_token, ident, semi_token, .. }: &ItemStruct,
        path: &Path,
    ) -> TokenStream {
        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, #path::prelude::StaticEntityAttribute)]
            #[cfg_attr(feature = "bevy", derive(Component))]
            #[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
            #[cfg_attr(all(feature = "serde", feature = "reflect"), reflect(Serialize, Deserialize))]
            #vis #struct_token #ident #semi_token
        }
    }
}
