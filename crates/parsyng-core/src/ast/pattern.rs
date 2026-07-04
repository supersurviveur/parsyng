use crate::ToTokens;

use crate::{
    ast::{
        delimiter::Parenthesized,
        tokens::{And, Comma, Mut, Ref},
    },
    combinator::Punctuated,
    error::Diagnostics,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub enum Pattern {
    Ident(PatIdent),
    Wildcard(PatWildcard),
    Tuple(Box<PatTuple>),
    Ref(PatRef),
}

#[derive(Clone, Debug)]
pub struct PatIdent {
    by_ref: Option<Ref>,
    mutability: Option<Mut>,
    ident: Ident,
}

impl Pattern {
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            Pattern::Ident(pat_ident) => Some(&pat_ident.ident),
            Pattern::Wildcard(pat_wildcard) => Some(&pat_wildcard.underscore),
            Pattern::Tuple(_) => None,
            Pattern::Ref(pat_ref) => pat_ref.pat.ident(),
        }
    }
    pub fn mutability(&self) -> Option<&Mut> {
        match self {
            Pattern::Ident(pat_ident) => pat_ident.mutability.as_ref(),
            Pattern::Wildcard(_) => None,
            Pattern::Tuple(_) => None,
            Pattern::Ref(pat_ref) => pat_ref.pat.mutability(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PatWildcard {
    underscore: Ident,
}

#[derive(Clone, Debug)]
pub struct PatTuple {
    elems: Parenthesized<Punctuated<Pattern, Comma>>,
}

#[derive(Clone, Debug)]
pub struct PatRef {
    and_token: And,
    mutability: Option<Mut>,
    pat: Box<Pattern>,
}

impl Parse for Pattern {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(reference) = input.try_parse() {
            Ok(Self::Ref(reference))
        } else if let Ok(tuple) = input.try_parse() {
            Ok(Self::Tuple(Box::new(tuple)))
        } else if let Ok(wildcard) = input.try_parse() {
            Ok(Self::Wildcard(wildcard))
        } else if let Ok(ident) = input.try_parse() {
            Ok(Self::Ident(ident))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a pattern",
                input.span(),
            ))
        }
    }
}

impl Parse for PatIdent {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            by_ref: input.try_parse().ok(),
            mutability: input.try_parse().ok(),
            ident: input.parse()?,
        })
    }
}

impl Parse for PatWildcard {
    #[allow(clippy::cmp_owned)]
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let underscore: Ident = input.parse()?;
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

impl Parse for PatTuple {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            elems: input.parse()?,
        })
    }
}

impl Parse for PatRef {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            and_token: input.parse()?,
            mutability: input.try_parse().ok(),
            pat: Box::new(input.parse()?),
        })
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Pattern::Ident(ident) => ident.to_tokens(tokens),
            Pattern::Wildcard(wildcard) => wildcard.to_tokens(tokens),
            Pattern::Tuple(tuple) => tuple.to_tokens(tokens),
            Pattern::Ref(reference) => reference.to_tokens(tokens),
        }
    }
}

impl ToTokens for PatIdent {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.by_ref.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl ToTokens for PatWildcard {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.underscore.to_tokens(tokens);
    }
}

impl ToTokens for PatTuple {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.elems.to_tokens(tokens);
    }
}

impl ToTokens for PatRef {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.and_token.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.pat.to_tokens(tokens);
    }
}
