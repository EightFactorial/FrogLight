use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    ItemStruct, Path, Token,
    parse::{Parse, ParseStream},
};

use crate::CrateManifest;

pub(crate) fn entity_types(input: TokenStream) -> TokenStream {
    let MacroInput { path, entities } = syn::parse2(input).unwrap();
    let path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-entity"));

    let mut struct_tokens = quote! {
        #[cfg(feature = "bevy")]
        use bevy_ecs::prelude::*;
        #[cfg(feature = "reflect")]
        use bevy_reflect::prelude::*;
    };

    let mut enum_tokens = TokenStream::new();
    let mut hook_tokens = TokenStream::new();
    let mut impl_tokens = TokenStream::new();

    for entity in entities {
        let ident = &entity.ident;
        enum_tokens.extend(quote!(#ident,));
        hook_tokens.extend(quote! {
            Self::#ident => { if !entity_ref.contains::<#ident>() { commands.insert(#ident); } },
        });
        impl_tokens.extend(quote! {
            impl From<#ident> for EntityType {
                #[inline]
                fn from(_: #ident) -> Self { Self::#ident }
            }
        });

        struct_tokens.extend(MacroInput::as_tokens(&entity, &path));
    }

    quote! {
        #struct_tokens

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component), component(on_add = Self::on_add))]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Clone, PartialEq, Hash))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
        #[cfg_attr(all(feature = "serde", feature = "reflect"), reflect(Serialize, Deserialize))]
        pub enum EntityType {
            #enum_tokens
        }

        #[cfg(feature = "bevy")]
        impl EntityType {
            fn on_add(mut world: bevy_ecs::world::DeferredWorld, ctx: bevy_ecs::component::HookContext) {
                let (entities, mut commands) = world.entities_and_commands();
                let mut commands = commands.entity(ctx.entity);

                if let Ok(entity_ref) = entities.get(ctx.entity)
                    && let Some(&entity_type) = entity_ref.get::<Self>() {
                        match entity_type {
                            #hook_tokens
                        }
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
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, #path::prelude::StaticEntityType)]
            #[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component), require(#path::prelude::EntityType::#ident))]
            #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect), reflect(Debug, Default, Clone, PartialEq, Hash))]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Component))]
            #[cfg_attr(all(feature = "serde", feature = "reflect"), reflect(Serialize, Deserialize))]
            #vis #struct_token #ident #semi_token
        }
    }
}
