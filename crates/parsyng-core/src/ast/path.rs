use crate::ToTokens;
use crate::ast::delimiter::Parenthesized;
use crate::ast::tokens::{Eq, RArrow};

use crate::combinator::Either;
use crate::{
    ast::{
        item::Lifetime,
        tokens::{Comma, Gt, Lt, PathSep},
        r#type::Type,
    },
    combinator::{Punctuated, StopOnError},
    error::Diagnostics,
    parse::{Parse, Peekable},
    proc_macro::{Ident, Span},
};

#[derive(Clone, Debug)]
pub struct SimplePath {
    start_token: Option<PathSep>,
    root: Ident,
    paths: Vec<(PathSep, Ident)>,
}

impl Parse for SimplePath {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            start_token: input.try_parse::<PathSep>().ok(),
            root: input.parse()?,
            paths: input.parse()?,
        })
    }
}

impl ToTokens for SimplePath {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.root.to_tokens(tokens);
        self.paths.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct TypePathSegment {
    path_ident: Ident,
    args: Option<(Option<PathSep>, Either<GenericArgs, TypePathFn>)>,
}

impl TypePathSegment {
    pub fn span(&self) -> Span {
        self.path_ident.span()
    }
}

#[derive(Clone, Debug)]
pub struct TypePathFn {
    inputs: Parenthesized<Option<TypePathFnInputs>>,
    return_type: Option<(RArrow, Box<Type>)>,
}

#[derive(Clone, Debug)]
pub struct TypePathFnInputs {
    args: Punctuated<Comma, Type>,
    trailing_comma: Option<Comma>,
}

#[derive(Clone, Debug)]
pub struct GenericArgs {
    start_token: Lt,
    generics: Punctuated<GenericArg, Comma, StopOnError>,
    last_token: Gt,
}

#[derive(Clone, Debug)]
pub enum GenericArg {
    Type(Box<Type>),
    Lifetime(Lifetime),
    Bindings(Ident, Option<Box<GenericArgs>>, Eq, Box<Type>),
}

impl ToTokens for TypePathSegment {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.path_ident.to_tokens(tokens);
        self.args.to_tokens(tokens);
    }
}

impl Parse for TypePathSegment {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            path_ident: input.parse()?,
            args: input
                .try_parse::<(Option<Peekable<_>>, _)>()
                .ok()
                .map(|(sep, generics)| (sep.map(|sep| sep.inner()), generics)),
        })
    }
}
impl ToTokens for GenericArg {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            GenericArg::Type(ty) => ty.to_tokens(tokens),
            GenericArg::Lifetime(lifetime) => lifetime.to_tokens(tokens),
            GenericArg::Bindings(ident, generics, eq, ty) => {
                (ident, generics, eq, ty).to_tokens(tokens)
            }
        }
    }
}

impl Parse for GenericArg {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok((ident, generics, eq, ty)) = input.try_parse::<(_, Option<Peekable<_>>, _, _)>() {
            Ok(Self::Bindings(
                ident,
                generics.map(|peekable| peekable.inner()),
                eq,
                ty,
            ))
        } else if let Ok(ty) = input.try_parse() {
            Ok(Self::Type(Box::new(ty)))
        } else if let Ok(lifetime) = input.try_parse() {
            Ok(Self::Lifetime(lifetime))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a generic argument",
                input.span(),
            ))
        }
    }
}
impl ToTokens for GenericArgs {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.last_token.to_tokens(tokens);
    }
}
impl Parse for GenericArgs {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            start_token: input.parse()?,
            generics: input.parse()?,
            last_token: input.parse()?,
        })
    }
}
impl ToTokens for TypePathFn {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.inputs.to_tokens(tokens);
        self.return_type.to_tokens(tokens);
    }
}

impl Parse for TypePathFn {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            inputs: input
                .parse::<Parenthesized<Option<Peekable<_>>>>()?
                .map(|inputs| inputs.map(|inner| inner.inner())),
            return_type: input.try_parse().ok(),
        })
    }
}
impl ToTokens for TypePathFnInputs {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.args.to_tokens(tokens);
        self.trailing_comma.to_tokens(tokens);
    }
}

impl Parse for TypePathFnInputs {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            args: input.parse()?,
            trailing_comma: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate as parsyng;
    use parsyng_quote_macros::quote;

    use super::*;
    use crate::ast::tests::check;

    #[test]
    fn test_type_path() {
        check::<TypePathSegment>(quote! {
            Iterator<Item = &Attribute>
        });
    }
}
