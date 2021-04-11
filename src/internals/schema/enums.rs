use crate::internals::Error;
use openapiv3::{Schema, SchemaKind, StringType, Type};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct VariantSchema {
    pub value: String,
}

#[derive(Debug)]
pub struct EnumSchema {
    pub variants: Vec<VariantSchema>,
}

impl TryFrom<&StringType> for EnumSchema {
    type Error = Error;

    fn try_from(typ: &StringType) -> Result<Self, Error> {
        if !typ.enumeration.is_empty() {
            let variants = typ
                .enumeration
                .iter()
                .map(|s| VariantSchema {
                    value: s.to_owned(),
                })
                .collect();

            Ok(Self { variants })
        } else {
            Err(Error::EnumNotDefined)
        }
    }
}

impl TryFrom<&Schema> for EnumSchema {
    type Error = Error;

    fn try_from(schema: &Schema) -> Result<Self, Error> {
        match &schema.schema_kind {
            SchemaKind::Type(Type::String(str)) => Self::try_from(str),
            _ => Err(Error::UnsupportedEnumType),
        }
    }
}
