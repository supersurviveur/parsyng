use crate::ToTokens;

use crate::{
    ast::{
        expression::{ExpressionWithBlock, ExpressionWithoutBlock},
        item::Item,
        tokens::Semicolon,
    },
    error::Diagnostics,
    parse::Parse,
};

#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Semicolon(Semicolon),
    Item(Item),
    ExpressionWithBlock(ExpressionWithBlock, Semicolon),
    ExpressionWithoutBlock(ExpressionWithoutBlock, Option<Semicolon>),
}

impl Parse for Statement {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(semicolon) = input.try_parse() {
            Ok(Self::Semicolon(semicolon))
        } else if let Ok(item) = input.try_parse() {
            Ok(Self::Item(item))
        } else if let Ok((expression, semicolon)) = input.try_parse() {
            Ok(Self::ExpressionWithBlock(expression, semicolon))
        } else if let Ok((expression, semicolon)) = input.try_parse() {
            Ok(Self::ExpressionWithoutBlock(expression, semicolon))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a statement",
                input.span(),
            ))
        }
    }
}

impl ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Statement::Semicolon(semicolon) => semicolon.to_tokens(tokens),
            Statement::Item(item) => item.to_tokens(tokens),
            Statement::ExpressionWithBlock(expression, semicolon) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
            }
            Statement::ExpressionWithoutBlock(expression, semicolon) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
            }
        }
    }
}
