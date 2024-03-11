use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

#[derive(Debug, Clone)]
pub(crate) struct BlockEnumMacro {
    pub(crate) version: Ident,
    pub(crate) blocks: Vec<Ident>,
}

impl Parse for BlockEnumMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::new();

        // Parse the version ident
        let version: Ident = input.parse()?;

        // If there's a comma, parse the comma
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        while !input.is_empty() {
            // Parse a block ident
            blocks.push(input.parse()?);

            // If there's a comma, parse the comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { version, blocks })
    }
}
