use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Fields, FieldsNamed, Generics, Ident, Item, ItemStruct, Token, Visibility,
};

pub(super) fn generate_blocks(tokens: proc_macro::TokenStream) -> TokenStream {
    let list: BlockList = syn::parse(tokens).expect("Failed to parse block list");

    let mut tokenstream = TokenStream::new();

    // Register the blocks
    let mut register_fns = TokenStream::new();
    for block in &list.blocks {
        let ident = &block.ident;
        register_fns.extend(quote! {
            app.register_type::<#ident>();
        });
    }

    // Add the `build` function
    tokenstream.extend(quote! {
        #[doc(hidden)]
        pub(super) fn build(app: &mut bevy_app::App) {
            #register_fns
        }
    });

    // Convert the blocks into structs
    list.blocks.into_iter().fold(tokenstream, |mut f, attr| {
        f.extend(Item::from(attr).into_token_stream());
        f
    })
}

struct BlockList {
    blocks: Vec<BlockDeclaration>,
}

impl Parse for BlockList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::new();

        while !input.is_empty() {
            // Parse a block declaration
            blocks.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { blocks })
    }
}

struct BlockDeclaration {
    ident: Ident,
    fields: Option<FieldsNamed>,
}

impl Parse for BlockDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the block name
        let ident = input.parse()?;

        if input.peek(syn::token::Comma) {
            // Struct with no fields
            Ok(Self { ident, fields: None })
        } else if input.peek(syn::token::Brace) {
            Ok(Self { ident, fields: Some(input.parse()?) })
        } else {
            panic!("Invalid attribute declaration");
        }
    }
}

impl From<BlockDeclaration> for Item {
    fn from(value: BlockDeclaration) -> Self {
        match value.fields {
            Some(fields) => Item::Struct(ItemStruct {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident: value.ident,
                generics: Generics::default(),
                fields: Fields::Named(fields),
                semi_token: None,
            }),
            None => Item::Struct(ItemStruct {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident: value.ident,
                generics: Generics::default(),
                fields: Fields::Unit,
                semi_token: Some(Token![;](Span::call_site())),
            }),
        }
    }
}
