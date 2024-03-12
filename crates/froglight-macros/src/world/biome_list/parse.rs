use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Fields, Ident, ItemStruct, Token,
};

#[derive(Debug, Clone)]
pub(crate) struct BiomeMacro {
    pub(crate) biomes: Vec<BiomeDeclaration>,
}

impl Parse for BiomeMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut biomes = Vec::new();

        while !input.is_empty() {
            // Parse a biome declaration
            biomes.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { biomes })
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BiomeDeclaration {
    pub(crate) name: Ident,
}

impl Parse for BiomeDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> { Ok(Self { name: input.parse()? }) }
}

/// Converts a `BlockDeclaration` into a `ItemStruct`
impl From<BiomeDeclaration> for ItemStruct {
    fn from(value: BiomeDeclaration) -> Self {
        ItemStruct {
            attrs: vec![
                syn::parse_quote! { #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, bevy_reflect::Reflect)] },
            ],
            vis: syn::Visibility::Public(syn::Token![pub](Span::call_site())),
            struct_token: syn::Token![struct](Span::call_site()),
            ident: Ident::new(&format!("Biome{}", value.name), value.name.span()),
            generics: syn::Generics::default(),
            semi_token: Some(syn::Token![;](Span::call_site())),
            fields: Fields::Unit,
        }
    }
}
