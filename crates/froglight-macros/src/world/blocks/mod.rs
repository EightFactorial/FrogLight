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
    blocks: &[BlockDeclaration],
    structs: &[ItemStruct],
    tokens: &mut TokenStream2,
) {
    let mut impl_tokens = TokenStream2::new();

    // Generate the `register` function
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

    // Generate the `from_dyn` function
    {
        // Collect the `where` conditions
        let where_tokens = structs
            .iter()
            .map(|struct_| {
                let struct_ident = &struct_.ident;
                quote! { #struct_ident: crate::blocks::traits::BlockExt<V>, }
            })
            .collect::<TokenStream2>();

        // Collect the match tokens
        let mut match_tokens = TokenStream2::new();
        for (block, struct_) in blocks.iter().zip(structs) {
            let block_ident = &block.name;
            let struct_ident = &struct_.ident;

            match_tokens.extend(quote! {
                type_id if type_id == std::any::TypeId::of::<#struct_ident>() => {
                    crate::blocks::traits::BlockExt::<V>::from_relative_state(relative_state).map(Self::#block_ident)
                }
            });
        }

        impl_tokens.extend(quote! {
            /// Converts a dynamic block into a static block.
            ///
            /// # Warning
            /// This function only works for blocks inside the `BlockEnum`.
            pub(crate) fn from_dyn<V: froglight_protocol::traits::Version>(block: &dyn crate::blocks::traits::BlockType<V>, state_id: u32, registry: &crate::blocks::registry::InnerRegistry<V>) -> Option<Self>
            where
                #where_tokens
            {
                let relative_state = registry.relative_state(state_id)?;
                match block.type_id() {
                    #match_tokens
                    _ => None,
                }
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
