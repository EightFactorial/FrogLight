use darling::{FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DataEnum, DataStruct, DeriveInput, Field, Fields, Ident, LitStr, Path, Type};

use crate::manifest::CrateManifest;

pub(crate) fn derive_frognbt(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogNbtMacro { path } = FrogNbtMacro::from_derive_input(&input).unwrap();
    let path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-nbt"));

    let DeriveInput { ident, data, .. } = input;

    let (from_tokens, as_tokens) = match data {
        syn::Data::Struct(data) => generate_struct(&path, data),
        syn::Data::Enum(data) => generate_enum(&path, data),
        syn::Data::Union(..) => panic!("`FrogNbt` does not support unions!"),
    };

    quote! {
        #[automatically_derived]
        impl #path::convert::ConvertNbt for #ident {
            fn from_compound(nbt: &#path::nbt::NbtCompound) -> Result<Self, #path::convert::ConvertError> {
                Ok(#from_tokens)
            }

            fn as_compound(&self) -> Result<#path::nbt::NbtCompound, #path::convert::ConvertError> {
                #as_tokens
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogNbtMacro {
    #[darling(default)]
    path: Option<Path>,
}

#[derive(Debug, Clone, PartialEq, Eq, FromField)]
#[darling(attributes(frog))]
struct FrogNbtField {
    #[darling(rename = "ident")]
    name: LitStr,
    tag: LitStr,
    with: Option<Ident>,
}

// -------------------------------------------------------------------------------------------------

fn generate_struct(path: &Path, data: DataStruct) -> (TokenStream, TokenStream) {
    let mut from_tokens = TokenStream::new();
    let as_tokens = quote! { todo!() };

    for (FrogNbtField { name, tag, with }, Field { ident, ty, .. }) in
        data.fields.iter().map(|f| (FrogNbtField::from_field(f).unwrap(), f))
    {
        // Use `#[frog(ident = "name")]`.
        let identifier = name.value();
        // Use `#[frog(tag = "type")]`.
        let (tag_type, tag_code) = TagType::tag_type(&tag, path, ty);

        // `from_tokens`
        {
            // Use `#[frog(with = `function`)]` or `TryFrom::try_from`.
            let value = if let Some(with) = with {
                quote!(#with::from_data(#tag_code)?)
            } else {
                quote! {
                    <#ty>::try_from(#tag_code).map_err(|err| {
                        #path::convert::ConvertError::ConversionError(std::any::type_name::<#ty>(), Box::new(err))
                    })?
                }
            };

            // Match the `NbtTag` to the expected type.
            let tokens = quote! {
                match nbt.get_tag(#identifier) {
                    Some(#path::nbt::NbtTag::#tag_type(value)) => #value,
                    None => Err(#path::convert::ConvertError::MissingField(#identifier))?,
                    _ => Err(#path::convert::ConvertError::MismatchedTag(#tag))?,
                },
            };

            match ident {
                Some(ident) => from_tokens.extend(quote!(#ident: #tokens)),
                None => from_tokens.extend(tokens),
            }
        }

        // `as_tokens`
        {}
    }

    // Wrap `from_tokens` in the appropriate struct constructor.
    let from_tokens = match data.fields {
        Fields::Named(_) => quote! { Self{#from_tokens} },
        Fields::Unnamed(_) => quote! { Self(#from_tokens) },
        Fields::Unit => panic!("`FrogNbt` does not support unit structs!"),
    };

    (from_tokens, as_tokens)
}

// -------------------------------------------------------------------------------------------------

fn generate_enum(_path: &Path, _data: DataEnum) -> (TokenStream, TokenStream) {
    let from_tokens = TokenStream::new();
    let as_tokens = TokenStream::new();

    (from_tokens, as_tokens)
}

// -------------------------------------------------------------------------------------------------

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

impl TagType {
    fn tag_type(lit: &LitStr, path: &Path, ty: &Type) -> (Ident, TokenStream) {
        let tag = Self::from(lit.value().as_str());

        let ident = match tag {
            TagType::Byte => Ident::new("Byte", lit.span()),
            TagType::Short => Ident::new("Short", lit.span()),
            TagType::Int => Ident::new("Int", lit.span()),
            TagType::Long => Ident::new("Long", lit.span()),
            TagType::Float => Ident::new("Float", lit.span()),
            TagType::Double => Ident::new("Double", lit.span()),
            TagType::ByteArray => Ident::new("ByteArray", lit.span()),
            TagType::String => Ident::new("String", lit.span()),
            TagType::List => Ident::new("List", lit.span()),
            TagType::Compound => Ident::new("Compound", lit.span()),
            TagType::IntArray => Ident::new("IntArray", lit.span()),
            TagType::LongArray => Ident::new("LongArray", lit.span()),
        };

        (ident, tag.tag_code(path, ty))
    }

    fn tag_code(self, path: &Path, ty: &Type) -> TokenStream {
        match self {
            Self::Float | Self::Double => quote!(*value),
            Self::Byte | Self::Short | Self::Int | Self::Long => quote!(*value as #ty),
            Self::Compound => quote!(<#ty as #path::convert::ConvertNbt>::from_compound(&value)?),
            Self::String | Self::List => quote!(value.clone()),
            Self::ByteArray | Self::IntArray | Self::LongArray => {
                let ty = ty.to_token_stream().to_string().to_lowercase();

                if ty.contains("u8") {
                    quote!(value.iter().map(|&v| v as u8).collect::<Vec<u8>>())
                } else if ty.contains("u32") {
                    quote!(value.iter().map(|&v| v as u32).collect::<Vec<u32>>())
                } else if ty.contains("u64") {
                    quote!(value.iter().map(|&v| v as u64).collect::<Vec<u64>>())
                } else {
                    quote!(value.clone())
                }
            }
        }
    }
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
            "bytearray" | "vec<i8>" | "[i8]" | "vec<u8>" | "[u8]" => TagType::ByteArray,
            "intarray" | "vec<i32>" | "[i32]" | "vec<u32>" | "[u32]" => TagType::IntArray,
            "longarray" | "vec<i64>" | "[i64]" | "vec<u64>" | "[u64]" => TagType::LongArray,
            _ => panic!("Unknown tag type: \"{value}\""),
        }
    }
}
