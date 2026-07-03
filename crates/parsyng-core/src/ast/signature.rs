use crate::ToTokens;

use crate::{
    ast::{
        delimiter::Parenthesized,
        item::{GenericParams, Lifetime, WhereClause},
        pattern::Pattern,
        tokens::{
            And, Async, Colon, Comma, Const, DotDotDot, Extern, Fn, Mut, RArrow, SelfValue, Unsafe,
        },
        r#type::Type,
    },
    combinator::Punctuated,
    error::Diagnostics,
    parse::Parse,
    proc_macro::{Ident, Literal},
};

#[derive(Clone, Debug)]
pub struct FnSignature {
    const_token: Option<Const>,
    async_token: Option<Async>,
    unsafe_token: Option<Unsafe>,
    extern_token: Option<(Extern, Option<Literal>)>,
    fn_token: Fn,
    ident: Ident,
    generics: Option<GenericParams>,
    params: Parenthesized<Punctuated<FnParam, Comma>>,
    return_type: Option<(RArrow, Type)>,
    where_clause: Option<WhereClause>,
}

#[derive(Clone, Debug)]
pub enum FnParam {
    SelfParam(SelfParam),
    Typed(PatType),
    Variadic(DotDotDot),
}

#[derive(Clone, Debug)]
pub struct SelfParam {
    reference: Option<(And, Option<Lifetime>)>,
    mutability: Option<Mut>,
    self_token: SelfValue,
    typed: Option<(Colon, Type)>,
}

#[derive(Clone, Debug)]
pub struct PatType {
    pat: Pattern,
    colon: Colon,
    ty: Type,
}

impl FnParam {
    pub fn ty(&self) -> Option<&Type> {
        match self {
            FnParam::SelfParam(self_param) => self_param.typed.as_ref().map(|x| &x.1),
            FnParam::Typed(pat_type) => Some(&pat_type.ty),
            FnParam::Variadic(_) => None,
        }
    }
    pub fn ident(&self) -> Option<&Ident> {
        match self {
            FnParam::SelfParam(_) => todo!(),
            FnParam::Typed(pat_type) => pat_type.pat.ident(),
            FnParam::Variadic(_) => None,
        }
    }
}

impl FnSignature {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
    pub fn return_type(&self) -> Option<&Type> {
        self.return_type.as_ref().map(|x| &x.1)
    }
    pub fn args(&self) -> &Punctuated<FnParam, Comma> {
        self.params.inner_ref()
    }
}

impl Parse for FnSignature {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            const_token: input.try_parse().ok(),
            async_token: input.try_parse().ok(),
            unsafe_token: input.try_parse().ok(),
            extern_token: if let Ok(extern_token) = input.try_parse() {
                let abi = input.try_parse().ok();
                Some((extern_token, abi))
            } else {
                None
            },
            fn_token: input.parse()?,
            ident: input.parse()?,
            generics: input.try_parse().ok(),
            params: input.parse()?,
            return_type: input.try_parse().ok(),
            where_clause: input.try_parse().ok(),
        })
    }
}

impl Parse for FnParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(variadic) = input.try_parse() {
            Ok(Self::Variadic(variadic))
        } else if let Ok(self_param) = input.try_parse() {
            Ok(Self::SelfParam(self_param))
        } else if let Ok(pat) = input.try_parse() {
            Ok(Self::Typed(PatType {
                pat,
                colon: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a function parameter",
                input.span(),
            ))
        }
    }
}

impl Parse for SelfParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let mut reference = None;
        let mutability;
        if let Ok(and_token) = input.try_parse::<And>() {
            let lifetime = input.try_parse().ok();
            mutability = input.try_parse().ok();
            reference = Some((and_token, lifetime));
        } else {
            mutability = input.try_parse().ok();
        }
        let self_token: SelfValue = input.parse()?;
        let typed = if let Ok(colon) = input.peek_parse() {
            Some((colon, input.parse()?))
        } else {
            None
        };
        Ok(Self {
            reference,
            mutability,
            self_token,
            typed,
        })
    }
}

impl ToTokens for FnSignature {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.const_token.to_tokens(tokens);
        self.async_token.to_tokens(tokens);
        self.unsafe_token.to_tokens(tokens);
        self.extern_token.to_tokens(tokens);
        self.fn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.params.to_tokens(tokens);
        self.return_type.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
    }
}

impl ToTokens for FnParam {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            FnParam::SelfParam(self_param) => self_param.to_tokens(tokens),
            FnParam::Typed(typed) => typed.to_tokens(tokens),
            FnParam::Variadic(variadic) => variadic.to_tokens(tokens),
        }
    }
}

impl ToTokens for SelfParam {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.reference.to_tokens(tokens);
        self.mutability.to_tokens(tokens);
        self.self_token.to_tokens(tokens);
        self.typed.to_tokens(tokens);
    }
}

impl ToTokens for PatType {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.pat.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
