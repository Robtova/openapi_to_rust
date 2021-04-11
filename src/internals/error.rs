use proc_macro2::Span;
use std::error::Error as StdError;
use std::fmt;
use syn::Error as SynError;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    CheckFailed(CheckFailure),
    Darling(String),
    EnumNotDefined,
    FileOpenFailed(String),
    FileReadFailed(String),
    InvalidEnumVariant(String),
    InvalidStructField(String),
    MissingComponents,
    ModelNotFound(String),
    UnknownField(String),
    UnsupportedEnumType,
    UnsupportedNestedObjectType,
    UnsupportedSchemaType(String),
    UnsupportedStructType,
}

impl From<darling::Error> for Error {
    fn from(err: darling::Error) -> Self {
        Error::Darling(err.to_string())
    }
}

impl Into<SynError> for Error {
    fn into(self) -> SynError {
        SynError::new(Span::call_site(), self.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CheckFailed(cf) => format!("schema check failed due to {}", cf).fmt(f),
            Error::Darling(err) => err.fmt(f),
            Error::EnumNotDefined => "enum not defined".fmt(f),
            Error::FileOpenFailed(fs) => format!("failed to open file '{}'", fs).fmt(f),
            Error::FileReadFailed(fs) => format!("failed to read file '{}'", fs).fmt(f),
            Error::InvalidEnumVariant(v) => format!("unknown variant '{}'", v).fmt(f),
            Error::InvalidStructField(fi) => format!("unknown field '{}'", fi).fmt(f),
            Error::MissingComponents => "schema missing components structure".fmt(f),
            Error::ModelNotFound(m) => format!("model '{}' not found in schemas", m).fmt(f),
            Error::UnknownField(n) => format!("unknown field '{}'", n).fmt(f),
            Error::UnsupportedEnumType => "enums are only supported for string types".fmt(f),
            Error::UnsupportedNestedObjectType => "nested objects are not supported".fmt(f),
            Error::UnsupportedSchemaType(s) => format!("'{}' is not supported", s).fmt(f),
            Error::UnsupportedStructType => "structs are only supported for object types".fmt(f),
        }
    }
}

impl StdError for Error {}

#[derive(Debug, Eq, PartialEq)]
pub enum CheckFailure {
    MissingFields(Vec<String>),
    MissingVariants(Vec<String>),
    UnknownFields(Vec<String>),
    UnknownVariants(Vec<String>),
}

impl fmt::Display for CheckFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CheckFailure::MissingFields(fs) => format!("missing fields: {}", fs.join(", ")).fmt(f),
            CheckFailure::MissingVariants(vs) => {
                format!("missing variants: {}", vs.join(", ")).fmt(f)
            }
            CheckFailure::UnknownFields(fs) => format!("unknown fields: {}", fs.join(", ")).fmt(f),
            CheckFailure::UnknownVariants(vs) => {
                format!("unknown variants: {}", vs.join(", ")).fmt(f)
            }
        }
    }
}
