use crate::ToTokens;

use crate::{
    ast::{
        delimiter::Bracketed,
        tokens::{Not, Pound},
    },
    error::Diagnostics,
    parse::{Parse, ParseBuffer},
    proc_macro::{Span, TokenStream},
};

#[derive(Clone, Debug)]
pub struct Attribute {
    pound: Pound,
    bang: Option<Not>,
    meta: Bracketed<TokenStream>,
}

impl Attribute {
    pub fn is_inner(&self) -> bool {
        self.bang.is_some()
    }

    pub fn span(&self) -> Span {
        self.meta.span()
    }
}

impl Parse for Attribute {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            pound: input.parse()?,
            bang: input.try_parse().ok(),
            meta: input.parse()?,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.pound.to_tokens(tokens);
        self.bang.to_tokens(tokens);
        self.meta.to_tokens(tokens);
    }
}

pub fn parse_outer_attributes(input: &mut ParseBuffer) -> Vec<Attribute> {
    let mut attributes = Vec::new();
    while let Ok(attribute) = input.try_advance(|input| {
        let attribute: Attribute = input.parse()?;
        if attribute.is_inner() {
            Err(Diagnostics::new_error_spanned(
                "Expected outer attribute",
                attribute.span(),
            ))
        } else {
            Ok(attribute)
        }
    }) {
        attributes.push(attribute);
    }
    attributes
}

pub fn parse_inner_attributes(input: &mut ParseBuffer) -> Vec<Attribute> {
    let mut attributes = Vec::new();
    while let Ok(attribute) = input.try_advance(|input| {
        let attribute: Attribute = input.parse()?;
        if attribute.is_inner() {
            Ok(attribute)
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected inner attribute",
                attribute.span(),
            ))
        }
    }) {
        attributes.push(attribute);
    }
    attributes
}
