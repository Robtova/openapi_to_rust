use crate::internals::schema::types::TypeSchema;
use crate::internals::Error;
use openapiv3::{ObjectType, Schema, SchemaKind, Type};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct FieldSchema {
    pub ty: TypeSchema,
    pub name: String,
    pub required: bool,
}

#[derive(Debug)]
pub struct StructSchema {
    pub fields: Vec<FieldSchema>,
}

impl TryFrom<&ObjectType> for StructSchema {
    type Error = Error;

    fn try_from(obj: &ObjectType) -> Result<Self, Error> {
        let mut fields = HashMap::new();

        for (name, prop) in &obj.properties {
            fields.insert(
                name.clone(),
                FieldSchema {
                    ty: TypeSchema::try_from(prop)?,
                    name: name.clone(),
                    required: false,
                },
            );
        }

        for required in &obj.required {
            match fields.get_mut(required) {
                Some(field) => field.required = true,
                None => return Err(Error::UnknownField(required.to_owned())),
            }
        }

        Ok(Self {
            fields: fields.drain().map(|(_, f)| f).collect(),
        })
    }
}

impl TryFrom<&Schema> for StructSchema {
    type Error = Error;

    fn try_from(schema: &Schema) -> Result<Self, Error> {
        match &schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => Self::try_from(obj),
            _ => Err(Error::UnsupportedStructType),
        }
    }
}
