use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Ident, Item, ItemEnum, ItemStruct};

mod parse;
use parse::BlockMacro;

use self::parse::BlockDeclaration;

pub(crate) fn frog_blocks(input: TokenStream) -> TokenStream {
    // Parse the input
    let BlockMacro { blocks } = syn::parse_macro_input!(input as BlockMacro);
    let structs: Vec<ItemStruct> = blocks.iter().map(|b| ItemStruct::from(b.clone())).collect();

    // Create a new token stream
    let mut tokens = TokenStream2::new();

    // Generate the block enum
    let enum_ident = create_block_enum(&blocks, &mut tokens);
    // Generate the block enum impl
    create_block_enum_impl(enum_ident, &blocks, &structs, &mut tokens);

    // Generate the block list
    for block in structs {
        tokens.extend(block.into_token_stream());
    }

    // Return the token stream
    TokenStream::from(tokens)
}

fn create_block_enum(blocks: &[BlockDeclaration], tokens: &mut TokenStream2) -> Ident {
    let ident = Ident::new("BlockEnum", Span::call_site());
    let mut variants = Punctuated::new();

    for block in blocks {
        let variant_ident = block.name.clone();
        let variant_type = Ident::new(&format!("Block{}", block.name), block.name.span());

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

fn create_block_enum_impl(
    ident: Ident,
    _blocks: &[BlockDeclaration],
    structs: &[ItemStruct],
    tokens: &mut TokenStream2,
) {
    let mut impl_tokens = TokenStream2::new();

    {
        let struct_idents: Vec<_> = structs.iter().map(|s| &s.ident).collect();
        impl_tokens.extend(quote! {
            /// Registers all block types for reflection.
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

/// Converts a `BlockDeclaration` into a `ItemStruct`
impl From<BlockDeclaration> for ItemStruct {
    fn from(value: BlockDeclaration) -> Self {
        ItemStruct {
            attrs: vec![
                syn::parse_quote! { #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
            ],
            vis: syn::Visibility::Public(syn::Token![pub](Span::call_site())),
            struct_token: syn::Token![struct](Span::call_site()),
            ident: Ident::new(&format!("Block{}", value.name), value.name.span()),
            generics: syn::Generics::default(),
            semi_token: if value.fields.is_empty() {
                Some(syn::Token![;](Span::call_site()))
            } else {
                None
            },
            fields: value.fields,
        }
    }
}
