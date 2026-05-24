use parsyng_quote::ToTokens;

use crate::{Parse, ast::item::Item, combinator::GreedyVec};

#[derive(Clone, Debug)]
pub struct Crate {
    items: Vec<Item>,
}

impl Parse for Crate {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let items: GreedyVec<_> = input.parse()?;

        Ok(Self {
            items: items.inner(),
        })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.items.to_tokens(tokens);
    }
}
