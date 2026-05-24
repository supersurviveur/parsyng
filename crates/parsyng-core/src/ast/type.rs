use parsyng_quote::ToTokens;

use crate::{
    ast::{path::TypePathSegment, tokens::PathSep},
    error::Diagnostics,
    parse::Parse,
};

#[derive(Clone, Debug)]
pub enum Type {
    Path(Box<TypePath>),
}
#[derive(Clone, Debug)]
pub struct TypePath {
    start_token: Option<PathSep>,
    root: TypePathSegment,
    paths: Vec<(PathSep, TypePathSegment)>,
}

impl Parse for TypePath {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            start_token: input.try_parse::<PathSep>().ok(),
            root: input.parse()?,
            paths: input.parse()?,
        })
    }
}

impl ToTokens for TypePath {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.root.to_tokens(tokens);
        self.paths.to_tokens(tokens);
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Type::Path(type_path) => type_path.to_tokens(tokens),
        }
    }
}
impl Parse for Type {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let mut diagnostics = Diagnostics::empty();
        match input.try_parse() {
            Ok(ok) => return Ok(Self::Path(ok)),
            Err(err) => diagnostics.join(err),
        }
        Err(diagnostics)
    }
}
