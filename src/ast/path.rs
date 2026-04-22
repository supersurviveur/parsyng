use parsyng_quote::ToTokens;

use crate::{
    ast::{
        item::Lifetime,
        tokens::{Comma, Gt, Lt, PathSep},
        r#type::Type,
    },
    combinator::{Punctuated, StopOnError},
    error::Diagnostics,
    parse::{Parse, Peekable},
    proc_macro::Ident,
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
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.root.to_tokens(tokens);
        self.paths.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct TypePathSegment {
    path_ident: Ident,
    args: Option<(Option<PathSep>, GenericArgs)>,
}

#[derive(Clone, Debug)]
pub struct GenericArgs {
    start_token: Lt,
    generics: Punctuated<GenericArg, Comma, StopOnError>,
    last_token: Gt,
}

#[derive(Clone, Debug)]
pub enum GenericArg {
    Type(Type),
    Lifetime(Lifetime),
}

impl ToTokens for TypePathSegment {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
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
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            GenericArg::Type(ty) => ty.to_tokens(tokens),
            GenericArg::Lifetime(lifetime) => lifetime.to_tokens(tokens),
        }
    }
}

impl Parse for GenericArg {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(ty) = input.try_parse() {
            Ok(Self::Type(ty))
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
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
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
