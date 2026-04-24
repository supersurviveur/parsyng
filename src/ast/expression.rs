use parsyng_quote::ToTokens;

use crate::{ast::literal::Literal, error::Diagnostics, parse::Parse};

#[derive(Clone, Debug)]
pub enum Expression {
    WithoutBlock(ExpressionWithoutBlock),
    WithBlock(ExpressionWithBlock),
}

#[derive(Clone, Debug)]
pub enum ExpressionWithoutBlock {
    Literal(Literal),
}

#[derive(Clone, Debug)]
pub enum ExpressionWithBlock {}

impl Parse for ExpressionWithoutBlock {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(literal) = input.try_parse() {
            Ok(Self::Literal(literal))
        // } else if let Ok(implementation) = input.try_parse() {
        //     Ok(Self::Impl(implementation))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an expression without block",
                input.span(),
            ))
        }
    }
}

impl Parse for ExpressionWithBlock {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        // if let Ok(literal) = input.try_parse() {
        //     Ok(Self::Literal(literal))
        // } else if let Ok(implementation) = input.try_parse() {
        //     Ok(Self::Impl(implementation))
        // } else {
        Err(Diagnostics::new_error_spanned(
            "Expected an expression with block",
            input.span(),
        ))
        // }
    }
}

impl Parse for Expression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(block) = input.try_parse() {
            Ok(Self::WithBlock(block))
        } else if let Ok(without_block) = input.try_parse() {
            Ok(Self::WithoutBlock(without_block))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an expression",
                input.span(),
            ))
        }
    }
}
impl ToTokens for ExpressionWithoutBlock {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            ExpressionWithoutBlock::Literal(literal) => literal.to_tokens(tokens),
        }
    }
}

impl ToTokens for ExpressionWithBlock {
    fn to_tokens(&self, _tokens: &mut parsyng_quote::proc_macro::TokenStream) {}
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Expression::WithoutBlock(expression_without_block) => {
                expression_without_block.to_tokens(tokens)
            }
            Expression::WithBlock(expression_with_block) => expression_with_block.to_tokens(tokens),
        }
    }
}
