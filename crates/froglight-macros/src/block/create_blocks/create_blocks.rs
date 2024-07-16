use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Fields, FieldsNamed, Generics, Ident, Item, ItemStruct, Token, Visibility,
};

pub(crate) fn generate_blocks(tokens: proc_macro::TokenStream) -> TokenStream {
    let list: BlockList = syn::parse(tokens).expect("Failed to parse block list");
    let mut tokenstream = TokenStream::new();

    // Create the Blocks enum
    {
        let mut variants = TokenStream::new();
        for block in &list.blocks {
            let ident = &block.ident;
            variants.extend(quote! {
                #ident(#ident),
            });
        }

        tokenstream.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From)]
            #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
            pub enum Blocks {
                #variants
            }
        });
    }

    // Convert the blocks into structs
    list.blocks.into_iter().fold(tokenstream, |mut f, attr| {
        f.extend(BlockDeclaration::to_tokens(attr, &list.namespace));
        f
    })
}

struct BlockList {
    namespace: String,
    blocks: Vec<BlockDeclaration>,
}

impl Parse for BlockList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::new();

        // Parse the namespace
        let namespace = input.parse::<syn::LitStr>()?.value();
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        while !input.is_empty() {
            // Parse a block declaration
            blocks.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { namespace, blocks })
    }
}

struct BlockDeclaration {
    ident: Ident,
    key: String,
    fields: Option<FieldsNamed>,
}

impl Parse for BlockDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the block name
        let ident = input.parse()?;
        input.parse::<Token![=>]>()?;
        // Parse the block key
        let key = input.parse::<syn::LitStr>()?.value();

        if input.peek(syn::token::Comma) {
            // Struct with no fields
            Ok(Self { ident, key, fields: None })
        } else if input.peek(syn::token::Brace) {
            Ok(Self { ident, key, fields: Some(input.parse()?) })
        } else {
            panic!("Invalid attribute declaration");
        }
    }
}

impl BlockDeclaration {
    fn to_tokens(BlockDeclaration { ident, key, fields }: Self, namespace: &str) -> TokenStream {
        let protocol_path = crate::protocol::get_protocol_path();

        let (block_impl, type_impl): (Item, Item) = {
            let name_key = format!("{namespace}:{key}");
            let lang_key = format!("block.{namespace}.{key}");

            let mut method_fns = TokenStream::new();
            method_fns.extend(quote! {
                #[inline]
                fn to_key(&self) -> &'static #protocol_path::common::ResourceKey { Self::BLOCK_KEY }
                #[inline]
                fn to_lang(&self) -> &'static str { #lang_key }
            });

            if ident.to_string().ends_with("Air") {
                method_fns.extend(quote! {
                    #[inline]
                    fn is_air(&self) -> bool { true }
                });
            }

            (
                syn::parse_quote! {
                    impl #ident {
                        pub const BLOCK_KEY: &'static #protocol_path::common::ResourceKey = &#protocol_path::common::ResourceKey::const_new(#name_key);
                    }
                },
                syn::parse_quote! {
                    impl crate::BlockType for #ident {
                        #method_fns
                    }
                },
            )
        };

        let block_struct = match fields {
            Some(fields) => Item::Struct(ItemStruct {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] },
                    syn::parse_quote! { #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident,
                generics: Generics::default(),
                fields: Fields::Named(fields),
                semi_token: None,
            }),
            None => Item::Struct(ItemStruct {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)] },
                    syn::parse_quote! { #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident,
                generics: Generics::default(),
                fields: Fields::Unit,
                semi_token: Some(Token![;](Span::call_site())),
            }),
        };

        quote! {
            #block_struct
            #block_impl
            #type_impl
        }
    }
}
