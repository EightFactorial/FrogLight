use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Ident;

mod parse;
use parse::BlockEnumMacro;

/// Generate traits on a version-specific grouping of blocks
///
/// Note: Does not actually crate an enum
pub(crate) fn frog_version_blocks(input: TokenStream) -> TokenStream {
    // Parse the input
    let BlockEnumMacro { version, blocks } = syn::parse_macro_input!(input as BlockEnumMacro);

    // Collect tokens for various functions
    let mut register_tokens = TokenStream2::new();
    let mut reflect_tokens = TokenStream2::new();
    let mut resolve_tokens = TokenStream2::new();

    for block in blocks {
        let block_struct = Ident::new(&format!("Block{block}"), block.span());

        // Collect tokens for block registration
        register_tokens.extend(
            quote! {
                .register_block::<crate::blocks::block_list::#block_struct>()
            }
            .into_token_stream(),
        );

        // Collect tokens for reflect registration
        reflect_tokens.extend(
            quote! {
                registry.register_type_data::<crate::blocks::block_list::#block_struct, crate::blocks::reflect::ReflectBlockType<Self>>();
            }
            .into_token_stream(),
        );

        // Collect tokens for block state resolution
        resolve_tokens.extend(
            quote! {
                type_id if type_id == std::any::TypeId::of::<crate::blocks::block_list::#block_struct>() => {
                    let relative = registry.relative_state_of::<#block_struct>(state)?;
                    let state = crate::blocks::block_list::#block_struct::from_relative_state(relative)?;
                    Some(crate::blocks::block_list::BlockEnum::#block(state))
                }
            }
            .into_token_stream(),
        );
    }

    quote! {
        impl crate::blocks::traits::BlockRegistration for #version {
            fn register_default(registry: &mut crate::blocks::registry::InnerBlockRegistry<Self>) {
                registry
                #register_tokens
                ;
            }

            fn register_reflect(world: &mut bevy_ecs::world::World) {
                let Some(registry) = world.get_resource::<bevy_ecs::prelude::AppTypeRegistry>() else { return; };
                bevy_log::debug!("Registering ReflectBlockType for {:?} blocks.", #version);

                let mut registry = registry.write();
                #reflect_tokens
            }
        }

        impl crate::blocks::traits::BlockResolution for #version {
            fn get_block(state: u32, registry: &crate::blocks::registry::InnerBlockRegistry<Self>) -> Option<crate::blocks::block_list::BlockEnum> {
                let dyn_block = registry.get_dyn(state)?;
                match dyn_block.type_id() {
                    #resolve_tokens
                    _ => None,
                }
            }
        }
    }.into()
}
