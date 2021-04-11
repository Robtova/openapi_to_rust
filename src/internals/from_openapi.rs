use crate::internals::ast::{Container, EnumContainer, StructContainer};
use crate::internals::attrs::OpenApiArgs;
use crate::internals::codegen::{Enum, Struct};
use crate::internals::error::Error;
use crate::internals::utils::openapi_from_file;
use darling::ast::Data;
use darling::{FromDeriveInput, FromMeta};
use openapiv3::{ReferenceOr, Schema};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::convert::TryInto;
use syn::{AttributeArgs, DeriveInput};

pub fn from_openapi(ast: &DeriveInput, attrs: &AttributeArgs) -> Result<TokenStream, Error> {
    let container: Container = Container::from_derive_input(ast)?;
    let args: OpenApiArgs = OpenApiArgs::from_list(attrs)?;

    match &container.data {
        Data::Enum(_) => Ok(from_openapi_enum(container.into(), args)?),
        Data::Struct(_) => Ok(from_openapi_struct(container.into(), args)?),
    }
}

fn from_openapi_enum(container: EnumContainer, args: OpenApiArgs) -> Result<TokenStream, Error> {
    let schema = openapi_from_file(&args.schema)?;
    let components = schema.components.ok_or(Error::MissingComponents)?;

    let reference = args
        .reference
        .unwrap_or_else(|| container.ident.to_string());

    let component: &Schema = match components.schemas.get(&reference) {
        Some(ReferenceOr::Item(schema)) => schema,
        _ => return Err(Error::ModelNotFound(reference).into()),
    };

    Ok(Enum::combined(container, component.try_into()?)?.into_token_stream())
}

fn from_openapi_struct(
    container: StructContainer,
    args: OpenApiArgs,
) -> Result<TokenStream, Error> {
    let schema = openapi_from_file(&args.schema)?;
    let components = schema.components.ok_or(Error::MissingComponents)?;

    let reference = args
        .reference
        .unwrap_or_else(|| container.ident.to_string());

    let component: &Schema = match components.schemas.get(&reference) {
        Some(ReferenceOr::Item(schema)) => schema,
        _ => return Err(Error::ModelNotFound(reference).into()),
    };

    Ok(Struct::combined(container, component.try_into()?)?.into_token_stream())
}

#[cfg(test)]
mod tests {
    use crate::internals::from_openapi::from_openapi;
    use crate::internals::utils::parse2;
    use quote::quote;
    use syn::{AttributeArgs, DeriveInput};

    #[test]
    fn test_enum() {
        let args = quote! { schema = "./test-resources/test_schema.yaml" };
        let input = quote! {
            enum TestEnum {}
        };

        let args: AttributeArgs = parse2(args).unwrap();
        let input: DeriveInput = parse2(input).unwrap();

        let expected = quote! {
            enum TestEnum {
                Bar,
                Foo
            }
        };

        assert_eq!(
            from_openapi(&input, &args).map(|s| s.to_string()),
            Ok(expected.to_string())
        );
    }

    #[test]
    fn test_struct() {
        let args = quote! { schema = "./test-resources/test_schema.yaml" };
        let input = quote! {
            struct TestStruct;
        };

        let args: AttributeArgs = parse2(args).unwrap();
        let input: DeriveInput = parse2(input).unwrap();

        let expected = quote! {
            struct TestStruct {
                pub bar: Option<i32>,
                pub foo: String
            }
        };

        assert_eq!(
            from_openapi(&input, &args).map(|s| s.to_string()),
            Ok(expected.to_string())
        );
    }
}
