use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, Token,
};

/// Implements the `State` trait for a state,
/// creates the clientbound and serverbound packet enums,
/// and implements `Encode` and `Decode` for them.
pub fn impl_state(input: TokenStream) -> TokenStream {
    let StatePackets {
        state,
        version,
        clientbound,
        serverbound,
    } = parse_macro_input!(input as StatePackets);

    // Create the clientbound enum name
    let clientbound_name = Ident::new(
        &format!("Clientbound{}Packets", state),
        state.span().join(version.span()).unwrap(),
    );

    // Create the serverbound enum name
    let serverbound_name = Ident::new(
        &format!("Serverbound{}Packets", state),
        state.span().join(version.span()).unwrap(),
    );

    // Create the `State` implementation
    let state_impl = quote! {
        impl crate::State<#version> for #state {
            type Clientbound = #clientbound_name;
            type Serverbound = #serverbound_name;
        }
    };

    // Create the enum and derive `Encode` and `Decode` for each clientbound packet
    let clientbound_enum = create_packet_enum(&clientbound_name, &clientbound.0);
    let clientbound_encode = implement_encode(&clientbound_name, &clientbound.0);
    let clientbound_decode = implement_decode(&clientbound_name, &clientbound.0);

    // Create the enum and derive `Encode` and `Decode` for each serverbound packet
    let serverbound_enum = create_packet_enum(&serverbound_name, &serverbound.0);
    let serverbound_encode = implement_encode(&serverbound_name, &serverbound.0);
    let serverbound_decode = implement_decode(&serverbound_name, &serverbound.0);

    quote! {
        #state_impl

        #clientbound_enum
        #clientbound_encode
        #clientbound_decode

        #serverbound_enum
        #serverbound_encode
        #serverbound_decode
    }
    .into()
}

/// Create an enum for a list of packets
fn create_packet_enum(enum_name: &Ident, packets: &[PacketID]) -> proc_macro2::TokenStream {
    // Create a variant name for a packet
    let variants = packets.iter().map(|PacketID { module, name, .. }| {
        let variant_name = variant_name(name);
        quote! {
            #variant_name(#module::#name)
        }
    });

    // Create the enum
    quote! {
        #[derive(Debug, Clone, derive_more::From)]
        pub enum #enum_name {
            #(#variants,)*
        }
    }
}

/// Implement `Encode` for a packet enum
fn implement_encode(enum_name: &Ident, packets: &[PacketID]) -> proc_macro2::TokenStream {
    // If there are no packets, always return an error
    if packets.is_empty() {
        return quote! {
            impl crate::buffer::Encode for #enum_name {
                fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                    Err(crate::buffer::EncodeError::NoPackets)
                }
            }
        };
    }

    // Create a match arm for each packet
    let variants = packets.iter().map(|PacketID { id, name, .. }| {
        let variant_name = variant_name(name);
        quote! {
            Self::#variant_name(packet) => {
                crate::buffer::VarEncode::var_encode(&#id, buf)?;
                crate::buffer::Encode::encode(packet, buf)
            }
        }
    });

    // Create the match expression
    quote! {
        impl crate::buffer::Encode for #enum_name {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                match self {
                    #(#variants,)*
                }
            }
        }
    }
}

/// Implement `Decode` for a packet enum
fn implement_decode(enum_name: &Ident, packets: &[PacketID]) -> proc_macro2::TokenStream {
    // Check if there are no packets
    if packets.is_empty() {
        return quote! {
            impl crate::buffer::Decode for #enum_name {
                fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                    Err(crate::buffer::DecodeError::UnknownPacketId(<u32 as crate::buffer::VarDecode>::var_decode(buf)?))
                }
            }
        };
    }

    // Create match arms for each packet
    let variants = packets.iter().map(|PacketID { id, name, module }| {
        quote! {
            #id => {
                #[cfg(feature = "debug")]
                log::trace!("Found packet 0x{:02X} ({}::{})", #id, stringify!(#module), stringify!(#name));
                Ok(#module::#name::decode(buf)?.into())
            }
        }
    });

    // Create the match expression
    quote! {
        impl crate::buffer::Decode for #enum_name {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                match <u32 as crate::buffer::VarDecode>::var_decode(buf)? {
                    #(#variants,)*
                    id => Err(crate::buffer::DecodeError::UnknownPacketId(id)),
                }
            }
        }
    }
}

/// Get the variant name from a packet name
fn variant_name(name: &Ident) -> Ident {
    let mut enum_string = name.to_string();

    if let Some(pos) = enum_string.find("bound") {
        enum_string.replace_range(0..pos + 5, "");
    }
    if let Some(pos) = enum_string.find("Packet") {
        enum_string.replace_range(pos..pos + 6, "");
    }

    Ident::new(&enum_string, name.span())
}

/// The whole macro
#[derive(Debug)]
struct StatePackets {
    state: Ident,
    version: Ident,
    clientbound: PacketMap,
    serverbound: PacketMap,
}

impl StatePackets {
    fn parse_direction(input: ParseStream) -> syn::Result<PacketMap> {
        input.parse::<Token![=>]>()?;

        let content;
        braced!(content in input);

        // If there is a comma, parse it away
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        content.parse()
    }
}

impl Parse for StatePackets {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let version = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;

        let first_token = input.parse::<Ident>()?;
        let first_content = Self::parse_direction(input)?;

        let second_token = input.parse::<Ident>()?;
        let second_content = Self::parse_direction(input)?;

        match (
            first_token.to_string().as_str(),
            second_token.to_string().as_str(),
        ) {
            ("Clientbound", "Serverbound") => Ok(StatePackets {
                state,
                version,
                clientbound: first_content,
                serverbound: second_content,
            }),
            ("Serverbound", "Clientbound") => Ok(StatePackets {
                state,
                version,
                clientbound: second_content,
                serverbound: first_content,
            }),
            ("Clientbound", _) => Err(syn::Error::new(
                first_token.span(),
                "expected `Clientbound` or `Serverbound`",
            )),
            (_, "Serverbound") => Err(syn::Error::new(
                second_token.span(),
                "expected `Clientbound` or `Serverbound`",
            )),
            _ => Err(syn::Error::new(
                first_token.span().join(second_token.span()).unwrap(),
                "expected `Clientbound` or `Serverbound`",
            )),
        }
    }
}

/// A list of packets with their IDs
#[derive(Debug)]
struct PacketMap(Vec<PacketID>);

impl Parse for PacketMap {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut packets = Vec::new();

        while let Ok(id) = input.parse::<LitInt>() {
            let id = id.base10_parse::<u32>()?;
            input.parse::<Token![=>]>()?;

            let module = input.parse::<Ident>()?;
            input.parse::<Token![::]>()?;
            let name = input.parse::<Ident>()?;

            packets.push(PacketID { id, module, name });
            if input.parse::<Token![,]>().is_err() {
                break;
            }
        }

        Ok(PacketMap(packets))
    }
}

/// A packet with its ID
#[derive(Debug)]
struct PacketID {
    id: u32,
    module: Ident,
    name: Ident,
}
