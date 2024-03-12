use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Ident, Item, ItemEnum};

mod parse;
use parse::BlockEnumMacro;

pub(crate) fn frog_version_blocks(input: TokenStream) -> TokenStream {
    // Parse the input
    let BlockEnumMacro { version, blocks } = syn::parse_macro_input!(input as BlockEnumMacro);

    // Create a new token stream
    let mut tokens = TokenStream2::new();

    // Generate the block enum
    let enum_ident = create_block_enum(&version, &blocks, &mut tokens);
    impl_blockenum(&enum_ident, &version, &blocks, &mut tokens);

    // Implement the BlockRegistration trait for the version
    impl_blockregistration(&enum_ident, &version, &blocks, &mut tokens);

    // Return the token stream
    TokenStream::from(tokens)
}

/// Creates a block enum from a version and a list of blocks.
fn create_block_enum(version: &Ident, blocks: &[Ident], tokens: &mut TokenStream2) -> Ident {
    let ident = Ident::new(&format!("BlockEnum{}", version.to_string()), Span::call_site());
    let mut variants = Punctuated::new();

    for block in blocks {
        let block_struct = Ident::new(&format!("Block{}", block), block.span());

        variants.push(syn::parse_quote! { #block(crate::blocks::block_list::#block_struct) });
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

/// Implements the `BlockRegistration` trait for the version.
fn impl_blockregistration(
    enum_ident: &Ident,
    version: &Ident,
    blocks: &[Ident],
    tokens: &mut TokenStream2,
) {
    let mut register_tokens = TokenStream2::new();

    for block in blocks {
        let block_struct = Ident::new(&format!("Block{}", block), block.span());

        register_tokens.extend(
            quote! {
                .register_block::<crate::blocks::block_list::#block_struct>()
            }
            .into_token_stream(),
        );
    }

    tokens.extend(
        quote! {
            impl From<#enum_ident> for crate::blocks::block_list::BlockEnum {
                fn from(block_enum: #enum_ident) -> Self {
                    match block_enum {
                        #(
                            #enum_ident::#blocks(block) => Self::#blocks(block),
                        )*
                    }
                }
            }

            impl BlockRegistration for #version {
                type Blocks = #enum_ident;

                fn register_default(registry: &mut InnerRegistry<Self>) {
                    registry
                    #register_tokens
                    ;
                }
            }
        }
        .into_token_stream(),
    );
}

fn impl_blockenum(
    enum_ident: &Ident,
    version: &Ident,
    blocks: &[Ident],
    tokens: &mut TokenStream2,
) {
    let mut match_tokens = TokenStream2::new();
    for block in blocks {
        let block_struct = Ident::new(&format!("Block{}", block), block.span());

        match_tokens.extend(
            quote! {
                type_id if type_id == std::any::TypeId::of::<crate::blocks::block_list::#block_struct>() => {
                    let relative = registry.relative_state_of::<#block_struct>(state)?;
                    Some(Self::#block(crate::blocks::block_list::#block_struct::from_relative_state(relative)?))
                }
            }
            .into_token_stream(),
        );
    }

    tokens.extend(
        quote! {
            impl crate::blocks::traits::BlockEnumTrait<#version> for #enum_ident {
                fn get_block(state: u32, registry: &InnerRegistry<#version>) -> Option<Self> {
                    let dyn_block = registry.get_dyn(state)?;
                    match dyn_block.type_id() {
                        #match_tokens
                        _ => None,
                    }
                }
            }
        }
        .into_token_stream(),
    );
}
