use crate::internals::error::Error;
use openapiv3::OpenAPI;

pub fn openapi_from_file(path: &str) -> Result<OpenAPI, Error> {
    let file = std::fs::File::open(path).map_err(|_| Error::FileOpenFailed(path.to_string()))?;
    serde_yaml::from_reader(file).map_err(|_| Error::FileReadFailed(path.to_string()))
}

#[allow(unused)]
pub fn bool_true() -> bool {
    true
}

/// This function is a copy of [syn::parse_macro_input::parse]. It allows [AttributeArgs] to be
/// parsed from a [proc_macro2::TokenStream].
#[cfg(test)]
pub fn parse2<T: syn::parse_macro_input::ParseMacroInput>(
    token_stream: proc_macro2::TokenStream,
) -> Result<T, syn::Error> {
    use syn::parse::Parser;

    // This may look strange but it works as `Parser` is implemented for
    // `impl FnOnce(ParseStream) -> Result<T>`
    T::parse.parse2(token_stream)
}
