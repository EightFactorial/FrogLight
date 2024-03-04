use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, DeriveInput, Ident};

/// Generate a `FrogRead` implementation for the given struct.
pub(crate) fn read_as_bitset(input: &DeriveInput) -> TokenStream {
    let syn::Data::Struct(data) = &input.data else { panic!("Only structs can be bitsets") };
    let size = BitsetAttributes::from_attributes(&input.attrs).unwrap().size;
    let name = &input.ident;

    match &data.fields {
        syn::Fields::Named(fields) => {
            // Read the bitset from the buffer
            let mut tokens = quote! {
                let bitset: crate::common::BitSet::<#size> = crate::io::FrogRead::fg_read(buf)?;
            };

            // Get the field idents
            let field_idents =
                fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            // Get the fields from the bitset
            for (index, &field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    let #field = bitset.get_bit(#index).expect("BitSet size is too small");
                });
            }

            // Return the struct
            tokens.extend(quote! {
                Ok(Self {
                    #(
                        #field_idents,
                    )*
                })
            });

            // Generate the impl
            generate_read(name, tokens)
        }
        syn::Fields::Unnamed(fields) => {
            // Read the bitset from the buffer
            let mut tokens = quote! {
                let bitset: crate::common::BitSet::<#size> = crate::io::FrogRead::fg_read(buf)?;
            };

            // Create identifiers for the fields
            let field_idents = (0..fields.unnamed.len())
                .map(|index| Ident::new(&format!("field_{index}"), Span::call_site()))
                .collect::<Vec<_>>();

            // Get the fields from the bitset
            for (index, field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    let #field = bitset.get_bit(#index).expect("BitSet size is too small");
                });
            }

            // Return the struct
            tokens.extend(quote! {
                Ok(Self(
                    #(
                        #field_idents,
                    )*
                ))
            });

            // Generate the impl
            generate_read(name, tokens)
        }
        syn::Fields::Unit => panic!("Unit structs cannot be bitsets"),
    }
}

/// Generate a `FrogWrite` implementation for the given struct.
pub(crate) fn write_as_bitset(input: &DeriveInput) -> TokenStream {
    let syn::Data::Struct(data) = &input.data else { panic!("Only structs can be bitsets") };
    let size = BitsetAttributes::from_attributes(&input.attrs).unwrap().size;
    let name = &input.ident;

    match &data.fields {
        syn::Fields::Named(fields) => {
            // Create the bitset
            let mut tokens = quote! {
                let mut bitset = crate::common::BitSet::<#size>::new();
            };

            // Get the field idents
            let field_idents =
                fields.named.iter().map(|field| field.ident.as_ref().unwrap()).collect::<Vec<_>>();

            // Set the fields in the bitset
            for (index, &field) in field_idents.iter().enumerate() {
                tokens.extend(quote! {
                    bitset.set_bit(#index, self.#field).expect("BitSet size is too small");
                });
            }

            // Write the bitset to the buffer
            tokens.extend(quote! {
                bitset.fg_write(buf)
            });

            // Generate the impl
            generate_write(name, tokens)
        }
        syn::Fields::Unnamed(fields) => {
            // Create identifiers for the fields
            let field_idents = (0..fields.unnamed.len())
                .map(|index| Ident::new(&format!("field_{index}"), Span::call_site()))
                .collect::<Vec<_>>();

            // Split the fields into individual variables
            let mut tokens = quote! {
                let Self(#(
                    #field_idents,
                )*) = *self;
            };

            // Create the bitset
            tokens.extend(quote! {
                let mut bitset = crate::common::BitSet::<#size>::from([#(
                    #field_idents,
                )*]);
            });

            // Write the bitset to the buffer
            tokens.extend(quote! {
                bitset.fg_write(buf)
            });

            // Generate the impl
            generate_write(name, tokens)
        }
        syn::Fields::Unit => panic!("Unit structs cannot be bitsets"),
    }
}

// ---- Helper functions ----

/// Wrap the collected tokens in a `FrogRead` impl.
fn generate_read(name: &Ident, tokens: TokenStream) -> TokenStream {
    quote! {
        impl crate::io::FrogRead for #name {
            fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, crate::io::ReadError>
            where
                Self: Sized,
            {
                #tokens
            }
        }
    }
}

/// Wrap the collected tokens in a `FrogWrite` impl.
fn generate_write(name: &Ident, tokens: TokenStream) -> TokenStream {
    quote! {
        impl crate::io::FrogWrite for #name {
            fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), crate::io::WriteError> {
                #tokens
            }
        }
    }
}

// ---- Attribute parsing ----

/// Attributes for bitset generation.
///
/// # Example
/// ```rust,ignore
/// #[frog(bitset(3))]
/// struct MyBitset {
///     a: bool,
///     b: bool,
///     c: bool,
/// }
/// ```
#[derive(Debug, Clone)]
struct BitsetAttributes {
    size: usize,
}

impl BitsetAttributes {
    fn from_attributes(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut size = None;
        for attr in attrs {
            if attr.path().is_ident("frog") {
                let attr: Self = attr.parse_args()?;
                size = Some(attr.size);
            }
        }

        Ok(Self {
            size: size.ok_or_else(|| {
                syn::Error::new(proc_macro2::Span::call_site(), "No bitset size specified")
            })?,
        })
    }
}

impl Parse for BitsetAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse the attribute name
        input.parse::<syn::Ident>()?;

        // Parse the attribute content
        let content;
        syn::parenthesized!(content in input);

        // Parse the size
        let size = content.parse::<syn::LitInt>()?;
        Ok(Self { size: size.base10_parse::<usize>()? })
    }
}
