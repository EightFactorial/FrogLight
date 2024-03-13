use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn frog_attribute_states(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = syn::parse_macro_input!(input as DeriveInput);

    // Get the number of states
    let states = count_states(&data);

    quote! {
        impl crate::blocks::traits::BlockAttribute for #ident {
            const STATES: u32 = #states;
        }
    }
    .into()
}

/// Count the number of states a block attribute has
fn count_states(data: &Data) -> u32 {
    match data {
        Data::Struct(data) => {
            // Allow only boolean fields
            assert!(
                data.fields.iter().all(|f| f.ty == syn::parse_quote!(bool)),
                "All fields must be booleans"
            );

            // Allow between 1 and 5 fields
            assert!(!data.fields.is_empty(), "Too few fields, the minimum is `1`");
            assert!(data.fields.len() <= 5, "Too many fields, the maximum is `5`");

            // The number of states is 2^(number of fields)
            u32::try_from(1 << data.fields.len()).unwrap()
        }
        Data::Enum(data) => {
            // Count the number of variants
            u32::try_from(data.variants.len()).unwrap()
        }
        Data::Union(_) => panic!("Unions are not supported"),
    }
}
