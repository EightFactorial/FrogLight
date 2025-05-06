use darling::{FromDeriveInput, FromField, util::Flag};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DataEnum, DataStruct, DeriveInput, Field, Fields, Ident, LitStr, Path, Type};

use crate::manifest::CrateManifest;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(frog))]
struct FrogNbtMacro {
    #[darling(default)]
    path: Option<Path>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(frog))]
struct FrogNbtField {
    #[darling(default, rename = "field")]
    field_name: Option<LitStr>,
    #[darling(default, rename = "default")]
    field_default: Flag,
    #[darling(rename = "tag")]
    tag_type: Ident,

    #[darling(default, rename = "skip_if")]
    _skip_if: Option<Path>,
    #[darling(default, rename = "with")]
    with_mod: Option<Ident>,
}

pub(crate) fn derive_frognbt(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let FrogNbtMacro { path } = FrogNbtMacro::from_derive_input(&input).unwrap();
    let path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-nbt"));

    let DeriveInput { ident, data, .. } = input;

    let (from_tokens, into_tokens) = match data {
        syn::Data::Struct(data) => generate_struct(&path, data),
        syn::Data::Enum(data) => generate_enum(&path, data),
        syn::Data::Union(..) => panic!("`FrogNbt` does not support unions!"),
    };

    quote! {
        #[automatically_derived]
        impl #path::convert::FromCompound for #ident {
            fn from_compound(nbt: &#path::nbt::NbtCompound) -> Result<Self, #path::convert::ConvertError> {
                Ok(#from_tokens)
            }
        }
        #[automatically_derived]
        impl #path::convert::IntoCompound for #ident {
            fn into_compound(&self) -> Result<#path::nbt::NbtCompound, #path::convert::ConvertError> {
                #into_tokens
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

fn generate_struct(path: &Path, data: DataStruct) -> (TokenStream, TokenStream) {
    let mut from_tokens = TokenStream::new();
    let into_tokens = quote! { todo!() };

    for (
        FrogNbtField { field_name, field_default, tag_type, _skip_if: _, with_mod },
        Field { ident, ty, .. },
    ) in data.fields.iter().map(|f| (FrogNbtField::from_field(f).unwrap(), f))
    {
        // Use `#[frog(field = "name")]`.
        let identifier = match (field_name, ident) {
            (Some(field), _) => field,
            (None, Some(ident)) => LitStr::new(&ident.to_string(), ident.span()),
            (None, None) => panic!("Fields without a name require a `field` attribute"),
        };

        // Use `#[frog(tag = "type")]`.
        let (tag_type, tag_code) = TagType::tag_type(&tag_type, path, ty);

        // `from_tokens`
        {
            // Use `#[frog(with = `function`)]` or `TryFrom::try_from`.
            let value = if let Some(with) = with_mod {
                quote! {{
                    let data = #tag_code;
                    #with::from_data(data)?
                }}
            } else {
                quote! {{
                    let data = #tag_code;
                    <#ty as TryFrom<_>>::try_from(data).map_err(|err| {
                        #path::convert::ConvertError::ConversionError(core::any::type_name::<#ty>(), Box::new(err))
                    })?
                }}
            };

            // Use `#[frog(default)]` or return an error when missing.
            let default = if field_default.is_present() {
                quote! {
                    None => Default::default()
                }
            } else {
                quote! {
                    None => Err(#path::convert::ConvertError::MissingField(core::any::type_name::<Self>(), #identifier))?
                }
            };

            // Match the `NbtTag` to the expected type.
            let tag_type_string = tag_type.to_string();
            let tokens = quote! {
                match nbt.get_tag(#identifier) {
                    Some(#path::nbt::NbtTag::#tag_type(value)) => #value,
                    #default,
                    _ => Err(#path::convert::ConvertError::MismatchedTag(core::any::type_name::<Self>(), #tag_type_string))?,
                },
            };

            match ident {
                Some(ident) => from_tokens.extend(quote!(#ident: #tokens)),
                None => from_tokens.extend(tokens),
            }
        }

        // `into_tokens`
        {}
    }

    // Wrap `from_tokens` in the appropriate struct constructor.
    let from_tokens = match data.fields {
        Fields::Named(_) => quote! { Self{#from_tokens} },
        Fields::Unnamed(_) => quote! { Self(#from_tokens) },
        Fields::Unit => panic!("`FrogNbt` does not support unit structs!"),
    };

    (from_tokens, into_tokens)
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
    Bool,
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
    fn tag_type(ident: &Ident, path: &Path, ty: &Type) -> (Ident, TokenStream) {
        let tag = Self::from(ident.to_string().as_str());

        let ident = match tag {
            TagType::Bool | TagType::Byte => Ident::new("Byte", ident.span()),
            TagType::Short => Ident::new("Short", ident.span()),
            TagType::Int => Ident::new("Int", ident.span()),
            TagType::Long => Ident::new("Long", ident.span()),
            TagType::Float => Ident::new("Float", ident.span()),
            TagType::Double => Ident::new("Double", ident.span()),
            TagType::ByteArray => Ident::new("ByteArray", ident.span()),
            TagType::String => Ident::new("String", ident.span()),
            TagType::List => Ident::new("List", ident.span()),
            TagType::Compound => Ident::new("Compound", ident.span()),
            TagType::IntArray => Ident::new("IntArray", ident.span()),
            TagType::LongArray => Ident::new("LongArray", ident.span()),
        };

        (ident, tag.tag_code(path, ty))
    }

    fn tag_code(self, path: &Path, ty: &Type) -> TokenStream {
        match self {
            Self::Bool => quote! {
                bool::try_from(*value).map_err(|err| #path::convert::ConvertError::ConversionError(core::any::type_name::<#ty>(), Box::new(err)))?
            },
            Self::Float | Self::Double => quote!(*value),
            Self::Byte | Self::Short | Self::Int | Self::Long => quote!(*value as #ty),
            Self::Compound => quote!(<#ty as #path::convert::FromCompound>::from_compound(&value)?),
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
            "bool" => TagType::Bool,
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
