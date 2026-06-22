use parsyng_quote::ToTokens;

use crate::{
    ast::{
        delimiter::Braced,
        tokens::{Mod, Semicolon},
    },
    parse::Parse,
    proc_macro::{Delimiter, Ident, TokenStream},
};

#[derive(Clone, Debug)]
pub struct ModItem {
    mod_token: Mod,
    ident: Ident,
    content: Option<Braced<TokenStream>>,
    semi: Option<Semicolon>,
}

impl Parse for ModItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let mod_token = input.parse()?;
        let ident = input.parse()?;
        let (content, semi) = if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Brace
        {
            (Some(input.parse()?), None)
        } else {
            (None, Some(input.parse()?))
        };

        Ok(Self {
            mod_token,
            ident,
            content,
            semi,
        })
    }
}

impl ToTokens for ModItem {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.mod_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.content.to_tokens(tokens);
        self.semi.to_tokens(tokens);
    }
}
