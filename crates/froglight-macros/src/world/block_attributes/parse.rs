use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Fields, FieldsUnnamed, Generics, Ident, Item, ItemEnum, ItemStruct, Token, Variant, Visibility,
};

#[derive(Debug, Clone)]
pub(crate) struct AttributeMacro {
    pub(crate) attributes: Vec<AttributeDeclaration>,
}

impl Parse for AttributeMacro {
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

#[derive(Debug, Clone)]
pub(crate) struct AttributeDeclaration {
    pub(crate) name: Ident,
    pub(crate) fields: AttributeFields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AttributeFields {
    Struct(FieldsUnnamed),
    Enum(Punctuated<Variant, Token![,]>),
}

impl Parse for AttributeDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the attribute name
        let ident = input.parse()?;

        if input.peek(syn::token::Paren) {
            // Parse unnamed struct fields
            Ok(Self { name: ident, fields: AttributeFields::Struct(input.parse()?) })
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

            Ok(Self { name: ident, fields: AttributeFields::Enum(variants) })
        } else {
            panic!("Invalid attribute declaration");
        }
    }
}

impl From<AttributeDeclaration> for Item {
    fn from(value: AttributeDeclaration) -> Self {
        match value.fields {
            AttributeFields::Struct(fields) => Item::Struct(ItemStruct {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect, froglight_macros::BlockAttribute)] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                struct_token: Token![struct](Span::call_site()),
                ident: Ident::new(&format!("{}Attribute", value.name), value.name.span()),
                generics: Generics::default(),
                fields: Fields::Unnamed(fields),
                semi_token: None,
            }),
            AttributeFields::Enum(variants) => Item::Enum(ItemEnum {
                attrs: vec![
                    syn::parse_quote! { #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect, froglight_macros::BlockAttribute)] },
                ],
                vis: Visibility::Public(Token![pub](Span::call_site())),
                enum_token: Token![enum](Span::call_site()),
                ident: Ident::new(&format!("{}Attribute", value.name), value.name.span()),
                generics: Generics::default(),
                brace_token: syn::token::Brace::default(),
                variants,
            }),
        }
    }
}
