//! Statements, i.e. the contents of a `{ ... }` block.

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

/// One statement inside a block: an empty `;`, a local item declaration, or
/// an expression.
///
/// The expression variants carry an optional trailing `;` — a
/// semicolon-less expression-without-block in tail position is the block's
/// value.
///
/// Reference: <https://doc.rust-lang.org/reference/statements.html>
#[derive(Clone, Debug)]
pub enum Statement {
    /// An empty `;` statement.
    Semicolon(Semicolon),
    /// A local item declaration.
    ///
    /// Reference: <https://doc.rust-lang.org/reference/statements.html#item-declarations>
    Item(Box<Item>),
    /// An expression ending in a block, followed by `;`.
    ///
    /// Reference: <https://doc.rust-lang.org/reference/statements.html#expression-statements>
    ExpressionWithBlock(ExpressionWithBlock, Semicolon),
    /// An expression without a block, with an optional trailing `;`.
    ///
    /// Reference: <https://doc.rust-lang.org/reference/statements.html#expression-statements>
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
            Self::Semicolon(semicolon) => semicolon.to_tokens(tokens),
            Self::Item(item) => item.to_tokens(tokens),
            Self::ExpressionWithBlock(expression, semicolon) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
            }
            Self::ExpressionWithoutBlock(expression, semicolon) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
            }
        }
    }
}
