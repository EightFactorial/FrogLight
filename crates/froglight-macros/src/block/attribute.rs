use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    ItemEnum, ItemStruct,
};

// TODO: Get the actual path to the `BlockAttribute` trait.
pub(super) fn impl_generated_attributes(tokens: TokenStream) -> TokenStream {
    let input = syn::parse2::<MacroInput>(tokens).unwrap();

    let mut output = TokenStream::new();

    // Add derives to each struct or enum
    for item in &input.0 {
        match item {
            EnumOrStruct::Enum(item) => {
                output.extend(impl_enum(item));
            }
            EnumOrStruct::Struct(item) => {
                output.extend(impl_struct(item));
            }
        }
    }
    // Build a register function to register all the block attributes
    output.extend(build_register(&input.0));

    output
}

fn impl_enum(item: &ItemEnum) -> TokenStream {
    let ident = &item.ident;
    let variants = item.variants.iter().map(|variant| &variant.ident);
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        #item
        impl From<#ident> for usize {
            fn from(value: #ident) -> usize {
                usize::from(value as u8)
            }
        }
        impl crate::BlockAttribute for #ident {
            const STATES: &'static [Self] = &[#(Self::#variants),*];
        }
    }
}
fn impl_struct(item: &ItemStruct) -> TokenStream {
    let ident = &item.ident;
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::Deref, derive_more::DerefMut)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        #item
        impl From<#ident> for usize {
            fn from(value: #ident) -> usize {
                usize::from(!value.0)
            }
        }
        impl crate::BlockAttribute for #ident {
            const STATES: &'static [Self] = &[Self(true), Self(false)];
        }
    }
}

fn build_register(items: &[EnumOrStruct]) -> TokenStream {
    let mut output = TokenStream::new();
    for item in items {
        let ident = match item {
            EnumOrStruct::Enum(item) => &item.ident,
            EnumOrStruct::Struct(item) => &item.ident,
        };
        output.extend(quote! {
            app.register_type::<#ident>();
        });
    }
    quote! {
        #[cfg(feature = "reflect")]
        pub(crate) fn register(app: &mut bevy_app::App) {
            #output
        }
    }
}

struct MacroInput(Vec<EnumOrStruct>);
impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(MacroInput(items))
    }
}

enum EnumOrStruct {
    Enum(ItemEnum),
    Struct(ItemStruct),
}
impl Parse for EnumOrStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::token::Enum) {
            Ok(EnumOrStruct::Enum(input.parse()?))
        } else {
            Ok(EnumOrStruct::Struct(input.parse()?))
        }
    }
}
