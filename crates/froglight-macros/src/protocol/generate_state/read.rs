use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::parse::StatePackets;

pub(super) fn impl_enum_read(ident: &Ident, packets: &StatePackets, output: &mut TokenStream) {
    let ident_string = ident.to_string();

    if packets.packets.is_empty() {
        output.extend(quote! {
            impl ::froglight::protocol::FrogRead for #ident {
                fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ::froglight::protocol::ReadError>
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
            #id => Ok(#ident::#variant(::froglight::protocol::FrogRead::fg_read(buf).map_err(|e| ::froglight::protocol::ReadError::PacketError(#id, #name, Box::new(e)))?)),
        });
    }

    output.extend(quote! {
        impl ::froglight::protocol::FrogRead for #ident {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ::froglight::protocol::ReadError>
            where
                Self: Sized,
            {
                match <u32 as ::froglight::protocol::FrogVarRead>::fg_var_read(buf)? {
                    #variant_tokens
                    unk => Err(::froglight::protocol::ReadError::InvalidEnum(i32::try_from(unk).expect("Packet ID Overflow"), #ident_string)),
                }
            }
        }
    });
}
