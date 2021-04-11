use crate::internals::ast::{Container, EnumContainer, StructContainer};
use crate::internals::attrs::OpenApiArgs;
use crate::internals::check::Check;
use crate::internals::codegen::{Enum, Struct};
use crate::internals::error::Error;
use crate::internals::schema::{EnumSchema, StructSchema};
use crate::internals::utils::openapi_from_file;
use darling::ast::Data;
use darling::{FromDeriveInput, FromMeta};
use openapiv3::{ReferenceOr, Schema};
use proc_macro2::TokenStream;
use std::convert::TryFrom;
use syn::{AttributeArgs, DeriveInput};

pub fn check_openapi(ast: &DeriveInput, attrs: &AttributeArgs) -> Result<TokenStream, Error> {
    let container: Container = Container::from_derive_input(ast)?;
    let args: OpenApiArgs = OpenApiArgs::from_list(attrs)?;

    match &container.data {
        Data::Enum(_) => check_openapi_enum(container.into(), args),
        Data::Struct(_) => check_openapi_struct(container.into(), args),
    }
}

fn check_openapi_enum(cont: EnumContainer, args: OpenApiArgs) -> Result<TokenStream, Error> {
    let schema = openapi_from_file(&args.schema)?;
    let components = schema.components.ok_or(Error::MissingComponents)?;

    let reference = args.reference.unwrap_or_else(|| cont.ident.to_string());

    let component: &Schema = match components.schemas.get(&reference) {
        Some(ReferenceOr::Item(schema)) => schema,
        _ => return Err(Error::ModelNotFound(reference).into()),
    };

    cont.check(&EnumSchema::try_from(component)?)?;

    Ok(Enum::from(cont).definition_tokens())
}

fn check_openapi_struct(cont: StructContainer, args: OpenApiArgs) -> Result<TokenStream, Error> {
    let schema = openapi_from_file(&args.schema)?;
    let components = schema.components.ok_or(Error::MissingComponents)?;

    let reference = args.reference.unwrap_or_else(|| cont.ident.to_string());

    let component: &Schema = match components.schemas.get(&reference) {
        Some(ReferenceOr::Item(schema)) => schema,
        _ => return Err(Error::ModelNotFound(reference).into()),
    };

    cont.check(&StructSchema::try_from(component)?)?;

    Ok(Struct::from(cont).definition_tokens())
}

#[cfg(test)]
mod tests {
    use crate::internals::error::CheckFailure;
    use crate::internals::utils::parse2;
    use crate::internals::{check_openapi, Error};
    use quote::quote;
    use syn::{AttributeArgs, DeriveInput};

    #[test]
    fn test_enum_success() {
        let args = quote! { schema = "./test-resources/test_schema.yaml" };
        let input = quote! {
            #[derive(Clone, Debug)]
            enum TestEnum {
                #[openapi(value = "foo")]
                Foo,
                #[openapi(value = "bar")]
                #[some_attribute]
                Bar,
            }
        };

        let args: AttributeArgs = parse2(args).unwrap();
        let input: DeriveInput = parse2(input).unwrap();

        let expected = quote! {
            #[derive(Clone, Debug)]
            enum TestEnum {
                Foo,
                #[some_attribute]
                Bar
            }
        };

        assert_eq!(
            check_openapi(&input, &args).map(|s| s.to_string()),
            Ok(expected.to_string())
        )
    }

    #[test]
    fn test_enum_missing_variants() {
        let args = quote! { schema = "./test-resources/test_schema.yaml" };
        let input = quote! {
            #[derive(Clone, Debug)]
            enum TestEnum {
                #[openapi(value = "bar")]
                #[some_attribute]
                Bar,
            }
        };

        let args: AttributeArgs = parse2(args).unwrap();
        let input: DeriveInput = parse2(input).unwrap();

        assert_eq!(
            check_openapi(&input, &args).map(|s| s.to_string()),
            Err(Error::CheckFailed(CheckFailure::MissingVariants(vec![
                "foo".to_string()
            ])))
        )
    }

    #[test]
    fn test_enum_unknown_variants() {
        let args = quote! { schema = "./test-resources/test_schema.yaml" };
        let input = quote! {
            #[derive(Clone, Debug)]
            enum TestEnum {
                Test,
                Foo,
                #[openapi(value = "bar")]
                #[some_attribute]
                Bar,
            }
        };

        let args: AttributeArgs = parse2(args).unwrap();
        let input: DeriveInput = parse2(input).unwrap();

        assert_eq!(
            check_openapi(&input, &args).map(|s| s.to_string()),
            Err(Error::CheckFailed(CheckFailure::UnknownVariants(vec![
                "Foo".to_string(),
                "Test".to_string()
            ])))
        )
    }
}
