use syn::{
    parse::{Parse, ParseStream},
    Fields, Ident, Token,
};

#[derive(Debug, Clone)]
pub(crate) struct BlockMacro {
    pub(crate) blocks: Vec<BlockDeclaration>,
}

impl Parse for BlockMacro {
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

#[derive(Debug, Clone)]
pub(crate) struct BlockDeclaration {
    pub(crate) name: Ident,
    pub(crate) fields: Fields,
}

impl Parse for BlockDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the block name
        let ident = input.parse()?;

        // Return a unit struct if there's a comma
        if input.peek(syn::Token![,]) {
            return Ok(Self { name: ident, fields: Fields::Unit });
        }
        // Return a unit struct if there's a semicolon
        if input.peek(syn::Token![;]) {
            return Ok(Self { name: ident, fields: Fields::Unit });
        }

        Ok(Self { name: ident, fields: Fields::Named(input.parse()?) })
    }
}
