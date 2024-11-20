use attribute_derive::FromAttr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

pub(super) fn impl_registry_consts(input: DeriveInput) -> TokenStream {
    // Get the enum ident and data
    let enum_ident = &input.ident;
    let Data::Enum(data) = &input.data else {
        panic!("Registries must be enums");
    };

    let mut variant_consts = TokenStream::new();
    for variant in &data.variants {
        let const_ident = Ident::new(
            &format!("KEY_{}", variant.ident.to_string().to_ascii_uppercase()),
            variant.ident.span(),
        );
        let VariantAttributes { key } = VariantAttributes::from_attributes(&variant.attrs).unwrap();

        variant_consts.extend(quote! {
            pub const #const_ident: &str = #key;
        });
    }

    quote! {
        impl #enum_ident {
            #variant_consts
        }
    }
}

/// Attributes for registry variants.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct VariantAttributes {
    /// The key for the registry variant.
    key: String,
}
