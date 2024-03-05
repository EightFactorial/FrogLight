use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident,
};

/// A state macro.
///
/// Expected syntax:
/// ```text
/// frog_state! {
///    Handshaking,
///    V1_20_0,
///    Clientbound {
///       0x00 => SomeClientboundPacket,
///       0x01 => SomeOtherClientboundPacket,
///    },
///    Serverbound {
///      0x00 => SomeServerboundPacket,
///      0x01 => SomeOtherServerboundPacket,
///    },
/// }
#[derive(Debug)]
pub(super) struct StateMacro {
    pub(super) state: Ident,
    pub(super) version: Ident,
    pub(super) clientbound: StatePackets,
    pub(super) serverbound: StatePackets,
}

impl Parse for StateMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the state name.
        let state = input.parse::<Ident>()?;

        // Optionally parse a comma.
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
        }

        // Parse the version.
        let version = input.parse::<Ident>()?;

        // Optionally parse a comma.
        if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
        }

        let mut clientbound = None;
        let mut serverbound = None;

        // Parse the direction and packets.
        for _ in 0..2 {
            let bound = input.parse::<Direction>()?;
            let packets = input.parse::<StatePackets>()?;

            // Optionally parse a comma.
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }

            // Store the packets in the correct direction.
            match bound {
                Direction::Clientbound => clientbound = Some(packets),
                Direction::Serverbound => serverbound = Some(packets),
            }

            // If there are no more directions, break.
            if input.is_empty() {
                break;
            }
        }

        Ok(Self {
            state,
            version,
            clientbound: clientbound.unwrap_or_default(),
            serverbound: serverbound.unwrap_or_default(),
        })
    }
}

#[derive(Debug)]
pub(super) enum Direction {
    Clientbound,
    Serverbound,
}

impl Parse for Direction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;

        match ident.to_string().as_str() {
            "Clientbound" => Ok(Self::Clientbound),
            "Serverbound" => Ok(Self::Serverbound),
            _ => Err(syn::Error::new(ident.span(), "expected `Clientbound` or `Serverbound`")),
        }
    }
}

/// A list of packets in a state.
///
/// Expected syntax:
/// ```text
/// Clientbound {
///    0x00 => MyPacket,
///    0x01 => MyOtherPacket,
/// }
/// ```
#[derive(Debug, Default)]
pub(super) struct StatePackets {
    pub(super) packets: Vec<Packet>,
}

impl Parse for StatePackets {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the opening brace.
        let content;
        syn::braced!(content in input);

        // If there are no packets, return an empty list.
        if content.is_empty() {
            Ok(Self { packets: Vec::new() })
        } else {
            // Parse the packets.
            let packets = content.call(Punctuated::<Packet, syn::Token![,]>::parse_terminated)?;

            Ok(Self { packets: packets.into_iter().collect() })
        }
    }
}

/// A packet in a state.
///
/// Expected syntax:
/// ```text
/// 0x00 => MyPacket,
/// ```
#[derive(Debug)]
pub(super) struct Packet {
    pub(super) _id: u32,
    pub(super) name: Ident,
}

impl Parse for Packet {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the packet ID.
        let int = input.parse::<syn::LitInt>()?;
        let id = int.base10_parse()?;

        // Parse the arrow.
        input.parse::<syn::Token![=>]>()?;

        // Parse the packet name.
        let name = input.parse::<Ident>()?;

        Ok(Self { _id: id, name })
    }
}
