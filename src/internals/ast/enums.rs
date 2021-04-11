use crate::internals::ast::container::Container;
use darling::FromVariant;
use syn::{Attribute, Ident, Visibility};

#[derive(Clone, Debug, FromVariant)]
#[darling(attributes(openapi), forward_attrs)]
pub struct Variant {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    #[darling(default)]
    pub value: Option<String>,
}

impl Variant {
    pub fn value(&self) -> String {
        self.value.clone().unwrap_or_else(|| self.ident.to_string())
    }
}

#[derive(Debug)]
pub struct EnumContainer {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub variants: Vec<Variant>,
    pub vis: Visibility,
}

impl From<Container> for EnumContainer {
    fn from(container: Container) -> Self {
        Self {
            ident: container.ident,
            attrs: container.attrs,
            variants: container.data.take_enum().unwrap(),
            vis: container.vis,
        }
    }
}
