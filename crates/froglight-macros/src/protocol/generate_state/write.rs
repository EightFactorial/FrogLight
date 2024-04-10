use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::parse::StatePackets;

pub(super) fn impl_enum_write(ident: &Ident, packets: &StatePackets, output: &mut TokenStream) {
    let crate_path = crate::protocol::get_protocol_path();

    if packets.packets.is_empty() {
        output.extend(
            quote! {
                impl #crate_path::protocol::FrogWrite for #ident {
                    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), #crate_path::protocol::WriteError> {
                        unreachable!("Impossible to FrogWrite, no variants");
                    }
                }
            },
        );
        return;
    }

    let mut variant_tokens = TokenStream::new();
    for packet in &packets.packets {
        let id = &packet.id;
        let variant = &packet.variant;

        variant_tokens.extend(quote! {
            #ident::#variant(packet) => {
                <u32 as #crate_path::protocol::FrogVarWrite>::fg_var_write(&#id, buf)?;
                #crate_path::protocol::FrogWrite::fg_write(packet, buf)
            },
        });
    }

    output.extend(quote! {
        impl #crate_path::protocol::FrogWrite for #ident {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), #crate_path::protocol::WriteError> {
                match self {
                    #variant_tokens
                }
            }
        }
    });
}
