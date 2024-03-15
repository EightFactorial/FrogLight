use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

mod read;
mod write;

mod parse;
use parse::{Direction, StateMacro, StatePackets};

pub(super) fn generate_state(tokens: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    let StateMacro { state, version, clientbound, serverbound } = match syn::parse2(tokens) {
        Ok(state) => state,
        Err(err) => panic!("Failed to parse state macro: {err}"),
    };

    let clientbound_ident = direction_enum_name(Direction::Clientbound, &state);
    let serverbound_ident = direction_enum_name(Direction::Serverbound, &state);

    // Implement the state trait for the state.
    impl_state(&state, &version, &clientbound_ident, &serverbound_ident, &mut output);

    // Create the clientbound enum.
    create_packet_enum(&clientbound_ident, &clientbound, &mut output);
    impl_packet(&clientbound_ident, &mut output);

    // Create the serverbound enum.
    create_packet_enum(&serverbound_ident, &serverbound, &mut output);
    impl_packet(&serverbound_ident, &mut output);

    output
}

/// Implement the state trait for the state.
fn impl_state(
    state: &Ident,
    version: &Ident,
    client: &Ident,
    server: &Ident,
    output: &mut TokenStream,
) {
    output.extend(quote! {
        impl crate::traits::State<super::#version> for crate::states::#state {
            type ClientboundPacket = #client;
            type ServerboundPacket = #server;
        }
    });
}

/// Get the name of the packet enum.
fn direction_enum_name(direction: Direction, state: &Ident) -> Ident {
    match direction {
        Direction::Clientbound => Ident::new(&format!("{state}ClientboundPackets"), state.span()),
        Direction::Serverbound => Ident::new(&format!("{state}ServerboundPackets"), state.span()),
    }
}

/// Create the packet enum.
fn create_packet_enum(ident: &Ident, packets: &StatePackets, output: &mut TokenStream) {
    let mut variants = TokenStream::new();

    // Gather the enum variants and packet names.
    for packet in &packets.packets {
        let variant = &packet.variant;
        let name = &packet.name;

        variants.extend(quote! {
            #variant(#name),
        });
    }

    // Create the enum.
    output.extend(quote! {
        #[derive(Debug, Clone, PartialEq, derive_more::From)]
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        pub enum #ident {
            #variants
        }
    });

    // Implement `FrogRead` and `FrogWrite` for the enum.
    read::impl_enum_read(ident, packets, output);
    write::impl_enum_write(ident, packets, output);
}

/// Implement the packet trait for the packet.
fn impl_packet(ident: &Ident, output: &mut TokenStream) {
    output.extend(quote! {
        impl crate::traits::PacketEnum for #ident {}
    });
}
