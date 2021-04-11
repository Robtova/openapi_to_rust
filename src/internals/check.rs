use crate::internals::ast::{EnumContainer, StructContainer};
use crate::internals::error::CheckFailure;
use crate::internals::schema::{EnumSchema, StructSchema};
use crate::internals::Error;
use std::collections::HashSet;

pub trait Check<Schema> {
    fn check(&self, schema: &Schema) -> Result<(), Error>;
}

impl Check<EnumSchema> for EnumContainer {
    fn check(&self, schema: &EnumSchema) -> Result<(), Error> {
        let ast_vars: HashSet<_> = self.variants.iter().map(|v| v.value()).collect();
        let schema_vars: HashSet<_> = schema.variants.iter().map(|v| v.value.clone()).collect();

        if ast_vars == schema_vars {
            Ok(())
        } else if ast_vars.is_subset(&schema_vars) {
            let mut variants: Vec<_> = schema_vars
                .difference(&ast_vars)
                .map(ToOwned::to_owned)
                .collect();
            variants.sort();
            Err(Error::CheckFailed(CheckFailure::MissingVariants(variants)))
        } else {
            let mut variants: Vec<_> = ast_vars
                .difference(&schema_vars)
                .map(ToOwned::to_owned)
                .collect();
            variants.sort();
            Err(Error::CheckFailed(CheckFailure::UnknownVariants(variants)))
        }
    }
}

impl Check<StructSchema> for StructContainer {
    fn check(&self, schema: &StructSchema) -> Result<(), Error> {
        let ast_fields: HashSet<_> = self.fields.iter().map(|f| f.field_name()).collect();

        let schema_fields: HashSet<_, _> =
            schema.fields.iter().map(|f| f.name.to_owned()).collect();

        if ast_fields == schema_fields {
            Ok(())
        } else if ast_fields.is_subset(&schema_fields) {
            let mut fields: Vec<_> = schema_fields
                .difference(&ast_fields)
                .map(ToOwned::to_owned)
                .collect();
            fields.sort();
            Err(Error::CheckFailed(CheckFailure::MissingFields(fields)))
        } else {
            let mut fields: Vec<_> = ast_fields
                .difference(&schema_fields)
                .map(ToOwned::to_owned)
                .collect();
            fields.sort();
            Err(Error::CheckFailed(CheckFailure::UnknownFields(fields)))
        }
    }
}
