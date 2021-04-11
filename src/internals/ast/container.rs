use crate::internals::ast::enums::Variant;
use crate::internals::ast::structs::Field;
use darling::ast::Data;
use darling::FromDeriveInput;
use syn::{Attribute, Ident, Visibility};

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(openapi),
    forward_attrs,
    supports(enum_unit, struct_named, struct_unit)
)]
pub struct Container {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub data: Data<Variant, Field>,
    pub vis: Visibility,
    #[darling(default)]
    pub allow_type_mismatch: Option<()>,
}
