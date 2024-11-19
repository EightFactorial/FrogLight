use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ItemStruct,
};

pub(super) fn impl_generated_blocks(tokens: TokenStream) -> TokenStream {
    let input = syn::parse2::<MacroInput>(tokens).unwrap();

    let mut output = TokenStream::new();

    // Add derives to each struct
    for item in &input.0 {
        output.extend(impl_struct(item));
    }
    // Create an enum containing all the structs
    output.extend(build_enum(&input.0));

    output
}

fn impl_struct(item: &ItemStruct) -> TokenStream {
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        #item
    }
}
fn build_enum(items: &[ItemStruct]) -> TokenStream {
    let mut output = TokenStream::new();
    for item in items {
        let ident = &item.ident;
        output.extend(quote! {
            #ident(#ident),
        });
    }
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        pub enum Blocks {
            #output
        }
    }
}

struct MacroInput(Vec<ItemStruct>);
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(MacroInput(items))
    }
}
