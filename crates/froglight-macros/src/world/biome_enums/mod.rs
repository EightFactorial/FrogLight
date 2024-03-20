use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::Ident;

mod parse;
use parse::BiomeEnumMacro;

pub(crate) fn frog_version_biomes(input: TokenStream) -> TokenStream {
    // Parse the input
    let BiomeEnumMacro { version, biomes } = syn::parse_macro_input!(input as BiomeEnumMacro);

    let mut register_tokens = TokenStream2::new();
    let mut reflect_tokens = TokenStream2::new();
    let mut resolve_tokens = TokenStream2::new();

    for biome in biomes {
        let biome_struct = Ident::new(&format!("Biome{biome}"), biome.span());

        register_tokens.extend(
            quote! {
                .register_biome::<crate::biomes::biome_list::#biome_struct>()
            }
            .into_token_stream(),
        );

        reflect_tokens.extend(
            quote! {
                registry.register_type_data::<crate::biomes::biome_list::#biome_struct, crate::biomes::reflect::ReflectBiomeType<Self>>();
            }
            .into_token_stream(),
        );

        resolve_tokens.extend(
            quote! {
                type_id if type_id == std::any::TypeId::of::<crate::biomes::biome_list::#biome_struct>() => {
                    Some(crate::biomes::biome_list::BiomeEnum::#biome(crate::biomes::biome_list::#biome_struct))
                }
            }
            .into_token_stream(),
        );
    }

    quote! {
        impl crate::biomes::traits::BiomeRegistration for #version {
            fn register_default(registry: &mut crate::biomes::registry::InnerBiomeRegistry<Self>) {
                registry
                #register_tokens
                ;
            }

            fn register_reflect(world: &mut bevy_ecs::world::World) {
                let Some(registry) = world.get_resource::<bevy_ecs::prelude::AppTypeRegistry>() else { return; };
                let mut registry = registry.write();

                bevy_log::debug!("Registering ReflectBiomeType<{:?}>", #version);

                #reflect_tokens
            }
        }

        impl crate::biomes::traits::BiomeResolution for #version {
            fn get_biome(biome: u32, registry: &crate::biomes::registry::InnerBiomeRegistry<Self>) -> Option<crate::biomes::biome_list::BiomeEnum> {
                let dyn_biome = registry.get_dyn(biome)?;
                match dyn_biome.type_id() {
                    #resolve_tokens
                    _ => None,
                }
            }
        }
    }.into()
}
