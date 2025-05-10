use darling::{FromDeriveInput, FromField, FromMeta, util::Flag};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Path};

use crate::manifest::CrateManifest;

mod enums;
mod structs;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(frog))]
struct MacroInvoke {
    /// An optional path to the `froglight-nbt` crate.
    #[darling(default)]
    path: Option<Path>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(frog))]
struct FieldAttrs {
    /// Use the default value if the field
    /// is missing when reading Nbt data.
    default: Flag,

    /// The name of the NbtTag.
    ///
    /// Defaults to the field name,
    /// required for fields without names.
    #[darling(default, rename = "name")]
    tag_name: Option<Ident>,

    /// Convert the type to/from a tag's inner
    /// type instead of a tag directly.
    #[darling(default, rename = "tag")]
    tag_type: FieldTagType,
    /// The type of list to expect when
    /// using a `FieldTagType::List`
    #[darling(default, rename = "list")]
    list_type: Option<FieldListType>,

    /// If given, use the following functions
    /// instead of the default to read/write Nbt.
    #[darling(default, rename = "with")]
    with_fn: Option<Path>,
    /// If the given method returns true,
    /// skip writing as an Nbt tag.
    #[darling(default, rename = "skip_if")]
    skip_fn: Option<Path>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromMeta)]
enum FieldTagType {
    #[default]
    None,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromMeta)]
enum FieldListType {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    Compound,
    IntArray,
    LongArray,
}

// -------------------------------------------------------------------------------------------------

pub(crate) fn derive_frognbt(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).unwrap();

    let MacroInvoke { path } = MacroInvoke::from_derive_input(&input).unwrap();
    let path = path.unwrap_or_else(|| CrateManifest::froglight("froglight-nbt"));

    let DeriveInput { ident, data, .. } = input;
    let (from_tokens, into_tokens) = match data {
        Data::Struct(data) => structs::generate_struct(data, &path),
        Data::Enum(data) => enums::generate_enum(data, &path),
        Data::Union(..) => panic!("`FrogNbt` does not support unions!"),
    };

    quote! {
        #[automatically_derived]
        impl #path::convert::FromCompound for #ident {
            fn from_compound(nbt: &#path::nbt::NbtCompound) -> Result<Self, #path::convert::ConvertError> {
                Ok(#from_tokens)
            }
        }
        #[automatically_derived]
        #[allow(dead_code, unreachable_code)]
        impl #path::convert::IntoCompound for #ident {
            fn into_compound(&self) -> Result<#path::nbt::NbtCompound, #path::convert::ConvertError> {
                let mut nbt = #path::nbt::NbtCompound::new();
                #into_tokens
                Ok(nbt)
            }
        }
    }
}
