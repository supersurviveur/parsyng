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

fn parse_exact<T: crate::Parse>(tokens: TokenStream) -> T {
    let mut input = ParseBuffer::new(tokens);
    let value = input.parse::<T>().unwrap();
    assert!(input.is_empty());
    value
}

fn check<T: crate::Parse + ToTokens>(tokens: TokenStream) -> T {
    let expected = tokens.to_string();
    let parsed = parse_exact::<T>(tokens);
    let mut out = TokenStream::new();
    parsed.to_tokens(&mut out);
    assert_eq!(out.to_string(), expected);
    parsed
}

fn rust_src_root() -> Option<PathBuf> {
    if let Ok(path) = env::var("RUST_SRC_PATH") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    let output = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let sysroot = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
    let library = sysroot.join("lib/rustlib/src/rust/library");
    if library.exists() {
        return Some(library);
    }

    let src = sysroot.join("lib/rustlib/src/rust/src");
    if src.exists() {
        return Some(src);
    }

    None
}

fn rust_std_file<F: Fn(String, TokenStream)>(root: &Path, test: F) {
    const CANDIDATES: &[&str] = &[
        // "core/src/option.rs",
        // "core/src/result.rs",
        "core/src/marker.rs",
        // "alloc/src/vec/mod.rs",
        // "std/src/lib.rs",
        "libcore/option.rs",
        "libcore/result.rs",
        "libcore/marker.rs",
        "liballoc/vec/mod.rs",
        "libstd/lib.rs",
    ];

    for candidate in CANDIDATES {
        let path = root.join(candidate);
        if path.is_file() {
            let source = fs::read_to_string(path).unwrap();
            let tokens: TokenStream = source.parse().unwrap();
            test(source, tokens);
            return;
        }
    }
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
        check::<StructStruct>(quote! { struct Point<T> where T: Copy { pub x: T, } });
    assert_eq!(struct_struct.ident().to_string(), "Point");
    assert!(struct_struct.generic_parameters().is_some());
    assert_eq!(struct_struct.fields().unwrap().len(), 1);

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

    let item_extern = check::<crate::ast::item::Item>(quote! { extern crate core as realcore; });
    assert!(matches!(
        item_extern,
        crate::ast::item::Item::ExternCrate(_)
    ));
    let item_use = check::<crate::ast::item::Item>(quote! { pub use core::fmt::*; });
    assert!(matches!(item_use, crate::ast::item::Item::Use(_)));
    let item_mod = check::<crate::ast::item::Item>(quote! { pub mod sync; });
    assert!(matches!(item_mod, crate::ast::item::Item::Mod(_)));
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

#[test]
fn parse_rust_std_file() {
    let root = match rust_src_root() {
        Some(root) => root,
        None => {
            println!("Warning: Failed to find std sources to test on a file.");
            return;
        }
    };
    rust_std_file(&root, |source, tokens| {
        let tokens_clone1 = tokens.clone();
        let tokens_clone2 = tokens.clone();
        // Manual parse loop to discover the failing item
        let mut input = ParseBuffer::new(tokens_clone1);
        let inner_attrs = crate::ast::attributes::parse_inner_attributes(&mut input);
        eprintln!("Consumed {} inner attributes", inner_attrs.len());
        let mut idx = 0usize;
        loop {
            if input.is_empty() { break; }
            match input.parse::<crate::ast::item::Item>() {
                Ok(it) => {
                    let name = match &it {
                        crate::ast::item::Item::Struct(_) => "Struct",
                        crate::ast::item::Item::Const(_) => "Const",
                        crate::ast::item::Item::TypeAlias(_) => "TypeAlias",
                        crate::ast::item::Item::Use(_) => "Use",
                        crate::ast::item::Item::ExternCrate(_) => "ExternCrate",
                        crate::ast::item::Item::ExternBlock(_) => "ExternBlock",
                        crate::ast::item::Item::Mod(_) => "Mod",
                        crate::ast::item::Item::Enum(_) => "Enum",
                        crate::ast::item::Item::Function(_) => "Function",
                        crate::ast::item::Item::Trait(_) => "Trait",
                        crate::ast::item::Item::Static(_) => "Static",
                        crate::ast::item::Item::MacroRules(_) => "MacroRules",
                        crate::ast::item::Item::Macro(_) => "Macro",
                        crate::ast::item::Item::MacroInvocation(_) => "MacroInvocation",
                        crate::ast::item::Item::Impl(_) => "Impl",
                    };
                    let mut ts = TokenStream::new();
                    it.to_tokens(&mut ts);
                    eprintln!("Parsed item {}: {} -- tokens: {}", idx, name, ts.to_string());
                    idx += 1;
                }
                Err(err) => {
                    eprintln!("Failed parsing item {}: {:?}", idx, err);
                    // try parsing an Implementation here to get a focused error (consuming outer attrs first)
                    let mut impl_input = input.clone();
                    let parsed_attrs = crate::ast::attributes::parse_outer_attributes(&mut impl_input);
                    eprintln!("Outer attrs before impl: {}", parsed_attrs.len());
                    match impl_input.parse::<crate::ast::item::implementation::Implementation>() {
                        Ok(impl_item) => eprintln!("Implementation parsed: {:?}", impl_item),
                        Err(e) => eprintln!("Parsing Implementation failed: {:?}", e),
                    }
                    // try input.try_parse to see if try_parse detects Implementation
                    match input.try_parse::<crate::ast::item::implementation::Implementation>() {
                        Ok(impl_item) => eprintln!("input.try_parse::<Implementation>() succeeded: {:?}", impl_item),
                        Err(e) => eprintln!("input.try_parse::<Implementation>() failed: {:?}", e),
                    }
                    // show remaining tokens at failure point
                    let mut rem = input.clone();
                    let mut remaining = TokenStream::new();
                    while let Some(tt) = rem.next() { remaining.extend(Some(tt)); }
                    eprintln!("Remaining tokens:\n{}", remaining.to_string());
                    eprintln!("Source head:\n{}", source.chars().take(1200).collect::<String>());
                    panic!("parse failed");
                }
            }
        }
        eprintln!("Parsed {} items successfully", idx);

    });
}

#[test]
fn debug_impl_parse() {
    let tokens: TokenStream = quote! { #[stable(feature = "rust1", since = "1.0.0")] impl<T: PointeeSized> !Send for *const T {} };
    let mut input = ParseBuffer::new(tokens);
    let attrs = crate::ast::attributes::parse_outer_attributes(&mut input);
    eprintln!("Outer attrs: {}", attrs.len());
    match input.parse::<crate::ast::visibility::Visibility>() {
        Ok(vis) => eprintln!("Visibility parsed: {:?}", vis),
        Err(e) => eprintln!("Visibility parse error (expected none): {:?}", e),
    }
    match input.parse::<crate::ast::item::implementation::Implementation>() {
        Ok(impl_item) => eprintln!("Implementation parsed directly: {:?}", impl_item),
        Err(err) => eprintln!("Failed to parse Implementation directly: {:?}", err),
    }
}
