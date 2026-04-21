use parsyng_quote::ToTokens;

use crate::{ast::tokens::PathSep, parse::Parse, proc_macro::Ident};

#[derive(Clone, Debug)]
pub struct SimplePath {
    start_token: Option<PathSep>,
    root: Ident,
    paths: Vec<(PathSep, Ident)>,
}

impl Parse for SimplePath {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            start_token: input.try_parse::<PathSep>().ok(),
            root: input.parse()?,
            paths: input.parse()?,
        })
    }
}

impl ToTokens for SimplePath {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.root.to_tokens(tokens);
        self.paths.to_tokens(tokens);
    }
}
