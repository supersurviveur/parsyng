use parsyng_quote::{ToTokens, proc_macro::Delimiter};

use crate::{
    ast::{
        delimiter::{Bracketed, Parenthesized},
        item::Lifetime,
        literal::{Literal, LiteralNumber},
        tokens::{Await, Break, Comma, Continue, Dot, DotDot, DotDotEq, Return, Semicolon},
    },
    combinator::Punctuated,
    error::Diagnostics,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub enum Expression {
    WithoutBlock(Box<ExpressionWithoutBlock>),
    WithBlock(ExpressionWithBlock),
}

#[derive(Clone, Debug)]
pub enum ExpressionWithoutBlock {
    Literal(Literal),
    Await(AwaitExpression),
    Index(IndexExpression),
    Array(ArrayExpression),
    Tuple(TupleExpression),
    TupleIndex(TupleIndexExpression),
    Field(FieldExpression),
    Return(ReturnExpression),
    Continue(ContinueExpression),
    Break(BreakExpression),
    Underscore(UnderscoreExpression),
    Grouped(GroupedExpression),
    Call(CallExpression),
    Range(RangeExpression),
}

#[derive(Clone, Debug)]
pub enum ExpressionWithBlock {}

#[derive(Clone, Debug)]
pub struct AwaitExpression {
    expr: Expression,
    dot: Dot,
    await_token: Await,
}
#[derive(Clone, Debug)]
pub struct IndexExpression {
    expr: Expression,
    index: Bracketed<Expression>,
}
#[derive(Clone, Debug)]
pub struct TupleExpression {
    exprs: Parenthesized<Punctuated<Expression, Comma>>,
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
    exprs: Bracketed<ArrayElements>,
}

#[derive(Clone, Debug)]
pub enum ArrayElements {
    Repetition(Expression, Semicolon, Expression),
    List(Punctuated<Expression, Comma>),
}
#[derive(Clone, Debug)]
pub struct TupleIndexExpression {
    expr: Expression,
    dot: Dot,
    index: LiteralNumber,
}

#[derive(Clone, Debug)]
pub struct FieldExpression {
    expr: Expression,
    dot: Dot,
    field: Ident,
}

#[derive(Clone, Debug)]
pub struct ReturnExpression {
    return_token: Return,
    expr: Expression,
}

#[derive(Clone, Debug)]
pub struct ContinueExpression {
    continue_token: Continue,
    label: Option<Lifetime>,
}
#[derive(Clone, Debug)]
pub struct BreakExpression {
    break_token: Break,
    label: Option<Lifetime>,
    expr: Option<Expression>,
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    expr: Expression,
    params: Parenthesized<Punctuated<Expression, Comma>>,
}
#[derive(Clone, Debug)]
pub struct RangeExpression {
    start: Option<Expression>,
    dot: Option<DotDot>,
    dot_eq: Option<DotDotEq>,
    end: Option<Expression>,
}
#[derive(Clone, Debug)]
pub struct UnderscoreExpression {
    underscore: Ident,
}
#[derive(Clone, Debug)]
pub struct GroupedExpression {
    group: Parenthesized<Expression>,
}

impl Parse for ExpressionWithoutBlock {
    #[allow(clippy::cmp_owned)]
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Some(ident) = input.peek_ident() {
            if ident.to_string() == "return" {
                return Ok(Self::Return(input.parse()?));
            } else if ident.to_string() == "break" {
                return Ok(Self::Break(input.parse()?));
            } else if ident.to_string() == "continue" {
                return Ok(Self::Continue(input.parse()?));
            }
        }
        if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Parenthesis
        {
            if let Ok(tuple) = input.try_parse() {
                return Ok(Self::Tuple(tuple));
            }
            return Ok(Self::Grouped(input.parse()?));
        }
        if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Bracket
        {
            return Ok(Self::Array(input.parse()?));
        }
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
            ExpressionWithoutBlock::Await(await_expression) => await_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Index(index_expression) => index_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Tuple(tuple_expression) => tuple_expression.to_tokens(tokens),
            ExpressionWithoutBlock::TupleIndex(tuple_index_expression) => {
                tuple_index_expression.to_tokens(tokens)
            }
            ExpressionWithoutBlock::Field(field_expression) => field_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Return(return_expression) => {
                return_expression.to_tokens(tokens)
            }
            ExpressionWithoutBlock::Continue(continue_expression) => {
                continue_expression.to_tokens(tokens)
            }
            ExpressionWithoutBlock::Break(break_expression) => break_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Underscore(underscore_expression) => {
                underscore_expression.to_tokens(tokens)
            }
            ExpressionWithoutBlock::Grouped(grouped_expression) => {
                grouped_expression.to_tokens(tokens)
            }
            ExpressionWithoutBlock::Call(call_expression) => call_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Range(range_expression) => range_expression.to_tokens(tokens),
            ExpressionWithoutBlock::Array(array_expression) => array_expression.to_tokens(tokens),
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

impl ToTokens for AwaitExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.await_token.to_tokens(tokens);
    }
}
impl ToTokens for FieldExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.field.to_tokens(tokens);
    }
}
impl ToTokens for TupleExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.exprs.to_tokens(tokens);
    }
}
impl ToTokens for IndexExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.index.to_tokens(tokens);
    }
}
impl ToTokens for TupleIndexExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.index.to_tokens(tokens);
    }
}
impl ToTokens for ReturnExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.return_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
impl ToTokens for ContinueExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.continue_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
    }
}
impl ToTokens for BreakExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.break_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
impl ToTokens for CallExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.params.to_tokens(tokens);
    }
}
impl ToTokens for RangeExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.start.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.dot_eq.to_tokens(tokens);
        self.end.to_tokens(tokens);
    }
}
impl ToTokens for UnderscoreExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.underscore.to_tokens(tokens);
    }
}
impl ToTokens for GroupedExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.group.to_tokens(tokens);
    }
}

impl Parse for ReturnExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            return_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl Parse for BreakExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            break_token: input.parse()?,
            label: input.try_parse().ok(),
            expr: input.try_parse().ok(),
        })
    }
}

impl Parse for ContinueExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            continue_token: input.parse()?,
            label: input.try_parse().ok(),
        })
    }
}
impl Parse for GroupedExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            group: input.parse()?,
        })
    }
}
impl Parse for TupleExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let exprs: Parenthesized<Punctuated<Expression, _>> = input.parse()?;

        if exprs.is_empty() || (exprs.len() == 1 && exprs.trailing().is_none()) {
            return Err(Diagnostics::new_error_spanned("", exprs.span()));
        }
        Ok(Self { exprs })
    }
}
impl ToTokens for ArrayElements {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            ArrayElements::Repetition(expression, semicolon, expression1) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
                expression1.to_tokens(tokens);
            }
            ArrayElements::List(punctuated) => {
                punctuated.to_tokens(tokens);
            }
        }
    }
}
impl ToTokens for ArrayExpression {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.exprs.to_tokens(tokens);
    }
}
impl Parse for ArrayElements {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if input.is_empty() {
            return Ok(Self::List(Punctuated::empty()));
        }
        let first = input.parse()?;
        if let Ok(semicolon) = input.peek_parse() {
            return Ok(Self::Repetition(first, semicolon, input.parse()?));
        }
        if !input.is_empty() {
            let comma = input.parse()?;
            let mut list: Punctuated<Expression, _> = input.parse()?;
            list.push_back((first, comma));
            Ok(Self::List(list))
        } else {
            Ok(Self::List(Punctuated::one(first)))
        }
    }
}
impl Parse for ArrayExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            exprs: input.parse()?,
        })
    }
}
