use parsyng_quote::ToTokens;

use crate::{
    ast::{delimiter::Braced, tokens::Extern},
    parse::Parse,
    proc_macro::{Literal, TokenStream},
};

#[derive(Clone, Debug)]
pub struct ExternBlockItem {
    extern_token: Extern,
    abi: Option<Literal>,
    items: Braced<TokenStream>,
}

impl Parse for ExternBlockItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            extern_token: input.parse()?,
            abi: input.try_parse().ok(),
            items: input.parse()?,
        })
    }
}

impl ToTokens for ExternBlockItem {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.extern_token.to_tokens(tokens);
        self.abi.to_tokens(tokens);
        self.items.to_tokens(tokens);
    }
}
