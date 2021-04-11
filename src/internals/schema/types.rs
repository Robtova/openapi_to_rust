use crate::internals::Error;
use openapiv3::{
    ArrayType, IntegerFormat, IntegerType, NumberFormat, NumberType, ReferenceOr, Schema,
    SchemaKind, StringFormat, StringType, Type,
};
use std::convert::TryFrom;

/// The OpenAPI types supported by [openapi_to_rust]. This currently includes most of the formats
/// defined in the OpenAPI Specification (except for objects which are only supported via reference)
/// as well as sets.
#[derive(Debug)]
pub enum TypeSchema {
    /// Signed 32-bit integer.
    Int32,
    /// Signed 64-bit integer.
    Int64,
    /// 32-bit floating-point number.
    Float,
    /// 64-bit floating-point number.
    Double,
    /// String.
    String,
    // /// String of base64 encoded characters.
    // Bytes,
    // /// String of octets.
    // Binary,
    /// Boolean value.
    Bool,
    /// Date string conforming to RFC3339.
    Date,
    /// Date-time string conforming to RFC3339.
    DateTime,
    /// Array of a specific type.
    Array(Box<TypeSchema>),
    /// Array of unique items of a specific type.
    Set(Box<TypeSchema>),
    /// A reference to an object.
    Object(String),
}

impl TryFrom<&Type> for TypeSchema {
    type Error = Error;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        use openapiv3::VariantOrUnknownOrEmpty::*;

        match &value {
            Type::String(StringType {
                format: Item(StringFormat::Date),
                ..
            }) => Ok(Self::Date),
            Type::String(StringType {
                format: Item(StringFormat::DateTime),
                ..
            }) => Ok(Self::DateTime),
            Type::String(_) => Ok(Self::String),
            Type::Number(NumberType {
                format: Item(NumberFormat::Double),
                ..
            }) => Ok(Self::Double),
            Type::Number(_) => Ok(Self::Float),
            Type::Integer(IntegerType {
                format: Item(IntegerFormat::Int64),
                ..
            }) => Ok(Self::Int64),
            Type::Integer(_) => Ok(Self::Int32),
            Type::Object(_) => Err(Error::UnsupportedNestedObjectType),
            Type::Array(ArrayType {
                items,
                unique_items: true,
                ..
            }) => Ok(Self::Set(Box::new(Self::try_from(items)?))),
            Type::Array(ArrayType { items, .. }) => {
                Ok(Self::Array(Box::new(Self::try_from(items)?)))
            }
            Type::Boolean { .. } => Ok(Self::Bool),
        }
    }
}

impl TryFrom<&Schema> for TypeSchema {
    type Error = Error;

    fn try_from(value: &Schema) -> Result<Self, Self::Error> {
        match &value.schema_kind {
            SchemaKind::Type(typ) => Self::try_from(typ),
            SchemaKind::OneOf { .. } => Err(Error::UnsupportedSchemaType("oneOf".to_string())),
            SchemaKind::AllOf { .. } => Err(Error::UnsupportedSchemaType("allOf".to_string())),
            SchemaKind::AnyOf { .. } => Err(Error::UnsupportedSchemaType("anyOf".to_string())),
            SchemaKind::Any(_) => Err(Error::UnsupportedSchemaType("any".to_string())),
        }
    }
}

impl TryFrom<&ReferenceOr<Box<Schema>>> for TypeSchema {
    type Error = Error;

    fn try_from(value: &ReferenceOr<Box<Schema>>) -> Result<Self, Self::Error> {
        match &value {
            ReferenceOr::Reference { reference } => Ok(Self::Object(reference.to_owned())),
            ReferenceOr::Item(schema) => Self::try_from(schema.as_ref()),
        }
    }
}
