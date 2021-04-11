use crate::internals::ast::Container;
use darling::FromField;
use syn::{Attribute, Ident, Type, Visibility};

#[derive(Debug, FromField)]
#[darling(attributes(openapi), forward_attrs)]
pub struct Field {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
    #[darling(default)]
    pub name: Option<String>,
    #[darling(default)]
    pub allow_type_mismatch: Option<()>,
}

impl Field {
    pub fn field_name(&self) -> String {
        // .unwrap() will not panic here as we only use named structs
        self.name
            .clone()
            .unwrap_or_else(|| self.ident.as_ref().unwrap().to_string())
    }
}

#[derive(Debug)]
pub struct StructContainer {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub fields: Vec<Field>,
    pub vis: Visibility,
    pub allow_type_mismatch: bool,
}

impl From<Container> for StructContainer {
    fn from(container: Container) -> Self {
        Self {
            ident: container.ident,
            attrs: container.attrs,
            fields: container.data.take_struct().unwrap().fields,
            vis: container.vis,
            allow_type_mismatch: container.allow_type_mismatch.is_some(),
        }
    }
}
