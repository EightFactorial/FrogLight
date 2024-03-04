use proc_macro::TokenStream;
use syn::DeriveInput;

mod readwrite;
mod tests;

/// Generates a `FrogRead` and `FrogWrite` implementation for the given struct
/// or enum.
pub(super) fn frog_read_write(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_read(&input)));
    output.extend(TokenStream::from(readwrite::generate_write(&input)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input)));
    }
    output
}

/// Generates a `FrogRead` implementation for the given struct or enum.
pub(super) fn frog_read(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_read(&input)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input)));
    }
    output
}

/// Generates a `FrogWrite` implementation for the given struct or enum.
pub(super) fn frog_write(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_write(&input)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input)));
    }
    output
}

/// Generates tests for `FrogRead` and `FrogWrite` implementations.
pub(super) fn frog_test(tokens: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    TokenStream::from(tests::generate_tests(&input))
}
