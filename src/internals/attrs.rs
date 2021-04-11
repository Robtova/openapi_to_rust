use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct OpenApiArgs {
    pub schema: String,
    #[darling(default, rename = "ref")]
    pub reference: Option<String>,
}
