use crate::{ToTokens, proc_macro::Delimiter};

use crate::{
    ast::{
        delimiter::{Braced, Bracketed, Parenthesized},
        item::Lifetime,
        literal::{Literal, LiteralNumber},
        statements::Statement,
        tokens::{
            Await, Break, Colon, Comma, Continue, Dot, DotDot, DotDotEq, Else, If, Loop, Return,
            Semicolon, Unsafe,
        },
        r#type::TypePath,
    },
    combinator::Punctuated,
    error::Diagnostics,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub enum Expression {
    WithoutBlock(Box<ExpressionWithoutBlock>),
    WithBlock(Box<ExpressionWithBlock>),
}

#[derive(Clone, Debug)]
pub enum ExpressionWithoutBlock {
    Literal(Literal),
    Path(TypePath),
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
pub enum ExpressionWithBlock {
    Block(BlockExpression),
    Unsafe(UnsafeBlockExpression),
    Loop(LoopExpression),
    If(IfExpression),
}

#[derive(Clone, Debug)]
pub struct BlockExpression {
    label: Option<(Lifetime, Colon)>,
    block: Braced<Vec<Statement>>,
}

#[derive(Clone, Debug)]
pub struct UnsafeBlockExpression {
    unsafe_token: Unsafe,
    block: Braced<Vec<Statement>>,
}

#[derive(Clone, Debug)]
pub struct LoopExpression {
    label: Option<(Lifetime, Colon)>,
    loop_token: Loop,
    block: Braced<Vec<Statement>>,
}

#[derive(Clone, Debug)]
pub struct IfExpression {
    if_token: If,
    condition: Expression,
    block: Braced<Vec<Statement>>,
    else_branch: Option<(Else, ElseExpression)>,
}

#[derive(Clone, Debug)]
pub enum ElseExpression {
    If(Box<IfExpression>),
    Block(Braced<Vec<Statement>>),
}

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
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let expr = if let Some(ident) = input.peek_ident() {
            #[allow(clippy::cmp_owned)]
            if ident.to_string() == "return" {
                Self::Return(input.parse()?)
            } else if ident.to_string() == "break" {
                Self::Break(input.parse()?)
            } else if ident.to_string() == "continue" {
                Self::Continue(input.parse()?)
            } else if ident.to_string() == "_" {
                Self::Underscore(input.parse()?)
            } else {
                Self::Path(input.parse()?)
            }
        } else if let Ok(dot) = input.try_parse::<DotDot>() {
            Self::Range(RangeExpression {
                start: None,
                dot: Some(dot),
                dot_eq: None,
                end: input.try_parse().ok(),
            })
        } else if let Ok(dot_eq) = input.try_parse::<DotDotEq>() {
            Self::Range(RangeExpression {
                start: None,
                dot: None,
                dot_eq: Some(dot_eq),
                end: input.try_parse().ok(),
            })
        } else if let Some(group) = input.peek_group() {
            if group.delimiter() == Delimiter::Parenthesis {
                if let Ok(tuple) = input.try_parse() {
                    Self::Tuple(tuple)
                } else {
                    Self::Grouped(input.parse()?)
                }
            } else if group.delimiter() == Delimiter::Bracket {
                Self::Array(input.parse()?)
            } else {
                return Err(Diagnostics::new_error_spanned(
                    "Expected an expression without block",
                    input.span(),
                ));
            }
        } else if let Ok(literal) = input.try_parse() {
            Self::Literal(literal)
        } else {
            return Err(Diagnostics::new_error_spanned(
                "Expected an expression without block",
                input.span(),
            ));
        };

        if matches!(
            expr,
            Self::Return(_) | Self::Break(_) | Self::Continue(_) | Self::Range(_)
        ) {
            return Ok(expr);
        }

        let mut wrapped = Expression::WithoutBlock(Box::new(expr));
        loop {
            if let Ok(dot) = input.try_parse::<Dot>() {
                if let Ok(await_token) = input.try_parse::<Await>() {
                    wrapped = Expression::WithoutBlock(Box::new(Self::Await(AwaitExpression {
                        expr: wrapped,
                        dot,
                        await_token,
                    })));
                    continue;
                }
                if let Ok(index) = input.try_parse::<LiteralNumber>() {
                    wrapped = Expression::WithoutBlock(Box::new(Self::TupleIndex(
                        TupleIndexExpression {
                            expr: wrapped,
                            dot,
                            index,
                        },
                    )));
                    continue;
                }
                if let Ok(field) = input.try_parse::<Ident>() {
                    wrapped = Expression::WithoutBlock(Box::new(Self::Field(FieldExpression {
                        expr: wrapped,
                        dot,
                        field,
                    })));
                    continue;
                }
                return Err(Diagnostics::new_error_spanned(
                    "Expected `await`, a tuple index, or a field after `.`",
                    input.span(),
                ));
            }
            if let Ok(index) = input.try_parse::<Bracketed<Expression>>() {
                wrapped = Expression::WithoutBlock(Box::new(Self::Index(IndexExpression {
                    expr: wrapped,
                    index,
                })));
                continue;
            }
            if let Ok(params) = input.try_parse::<Parenthesized<Punctuated<Expression, Comma>>>() {
                wrapped = Expression::WithoutBlock(Box::new(Self::Call(CallExpression {
                    expr: wrapped,
                    params,
                })));
                continue;
            }
            if let Ok(dot) = input.try_parse::<DotDot>() {
                wrapped = Expression::WithoutBlock(Box::new(Self::Range(RangeExpression {
                    start: Some(wrapped),
                    dot: Some(dot),
                    dot_eq: None,
                    end: input.try_parse().ok(),
                })));
                continue;
            }
            if let Ok(dot_eq) = input.try_parse::<DotDotEq>() {
                wrapped = Expression::WithoutBlock(Box::new(Self::Range(RangeExpression {
                    start: Some(wrapped),
                    dot: None,
                    dot_eq: Some(dot_eq),
                    end: input.try_parse().ok(),
                })));
                continue;
            }
            break;
        }

        match wrapped {
            Expression::WithoutBlock(without_block) => Ok(*without_block),
            Expression::WithBlock(_) => {
                unreachable!("postfix parsing must keep non-block expression")
            }
        }
    }
}

impl Parse for ExpressionWithBlock {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(unsafe_block) = input.try_parse() {
            Ok(Self::Unsafe(unsafe_block))
        } else if let Ok(if_expr) = input.try_parse() {
            Ok(Self::If(if_expr))
        } else if let Ok(loop_expr) = input.try_parse() {
            Ok(Self::Loop(loop_expr))
        } else if let Ok(block_expr) = input.try_parse() {
            Ok(Self::Block(block_expr))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an expression with block",
                input.span(),
            ))
        }
    }
}

impl Parse for Expression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(block) = input.try_parse() {
            Ok(Self::WithBlock(Box::new(block)))
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
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::Literal(literal) => literal.to_tokens(tokens),
            Self::Path(path) => path.to_tokens(tokens),
            Self::Await(await_expression) => await_expression.to_tokens(tokens),
            Self::Index(index_expression) => index_expression.to_tokens(tokens),
            Self::Tuple(tuple_expression) => tuple_expression.to_tokens(tokens),
            Self::TupleIndex(tuple_index_expression) => {
                tuple_index_expression.to_tokens(tokens);
            }
            Self::Field(field_expression) => field_expression.to_tokens(tokens),
            Self::Return(return_expression) => {
                return_expression.to_tokens(tokens);
            }
            Self::Continue(continue_expression) => {
                continue_expression.to_tokens(tokens);
            }
            Self::Break(break_expression) => break_expression.to_tokens(tokens),
            Self::Underscore(underscore_expression) => {
                underscore_expression.to_tokens(tokens);
            }
            Self::Grouped(grouped_expression) => {
                grouped_expression.to_tokens(tokens);
            }
            Self::Call(call_expression) => call_expression.to_tokens(tokens),
            Self::Range(range_expression) => range_expression.to_tokens(tokens),
            Self::Array(array_expression) => array_expression.to_tokens(tokens),
        }
    }
}

impl ToTokens for ExpressionWithBlock {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::Block(block_expression) => block_expression.to_tokens(tokens),
            Self::Unsafe(unsafe_block_expression) => unsafe_block_expression.to_tokens(tokens),
            Self::Loop(loop_expression) => loop_expression.to_tokens(tokens),
            Self::If(if_expression) => if_expression.to_tokens(tokens),
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::WithoutBlock(expression_without_block) => {
                expression_without_block.to_tokens(tokens);
            }
            Self::WithBlock(expression_with_block) => expression_with_block.to_tokens(tokens),
        }
    }
}

impl ToTokens for AwaitExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.await_token.to_tokens(tokens);
    }
}
impl ToTokens for FieldExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.field.to_tokens(tokens);
    }
}
impl ToTokens for TupleExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.exprs.to_tokens(tokens);
    }
}
impl ToTokens for IndexExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.index.to_tokens(tokens);
    }
}
impl ToTokens for TupleIndexExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.index.to_tokens(tokens);
    }
}
impl ToTokens for ReturnExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.return_token.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
impl ToTokens for ContinueExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.continue_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
    }
}
impl ToTokens for BreakExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.break_token.to_tokens(tokens);
        self.label.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
impl ToTokens for CallExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.expr.to_tokens(tokens);
        self.params.to_tokens(tokens);
    }
}
impl ToTokens for RangeExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.start.to_tokens(tokens);
        self.dot.to_tokens(tokens);
        self.dot_eq.to_tokens(tokens);
        self.end.to_tokens(tokens);
    }
}
impl ToTokens for UnderscoreExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.underscore.to_tokens(tokens);
    }
}
impl ToTokens for GroupedExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
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

impl Parse for AwaitExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            dot: input.parse()?,
            await_token: input.parse()?,
        })
    }
}

impl Parse for IndexExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            index: input.parse()?,
        })
    }
}

impl Parse for TupleIndexExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            dot: input.parse()?,
            index: input.parse()?,
        })
    }
}

impl Parse for FieldExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            dot: input.parse()?,
            field: input.parse()?,
        })
    }
}

impl Parse for CallExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
            params: input.parse()?,
        })
    }
}

impl Parse for RangeExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(dot) = input.try_parse() {
            return Ok(Self {
                start: None,
                dot: Some(dot),
                dot_eq: None,
                end: input.try_parse().ok(),
            });
        }
        if let Ok(dot_eq) = input.try_parse() {
            return Ok(Self {
                start: None,
                dot: None,
                dot_eq: Some(dot_eq),
                end: input.try_parse().ok(),
            });
        }

        let start = input.parse()?;
        if let Ok(dot) = input.try_parse() {
            Ok(Self {
                start: Some(start),
                dot: Some(dot),
                dot_eq: None,
                end: input.try_parse().ok(),
            })
        } else if let Ok(dot_eq) = input.try_parse() {
            Ok(Self {
                start: Some(start),
                dot: None,
                dot_eq: Some(dot_eq),
                end: input.try_parse().ok(),
            })
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected `..` or `..=` in range expression",
                input.span(),
            ))
        }
    }
}

impl Parse for UnderscoreExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let underscore: Ident = input.parse()?;
        #[allow(clippy::cmp_owned)]
        if underscore.to_string() == "_" {
            Ok(Self { underscore })
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected `_`",
                underscore.span(),
            ))
        }
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
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::Repetition(expression, semicolon, expression1) => {
                expression.to_tokens(tokens);
                semicolon.to_tokens(tokens);
                expression1.to_tokens(tokens);
            }
            Self::List(punctuated) => {
                punctuated.to_tokens(tokens);
            }
        }
    }
}
impl ToTokens for ArrayExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.exprs.to_tokens(tokens);
    }
}
impl Parse for ArrayElements {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if input.is_empty() {
            return Ok(Self::List(Punctuated::new()));
        }
        let first = input.parse()?;
        if let Ok(semicolon) = input.peek_parse() {
            return Ok(Self::Repetition(first, semicolon, input.parse()?));
        }
        if input.is_empty() {
            Ok(Self::List(Punctuated::one(first)))
        } else {
            let comma = input.parse()?;
            let mut list: Punctuated<Expression, _> = input.parse()?;
            list.push_back((first, comma));
            Ok(Self::List(list))
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

impl Parse for BlockExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            label: input.try_parse().ok(),
            block: input.parse()?,
        })
    }
}

impl Parse for UnsafeBlockExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            unsafe_token: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl Parse for LoopExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            label: input.try_parse().ok(),
            loop_token: input.parse()?,
            block: input.parse()?,
        })
    }
}

impl Parse for ElseExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(if_expression) = input.try_parse() {
            Ok(Self::If(Box::new(if_expression)))
        } else if let Ok(block) = input.try_parse() {
            Ok(Self::Block(block))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected block expression or `if` after `else`",
                input.span(),
            ))
        }
    }
}

impl Parse for IfExpression {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let if_token = input.parse()?;
        let condition = input.parse()?;
        let block = input.parse()?;

        let else_branch = if let Ok(else_token) = input.peek_parse::<Else>() {
            Some((else_token, input.parse()?))
        } else {
            None
        };

        Ok(Self {
            if_token,
            condition,
            block,
            else_branch,
        })
    }
}

impl ToTokens for BlockExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.label.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl ToTokens for UnsafeBlockExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.unsafe_token.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl ToTokens for LoopExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.label.to_tokens(tokens);
        self.loop_token.to_tokens(tokens);
        self.block.to_tokens(tokens);
    }
}

impl ToTokens for ElseExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::If(if_expression) => if_expression.to_tokens(tokens),
            Self::Block(block) => block.to_tokens(tokens),
        }
    }
}

impl ToTokens for IfExpression {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.if_token.to_tokens(tokens);
        self.condition.to_tokens(tokens);
        self.block.to_tokens(tokens);
        self.else_branch.to_tokens(tokens);
    }
}
