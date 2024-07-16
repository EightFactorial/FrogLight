use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Brace,
    Attribute, Fields, FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemStruct, Token, Variant,
    Visibility,
};

pub(super) fn generate_attributes(tokens: proc_macro::TokenStream) -> TokenStream {
    let list: BlockAttributeList =
        syn::parse(tokens).expect("Failed to parse block attribute list");

    // Convert the attributes into structs
    list.attributes.into_iter().fold(TokenStream::new(), |mut f, attr| {
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
        let reflect: Attribute =
            syn::parse_quote! { #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))] };

        match value.fields {
            AttributeFields::Struct(fields) => Item::Struct(ItemStruct {
                attrs: if fields.unnamed.len() == 1 {
                    vec![
                        syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::From, derive_more::Into, derive_more::Deref, derive_more::DerefMut)] },
                        reflect,
                    ]
                } else {
                    vec![
                        syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] },
                        reflect,
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
                    syn::parse_quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] },
                    reflect,
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
