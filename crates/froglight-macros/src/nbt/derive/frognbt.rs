use darling::{FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, Path};

use crate::manifest::CrateManifest;

pub(crate) fn derive_frognbt(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();
    let DeriveInput { ident, data, .. } = input;

    let path = CrateManifest::froglight("froglight-nbt");

    let from_tokens = TokenStream::new();
    let as_tokens = TokenStream::new();

    match data {
        syn::Data::Struct(_data) => {}
        syn::Data::Enum(_data) => {}
        syn::Data::Union(..) => panic!("`FrogNbt` does not support unions!"),
    }

    quote! {
        impl #path::convert::ConvertNbt for #ident {
            fn from_compound(nbt: &#path::nbt::NbtCompound) -> Result<Self, #path::convert::ConvertError> {
                #from_tokens
            }

            fn as_compound(&self) -> Result<#path::nbt::NbtCompound, #path::convert::ConvertError> {
                #as_tokens
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromField)]
#[darling(attributes(frog))]
struct FrogNbtField {
    tag: Option<LitStr>,
    with: Option<Path>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromVariant)]
#[darling(attributes(frog))]
struct FrogNbtVariant {
    tag: LitStr,
    with: Option<Path>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}
impl<'a> From<&'a str> for TagType {
    fn from(value: &'a str) -> Self {
        match value.to_lowercase().as_str() {
            "byte" | "i8" | "u8" => TagType::Byte,
            "short" | "i16" | "u16" => TagType::Short,
            "int" | "integer" | "i32" | "u32" => TagType::Int,
            "long" | "i64" | "u64" => TagType::Long,
            "float" | "f32" => TagType::Float,
            "double" | "f64" => TagType::Double,
            "string" | "str" => TagType::String,
            "list" | "vec" => TagType::List,
            "compound" | "object" => TagType::Compound,
            "bytearray" | "vec<i8>" | "vec<u8>" | "[i8]" | "[u8]" => TagType::ByteArray,
            "intarray" | "vec<i32>" | "vec<u32>" | "[i32]" | "[u32]" => TagType::IntArray,
            "longarray" | "vec<i64>" | "vec<u64>" | "[i64]" | "[u64]" => TagType::LongArray,
            _ => panic!("Unknown tag type: \"{value}\""),
        }
    }
}
