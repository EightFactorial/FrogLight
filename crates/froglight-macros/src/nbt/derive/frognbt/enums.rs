use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, Fields, FieldsNamed, FieldsUnnamed, Path, Variant};

/// Generate `FromCompound` and `IntoCompound` implementations for an enum.
pub(super) fn generate_enum(data: DataEnum, path: &Path) -> (TokenStream, TokenStream) {
    let (mut from, mut into) = (TokenStream::new(), TokenStream::new());

    for Variant { fields, .. } in data.variants {
        let (f_from, f_into) = match fields {
            Fields::Named(fields) => named_fields(fields, path),
            Fields::Unnamed(fields) => unnamed_fields(fields, path),
            Fields::Unit => panic!("`FrogNbt` does not support unit variants!"),
        };

        from.extend(f_from);
        into.extend(f_into);
    }

    (from, into)
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
fn named_fields(_fields: FieldsNamed, path: &Path) -> (TokenStream, TokenStream) {
    let mut from = quote!();
    let mut into = quote!();

    from.extend(quote!(todo!()));
    into.extend(quote!(todo!()));

    (from, into)
}

// -------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
fn unnamed_fields(fields: FieldsUnnamed, path: &Path) -> (TokenStream, TokenStream) {
    match fields.unnamed.len() {
        1 => todo!(),
        _ => panic!("`FrogNbt` only supports variants with exactly 1 unnamed field!"),
    }
}
