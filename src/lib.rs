mod internals;

use internals::Error;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Error as SynError};

#[proc_macro_attribute]
pub fn from_openapi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    match internals::from_openapi(&input, &args) {
        Ok(tokens) => tokens,
        Err(err) => <Error as Into<SynError>>::into(err).to_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn check_openapi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);

    match internals::check_openapi(&input, &args) {
        Ok(tokens) => tokens,
        Err(err) => <Error as Into<SynError>>::into(err).to_compile_error(),
    }
    .into()
}
