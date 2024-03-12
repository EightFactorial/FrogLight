use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Ident, Item};

mod parse;
use parse::AttributeMacro;

pub(crate) fn frog_block_attributes(input: TokenStream) -> TokenStream {
    let AttributeMacro { attributes } = syn::parse_macro_input!(input as AttributeMacro);
    let attribute_items: Vec<Item> = attributes.iter().map(|a| Item::from(a.clone())).collect();

    // Create a new token stream
    let mut tokens = TokenStream2::new();

    // Create the register function
    create_register_fn(&attribute_items, &mut tokens);

    // Generate the block attribute structs/enums
    for attribute in attribute_items {
        tokens.extend(attribute.into_token_stream());
    }

    // Return the token stream
    TokenStream::from(tokens)
}

fn create_register_fn(items: &[Item], output: &mut TokenStream2) {
    let items: Vec<&Ident> = items
        .iter()
        .map(|item| match item {
            Item::Struct(s) => &s.ident,
            Item::Enum(e) => &e.ident,
            _ => unreachable!(),
        })
        .collect();

    output.extend(quote! {
        #[doc(hidden)]
        #[allow(clippy::too_many_lines)]
        pub(super) fn register(app: &mut App) {
            app
            #(.register_type::<#items>())*
            ;
        }
    });
}
