use proc_macro2::TokenStream;
use syn::Ident;

use super::parse::StatePackets;

pub(super) fn impl_enum_read(_ident: &Ident, _packets: &StatePackets, _output: &mut TokenStream) {}
