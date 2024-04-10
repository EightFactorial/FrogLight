use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::parse::StatePackets;

pub(super) fn impl_enum_read(ident: &Ident, packets: &StatePackets, output: &mut TokenStream) {
    let crate_path = crate::protocol::get_protocol_path();

    let ident_string = ident.to_string();

    if packets.packets.is_empty() {
        output.extend(quote! {
            impl #crate_path::protocol::FrogRead for #ident {
                fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, #crate_path::protocol::ReadError>
                where
                    Self: Sized,
                {
                    unreachable!("Impossible to FrogRead, no variants");
                }
            }
        });
        return;
    }

    let mut variant_tokens = TokenStream::new();
    for packet in &packets.packets {
        let id = &packet.id;
        let name = packet.name.to_string();
        let variant = &packet.variant;

        variant_tokens.extend(quote! {
            #id => Ok(#ident::#variant(#crate_path::protocol::FrogRead::fg_read(buf).map_err(|e| #crate_path::protocol::ReadError::PacketError(#id, #name, Box::new(e)))?)),
        });
    }

    output.extend(quote! {
        impl #crate_path::protocol::FrogRead for #ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, #crate_path::protocol::ReadError>
            where
                Self: Sized,
            {
                match <u32 as #crate_path::protocol::FrogVarRead>::fg_var_read(buf)? {
                    #variant_tokens
                    unk => Err(#crate_path::protocol::ReadError::InvalidEnum(i32::try_from(unk).expect("Packet ID Overflow"), #ident_string)),
                }
            }
        }
    });
}
