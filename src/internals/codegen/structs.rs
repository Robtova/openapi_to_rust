use crate::internals::ast::StructContainer;
use crate::internals::codegen::types::TypeCodegen;
use crate::internals::schema::{FieldSchema, StructSchema};
use crate::internals::{ast, Error};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Ident, VisPublic, Visibility};

#[derive(Debug)]
pub struct Field {
    ident: Ident,
    name: String,
    ty: TypeCodegen,
    vis: Visibility,
    attrs: Vec<Attribute>,
}

impl Field {
    pub fn update(&mut self, other: Field) {
        self.ident = other.ident;
        self.ty = other.ty;
        self.vis = other.vis;
        self.attrs = other.attrs;
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let ty = &self.ty;
        let vis = &self.vis;
        let attrs = &self.attrs;

        (quote! {
            #(#attrs)*
            #vis #ident: #ty
        })
        .to_tokens(tokens);
    }
}

impl From<&ast::Field> for Field {
    fn from(field: &ast::Field) -> Self {
        Self {
            ident: field.ident.clone().unwrap(),
            name: field.field_name(),
            ty: TypeCodegen::Verbatim(field.ty.clone()),
            vis: field.vis.clone(),
            attrs: field.attrs.clone(),
        }
    }
}

impl From<&FieldSchema> for Field {
    fn from(field: &FieldSchema) -> Self {
        Self {
            ident: format_ident!("{}", field.name.to_snake_case()),
            name: field.name.clone(),
            ty: if field.required {
                TypeCodegen::from(&field.ty)
            } else {
                TypeCodegen::Optional(Box::new(TypeCodegen::from(&field.ty)))
            },
            vis: Visibility::Public(VisPublic {
                pub_token: Default::default(),
            }),
            attrs: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Struct {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,
    pub fields: Vec<Field>,
    pub vis: Visibility,
}

impl Struct {
    pub fn combined(container: ast::StructContainer, schema: StructSchema) -> Result<Self, Error> {
        let mut strct = Self {
            ident: container.ident,
            vis: container.vis,
            attrs: container.attrs,
            fields: schema.fields.iter().map(Field::from).collect(),
        };

        for field in &container.fields {
            if let Some(f) = strct.get_mut_field(&field.field_name()) {
                f.update(field.into())
            } else {
                return Err(Error::InvalidStructField(field.field_name()));
            }
        }

        strct.fields.sort_by_key(|f| f.ident.to_string());

        Ok(strct)
    }

    fn get_mut_field(&mut self, value: &str) -> Option<&mut Field> {
        for field in &mut self.fields {
            if field.name == value {
                return Some(field);
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
        let ident = &self.ident;
        let attrs = &self.attrs;
        let fields = &self.fields;
        let vis = &self.vis;

        (quote! {
            #(#attrs)*
            #vis struct #ident {
                #(#fields),*
            }
        })
        .to_tokens(tokens);
    }
}

impl From<StructContainer> for Struct {
    fn from(container: StructContainer) -> Self {
        Self {
            ident: container.ident,
            fields: container.fields.iter().map(Field::from).collect(),
            vis: container.vis,
            attrs: container.attrs,
        }
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.definition_to_tokens(tokens)
    }
}
