use crate::ToTokens;

use crate::{
    Parse,
    ast::{attributes::parse_inner_attributes, item::Item},
};

#[derive(Clone, Debug)]
pub struct Crate {
    inner_attributes: Vec<crate::ast::attributes::Attribute>,
    items: Vec<Item>,
}

impl Parse for Crate {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let inner_attributes = parse_inner_attributes(input);

        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse::<crate::ast::item::Item>()?);
        }

        Ok(Self {
            inner_attributes,
            items,
        })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.inner_attributes.to_tokens(tokens);
        self.items.to_tokens(tokens);
    }
}
