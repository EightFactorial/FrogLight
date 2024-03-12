use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

#[derive(Debug, Clone)]
pub(crate) struct BiomeEnumMacro {
    pub(crate) version: Ident,
    pub(crate) biomes: Vec<Ident>,
}

impl Parse for BiomeEnumMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut biomes = Vec::new();

        // Parse the version
        let version = input.parse()?;

        // If there's a comma, parse the comma
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        while !input.is_empty() {
            // Parse a biome ident
            biomes.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { version, biomes })
    }
}
