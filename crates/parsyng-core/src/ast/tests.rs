use crate as parsyng;

use crate::{ToTokens, quote};

use crate::{
    ast::{
        crate_source::Crate,
        delimiter::{Braced, Bracketed, Parenthesized},
        expression::{
            ArrayElements, ArrayExpression, AwaitExpression, BlockExpression, BreakExpression,
            CallExpression, ContinueExpression, ElseExpression, Expression, ExpressionWithBlock,
            ExpressionWithoutBlock, FieldExpression, GroupedExpression, IfExpression,
            IndexExpression, LoopExpression, RangeExpression, ReturnExpression, TupleExpression,
            TupleIndexExpression, UnderscoreExpression, UnsafeBlockExpression,
        },
        item::{
            GenericParam, GenericParams, Lifetime, LifetimeBounds, LifetimeParam,
            LifetimeWhereClauseItem, TraitBound, TypeBoundWhereClauseItem, TypeParam,
            TypeParamBound, TypeParamBounds, WhereClause, WhereClauseItem, associated::*,
            constant::ConstantItem, implementation::Implementation, r#struct::*,
        },
        literal::{Literal, LiteralFloat, LiteralNumber},
        path::{GenericArg, GenericArgs, SimplePath, TypePathSegment},
        statements::Statement,
        r#type::{Type, TypePath},
        visibility::Visibility,
    },
    parse::ParseBuffer,
    proc_macro::TokenStream,
};

fn ts(input: &str) -> TokenStream {
    input.parse().unwrap()
}

pub(crate) fn parse_exact<T: crate::Parse>(tokens: TokenStream) -> T {
    let mut input = ParseBuffer::new(tokens);
    let value = input.parse::<T>().unwrap();
    assert!(input.is_empty());
    value
}

pub(crate) fn check<T: crate::Parse + ToTokens>(tokens: TokenStream) -> T {
    let expected = tokens.to_string();
    let parsed = parse_exact::<T>(tokens);
    let mut out = TokenStream::new();
    parsed.to_tokens(&mut out);
    assert_eq!(out.to_string(), expected);
    parsed
}

#[test]
fn token_stream_nodes() {
    check::<TokenStream>(quote! { a + b });
    check::<crate::proc_macro::TokenTree>(quote! { a });
    check::<crate::proc_macro::Group>(quote! { (a) });
    check::<crate::proc_macro::Ident>(quote! { ident });
    check::<crate::proc_macro::Punct>(quote! { + });
}

#[test]
fn delimiter_nodes() {
    check::<Bracketed<Expression>>(quote! { [foo] });
    check::<Braced<Vec<Statement>>>(quote! { { ; } });
    check::<Parenthesized<Expression>>(quote! { (foo) });
}

#[test]
fn literal_nodes() {
    let int = check::<LiteralNumber>(ts("123u32"));
    assert_eq!(int.content(), "123");
    assert_eq!(int.prefix(), "");
    assert_eq!(int.suffix(), "u32");

    let float = check::<LiteralFloat>(ts("1.5f32"));
    assert_eq!(float.content(), "1.5");
    assert_eq!(float.suffix(), "f32");

    let lit_int = check::<Literal>(ts("10"));
    assert!(matches!(lit_int, Literal::UInt(_)));

    let lit_float = check::<Literal>(ts("2.0"));
    assert!(matches!(lit_float, Literal::Float(_)));
}

#[test]
fn visibility_nodes() {
    let private = check::<Visibility>(quote! {});
    assert!(matches!(private, Visibility::Private));

    let public = check::<Visibility>(quote! { pub });
    assert!(matches!(public, Visibility::Public(_)));

    let vis_crate = check::<Visibility>(quote! { pub(crate) });
    assert!(matches!(vis_crate, Visibility::Crate(_, _)));

    let vis_self = check::<Visibility>(quote! { pub(self) });
    assert!(matches!(vis_self, Visibility::SelfVis(_, _)));

    let vis_in = check::<Visibility>(quote! { pub(in a::b) });
    assert!(matches!(vis_in, Visibility::PubIn(_, _)));
}

#[test]
fn path_and_type_nodes() {
    check::<SimplePath>(quote! { ::core::fmt });
    check::<TypePathSegment>(quote! { Vec::<u8> });
    check::<GenericArgs>(quote! { <u8, 'a> });

    let type_arg = check::<GenericArg>(quote! { u8 });
    assert!(matches!(type_arg, GenericArg::Type(_)));
    let lt_arg = check::<GenericArg>(quote! { 'a });
    assert!(matches!(lt_arg, GenericArg::Lifetime(_)));

    check::<TypePath>(quote! { ::std::vec::Vec::<u8> });

    let ty = check::<Type>(quote! { std::vec::Vec::<u8> });
    assert!(matches!(ty, Type::Path(_)));
}

#[test]
fn generic_and_where_nodes() {
    check::<Lifetime>(quote! { 'a });

    check::<LifetimeBounds>(quote! { 'a + 'b });
    check::<LifetimeParam>(quote! { 'a: 'b + 'c });
    check::<TraitBound>(quote! { (?for<'a> Foo<'a>) });

    let bound_lt = check::<TypeParamBound>(quote! { 'a });
    assert!(matches!(bound_lt, TypeParamBound::Lifetime(_)));
    let bound_trait = check::<TypeParamBound>(quote! { Foo<'a> });
    assert!(matches!(bound_trait, TypeParamBound::Trait(_)));

    check::<TypeParamBounds>(quote! { Foo<'a> + 'a });
    check::<TypeParam>(quote! { T: Foo<'a> + 'a = U });

    let gp_ty = check::<GenericParam>(quote! { T });
    assert!(matches!(gp_ty, GenericParam::Type(_)));
    let gp_lt = check::<GenericParam>(quote! { 'a: 'b });
    assert!(matches!(gp_lt, GenericParam::Lifetime(_)));

    check::<GenericParams>(quote! { <T, 'a> });
    check::<LifetimeWhereClauseItem>(quote! { 'a: 'b + 'c });
    check::<TypeBoundWhereClauseItem>(quote! { for<'a> T: Foo<'a> + 'a });

    let wc_lt = check::<WhereClauseItem>(quote! { 'a: 'b });
    assert!(matches!(wc_lt, WhereClauseItem::Lifetime(_)));
    let wc_ty = check::<WhereClauseItem>(quote! { T: Foo<'a> });
    assert!(matches!(wc_ty, WhereClauseItem::Type(_)));

    check::<WhereClause>(quote! { where 'a: 'b, T: Foo<'a> });
}

#[test]
fn item_nodes() {
    let field = check::<StructField>(quote! { pub x: u8 });
    assert_eq!(field.ident().to_string(), "x");

    let struct_struct =
        check::<Struct>(quote! { struct Point<T> where T: Copy { pub x: T, } });
    assert_eq!(struct_struct.ident().to_string(), "Point");
    assert!(struct_struct.generic_parameters().is_some());

    let item_struct = check::<Struct>(quote! { struct Unit; });
    assert_eq!(item_struct.ident().to_string(), "Unit");

    check::<ConstantItem>(quote! { const VALUE: u8; });
    check::<TypeAlias>(quote! { type Assoc; });

    let aa_type = check::<AssociatedAlias>(quote! { type Assoc; });
    assert!(matches!(aa_type, AssociatedAlias::TypeAlias(_, _)));
    let aa_const = check::<AssociatedAlias>(quote! { const VALUE: u8; });
    assert!(matches!(aa_const, AssociatedAlias::Const(_, _)));

    check::<Implementation>(quote! { impl Type { type Assoc; const VALUE: u8; } });
    check::<Implementation>(quote! { impl !Trait for Type { type Assoc; } });

    let item_struct = check::<crate::ast::item::Item>(quote! { pub struct S; });
    assert!(matches!(item_struct, crate::ast::item::Item::Struct(_)));
    let item_impl = check::<crate::ast::item::Item>(quote! { impl Type { type Assoc; } });
    assert!(matches!(item_impl, crate::ast::item::Item::Impl(_)));
}

#[test]
fn expression_nodes() {
    let expr = check::<Expression>(quote! { foo });
    assert!(matches!(expr, Expression::WithoutBlock(_)));

    let expr_wo = check::<ExpressionWithoutBlock>(quote! { foo });
    assert!(matches!(expr_wo, ExpressionWithoutBlock::Path(_)));

    let expr_wb = check::<ExpressionWithBlock>(quote! { { ; } });
    assert!(matches!(expr_wb, ExpressionWithBlock::Block(_)));

    check::<BlockExpression>(quote! { 'lbl: { ; } });
    check::<UnsafeBlockExpression>(quote! { unsafe { ; } });
    check::<LoopExpression>(quote! { 'lbl: loop { ; } });
    check::<IfExpression>(quote! { if foo { ; } else { ; } });

    let else_if = check::<ElseExpression>(quote! { if foo { ; } else { ; } });
    assert!(matches!(else_if, ElseExpression::If(_)));
    let else_block = check::<ElseExpression>(quote! { { ; } });
    assert!(matches!(else_block, ElseExpression::Block(_)));

    check::<AwaitExpression>(quote! { { ; }.await });
    check::<IndexExpression>(quote! { { ; }[idx] });
    check::<TupleExpression>(quote! { (a, b) });
    check::<ArrayExpression>(quote! { [a, b] });

    let arr_rep = check::<ArrayElements>(quote! { a; n });
    assert!(matches!(arr_rep, ArrayElements::Repetition(_, _, _)));
    let arr_list = check::<ArrayElements>(quote! { a, b, c });
    assert!(matches!(arr_list, ArrayElements::List(_)));

    check::<TupleIndexExpression>(ts("{ ; }.0"));
    check::<FieldExpression>(quote! { { ; }.field });
    check::<ReturnExpression>(quote! { return foo });
    check::<ContinueExpression>(quote! { continue 'a });
    check::<BreakExpression>(quote! { break 'a foo });
    check::<CallExpression>(quote! { { ; }(foo) });
    check::<RangeExpression>(quote! { ..b });
    check::<UnderscoreExpression>(quote! { _ });
    check::<GroupedExpression>(quote! { (foo) });
}

#[test]
fn statement_and_crate_nodes() {
    let s1 = check::<Statement>(quote! { ; });
    assert!(matches!(s1, Statement::Semicolon(_)));

    let s2 = check::<Statement>(quote! { struct A; });
    assert!(matches!(s2, Statement::Item(_)));

    let s3 = check::<Statement>(quote! { if foo { ; }; });
    assert!(matches!(s3, Statement::ExpressionWithBlock(_, _)));

    let s4 = check::<Statement>(quote! { foo; });
    assert!(matches!(s4, Statement::ExpressionWithoutBlock(_, Some(_))));

    let s5 = check::<Statement>(quote! { foo });
    assert!(matches!(s5, Statement::ExpressionWithoutBlock(_, None)));

    check::<Crate>(quote! {
        pub struct A;
        impl A { type Assoc; const VALUE: u8; }
    });
}
