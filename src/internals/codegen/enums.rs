use crate::internals::ast::EnumContainer;
use crate::internals::schema::{EnumSchema, VariantSchema};
use crate::internals::{ast, Error};
use heck::CamelCase;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{Attribute, Ident, Visibility};

#[derive(Debug)]
pub struct Variant {
    name: Ident,
    attrs: Vec<Attribute>,
    value: String,
}

impl Variant {
    pub fn update(&mut self, other: Variant) {
        self.name = other.name;
        self.attrs = other.attrs;
    }
}

impl From<&ast::Variant> for Variant {
    fn from(variant: &ast::Variant) -> Self {
        Self {
            name: variant.ident.clone(),
            attrs: variant.attrs.clone(),
            value: variant.value(),
        }
    }
}

impl From<&VariantSchema> for Variant {
    fn from(variant: &VariantSchema) -> Self {
        Self {
            name: format_ident!("{}", variant.value.to_camel_case()),
            attrs: vec![],
            value: variant.value.clone(),
        }
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let attrs = &self.attrs;
        (quote! {
            #(#attrs)*
            #name
        })
        .to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct Enum {
    name: Ident,
    variants: Vec<Variant>,
    vis: Visibility,
    attrs: Vec<Attribute>,
}

impl Enum {
    pub fn combined(container: ast::EnumContainer, schema: EnumSchema) -> Result<Self, Error> {
        let mut enm = Self {
            name: container.ident,
            variants: schema.variants.iter().map(Variant::from).collect(),
            vis: container.vis,
            attrs: container.attrs,
        };

        for variant in &container.variants {
            if let Some(v) = enm.get_mut_variant(&variant.value()) {
                v.update(variant.into())
            } else {
                return Err(Error::InvalidEnumVariant(variant.value()));
            }
        }

        enm.variants.sort_by_key(|v| v.name.to_string());

        Ok(enm)
    }

    fn get_mut_variant(&mut self, value: &str) -> Option<&mut Variant> {
        for variant in &mut self.variants {
            if variant.value == value {
                return Some(variant);
            }
        }
        None
    }

    pub fn definition_tokens(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.definition_to_tokens(&mut tokens);
        tokens
    }

    fn definition_to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let variants = &self.variants;
        let vis = &self.vis;
        let attrs = &self.attrs;

        (quote! {
            #(#attrs)*
            #vis enum #name {
                #(#variants),*
            }
        })
        .to_tokens(tokens);
    }
}

impl From<EnumContainer> for Enum {
    fn from(container: EnumContainer) -> Self {
        Self {
            name: container.ident,
            variants: container.variants.iter().map(Variant::from).collect(),
            vis: container.vis,
            attrs: container.attrs,
        }
    }
}

impl ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.definition_to_tokens(tokens);
    }
}
