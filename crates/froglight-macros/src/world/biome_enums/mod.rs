use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Ident, Item, ItemEnum};

mod parse;
use parse::BiomeEnumMacro;

pub(crate) fn frog_version_biomes(input: TokenStream) -> TokenStream {
    // Parse the input
    let BiomeEnumMacro { version, biomes } = syn::parse_macro_input!(input as BiomeEnumMacro);

    // Create a new token stream
    let mut tokens = TokenStream2::new();

    // Generate the biome enum
    let enum_ident = create_biome_enum(&version, &biomes, &mut tokens);
    impl_biomeenum(&enum_ident, &version, &biomes, &mut tokens);

    // Implement the BiomeRegistration trait for the version
    impl_biomeregistration(&enum_ident, &version, &biomes, &mut tokens);

    // Return the token stream
    TokenStream::from(tokens)
}

/// Creates a biome enum from a version and a list of biomes.
fn create_biome_enum(version: &Ident, biomes: &[Ident], tokens: &mut TokenStream2) -> Ident {
    let ident = Ident::new(&format!("BiomeEnum{version}"), Span::call_site());
    let mut variants = Punctuated::new();

    for biome in biomes {
        let biome_struct = Ident::new(&format!("Biome{biome}"), biome.span());

        variants.push(syn::parse_quote! { #biome(crate::biomes::biome_list::#biome_struct) });
    }

    tokens.extend(
        Item::Enum(ItemEnum {
            attrs: vec![
                syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
            ],
            vis: syn::Visibility::Public(syn::Token![pub](Span::call_site())),
            enum_token: syn::Token![enum](Span::call_site()),
            ident: ident.clone(),
            generics: syn::Generics::default(),
            brace_token: syn::token::Brace::default(),
            variants,
        })
        .into_token_stream(),
    );

    ident
}

/// Implements the `BiomeRegistration` trait for the version.
fn impl_biomeregistration(
    enum_ident: &Ident,
    version: &Ident,
    biomes: &[Ident],
    tokens: &mut TokenStream2,
) {
    let mut register_tokens = TokenStream2::new();

    for biome in biomes {
        let biome_struct = Ident::new(&format!("Biome{biome}"), biome.span());

        register_tokens.extend(
            quote! {
                .register_biome::<crate::biomes::biome_list::#biome_struct>()
            }
            .into_token_stream(),
        );
    }

    tokens.extend(quote! {
        impl From<#enum_ident> for crate::biomes::biome_list::BiomeEnum {
            fn from(biome_enum: #enum_ident) -> Self {
                match biome_enum {
                    #(
                        #enum_ident::#biomes(biome) => Self::#biomes(biome),
                    )*
                }
            }
        }

        impl BiomeRegistration for #version {
            type Biomes = #enum_ident;
            fn register_default(registry: &mut crate::biomes::registry::InnerBiomeRegistry<Self>) {
                registry
                #register_tokens
                ;
            }
        }
    });
}

fn impl_biomeenum(
    enum_ident: &Ident,
    version: &Ident,
    biomes: &[Ident],
    tokens: &mut TokenStream2,
) {
    let mut match_tokens = TokenStream2::new();
    for biome in biomes {
        let biome_struct = Ident::new(&format!("Biome{biome}"), biome.span());

        match_tokens.extend(
            quote! {
                type_id if type_id == std::any::TypeId::of::<crate::biomes::biome_list::#biome_struct>() => {
                    Some(Self::#biome(crate::biomes::biome_list::#biome_struct))
                }
            }
            .into_token_stream(),
        );
    }

    tokens.extend(
        quote! {
            impl crate::biomes::traits::BiomeEnumTrait<#version> for #enum_ident {
                fn get_biome(biome: u32, registry: &crate::biomes::registry::InnerBiomeRegistry<#version>) -> Option<Self> {
                    let dyn_biome = registry.get_dyn(biome)?;
                    match dyn_biome.type_id() {
                        #match_tokens
                        _ => None,
                    }
                }
            }
        }
        .into_token_stream(),
    );
}
