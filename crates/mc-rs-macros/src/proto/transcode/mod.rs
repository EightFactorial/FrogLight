use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::DeriveMacroAttr;

use super::{decode::DecodeMacro, encode::EncodeMacro, macro_type::MacroTypeTrait};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TranscodeMacro;

impl MacroTypeTrait for TranscodeMacro {
    fn generate_macro(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        let mut derives = TokenStream::new();

        derives.extend(EncodeMacro.generate_macro(attr, input));
        derives.extend(DecodeMacro.generate_macro(attr, input));

        derives
    }
}
