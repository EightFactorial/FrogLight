use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Ident, Item, ItemEnum, ItemStruct};

mod parse;
use parse::{BiomeDeclaration, BiomeMacro};

pub(crate) fn frog_biomes(input: TokenStream) -> TokenStream {
    // Parse the input
    let BiomeMacro { biomes } = syn::parse_macro_input!(input as BiomeMacro);
    let structs: Vec<ItemStruct> = biomes.iter().map(|b| ItemStruct::from(b.clone())).collect();

    // Create a new token stream
    let mut tokens = TokenStream2::new();

    // Generate the biome enum
    let enum_ident = create_biome_enum(&biomes, &mut tokens);
    // Generate the biome enum impl
    create_biome_enum_impl(enum_ident, &biomes, &structs, &mut tokens);

    // Generate the biome list
    for biome in structs {
        tokens.extend(biome.into_token_stream());
    }

    // Return the token stream
    TokenStream::from(tokens)
}

fn create_biome_enum(biomes: &[BiomeDeclaration], tokens: &mut TokenStream2) -> Ident {
    let ident = Ident::new("BiomeEnum", Span::call_site());
    let mut variants = Punctuated::new();

    for biome in biomes {
        let variant_ident = biome.name.clone();
        let variant_type = Ident::new(&format!("Biome{}", biome.name), biome.name.span());

        variants.push(syn::parse_quote! { #variant_ident(#variant_type) });
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
        .to_token_stream(),
    );

    ident
}

fn create_biome_enum_impl(
    ident: Ident,
    _biomes: &[BiomeDeclaration],
    structs: &[ItemStruct],
    tokens: &mut TokenStream2,
) {
    let mut impl_tokens = TokenStream2::new();

    // Generate the `register` function
    {
        let struct_idents: Vec<_> = structs.iter().map(|s| &s.ident).collect();
        impl_tokens.extend(quote! {
            /// Registers all biome types for reflection.
            pub(crate) fn register(app: &mut bevy_app::App) {
                app.register_type::<Self>()
                #(
                    .register_type::<#struct_idents>()
                )*
                ;
            }
        });
    }

    tokens.extend(
        Item::Impl(syn::parse_quote! {
            impl #ident {
                #impl_tokens
            }
        })
        .into_token_stream(),
    );
}
