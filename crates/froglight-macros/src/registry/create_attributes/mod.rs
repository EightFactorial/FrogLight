use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Fields, FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemStruct, Token, Variant, Visibility,
};

pub(super) fn generate_attributes(tokens: proc_macro::TokenStream) -> TokenStream {
    let list: BlockAttributeList =
        syn::parse(tokens).expect("Failed to parse block attribute list");

    // Create the token stream
    let mut tokenstream = TokenStream::new();

    // Register the attributes
    let mut register_fns = TokenStream::new();
    for attr in &list.attributes {
        let ident = &attr.ident;
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

    // Convert the attributes into structs
    list.attributes.into_iter().fold(tokenstream, |mut f, attr| {
        f.extend(Item::from(attr).into_token_stream());
        f
    })
}

struct BlockAttributeList {
    attributes: Vec<AttributeDeclaration>,
}

impl Parse for BlockAttributeList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = Vec::new();

        while !input.is_empty() {
            // Parse a block declaration
            attributes.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { attributes })
    }
}

struct AttributeDeclaration {
    ident: Ident,
    fields: AttributeFields,
}

enum AttributeFields {
    Struct(FieldsUnnamed),
    Enum(Punctuated<Variant, Token![,]>),
}

impl Parse for AttributeDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the attribute name
        let ident = input.parse()?;

        if input.peek(syn::token::Paren) {
            // Parse unnamed struct fields
            Ok(Self { ident, fields: AttributeFields::Struct(input.parse()?) })
        } else if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);

            // Parse enum variants
            let mut variants = Punctuated::new();
            while !content.is_empty() {
                variants.push(content.parse()?);

                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(Self { ident, fields: AttributeFields::Enum(variants) })
        } else {
            panic!("Invalid attribute declaration");
        }
    }
}

impl From<AttributeDeclaration> for Item {
    fn from(value: AttributeDeclaration) -> Self {
        match value.fields {
            AttributeFields::Struct(fields) => Item::Struct(ItemStruct {
                attrs: if fields.unnamed.len() == 1 {
                    vec![
                        syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_derive::Deref, bevy_derive::DerefMut, bevy_reflect::Reflect)] },
                    ]
                } else {
                    vec![
                        syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
                    ]
                },
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident: value.ident,
                generics: Generics::default(),
                fields: Fields::Unnamed(fields),
                semi_token: None,
            }),
            AttributeFields::Enum(variants) => Item::Enum(ItemEnum {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                enum_token: Token![enum](Span::call_site()),
                ident: value.ident,
                generics: Generics::default(),
                brace_token: Brace::default(),
                variants,
            }),
        }
    }
}
