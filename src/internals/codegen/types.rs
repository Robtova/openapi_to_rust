use crate::internals::schema::TypeSchema;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Type;

#[derive(Debug)]
pub enum TypeCodegen {
    Int32,
    Int64,
    Float,
    Double,
    String,
    Bool,
    Date,
    DateTime,
    Array(Box<TypeCodegen>),
    Set(Box<TypeCodegen>),
    Object(String),
    Optional(Box<TypeCodegen>),
    Verbatim(Type),
}

impl TypeCodegen {
    #[cfg(feature = "chrono")]
    fn date_tokens() -> TokenStream {
        quote! {chrono::NaiveData}
    }

    #[cfg(not(feature = "chrono"))]
    fn date_tokens() -> TokenStream {
        quote! {String}
    }

    #[cfg(feature = "chrono")]
    fn datetime_tokens() -> TokenStream {
        quote! {chrono::DateTime<Utc>}
    }

    #[cfg(not(feature = "chrono"))]
    fn datetime_tokens() -> TokenStream {
        quote! {String}
    }
}

impl From<&TypeSchema> for TypeCodegen {
    fn from(typ: &TypeSchema) -> Self {
        match typ {
            TypeSchema::Int32 => Self::Int32,
            TypeSchema::Int64 => Self::Int64,
            TypeSchema::Float => Self::Float,
            TypeSchema::Double => Self::Double,
            TypeSchema::String => Self::String,
            TypeSchema::Bool => Self::Bool,
            TypeSchema::Date => Self::Date,
            TypeSchema::DateTime => Self::DateTime,
            TypeSchema::Array(t) => Self::Array(Box::new(Self::from(t.as_ref()))),
            TypeSchema::Set(t) => Self::Set(Box::new(Self::from(t.as_ref()))),
            TypeSchema::Object(reference) => Self::Object(reference.to_owned()),
        }
    }
}

impl ToTokens for TypeCodegen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Int32 => quote! {i32},
            Self::Int64 => quote! {i64},
            Self::Float => quote! {f32},
            Self::Double => quote! {f64},
            Self::String => quote! {String},
            Self::Bool => quote! {bool},
            Self::Date => Self::date_tokens(),
            Self::DateTime => Self::datetime_tokens(),
            Self::Array(t) => quote! {Vec<#t>},
            Self::Set(t) => quote! {std::collections::HashSet<#t>},
            Self::Object(reference) => format_ident!("{}", reference).into_token_stream(),
            Self::Optional(t) => quote! {Option<#t>},
            Self::Verbatim(t) => t.to_token_stream(),
        }
        .to_tokens(tokens)
    }
}
