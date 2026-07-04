use crate::{
    ToTokens,
    ast::item::{GenericParam, GenericParams},
    ast::tokens::Comma,
    proc_macro::Span,
};

pub struct ImplGenerics<'a>(&'a GenericParams);
pub struct TypeGenerics<'a>(&'a GenericParams);

impl<'a> From<&'a GenericParams> for ImplGenerics<'a> {
    fn from(value: &'a GenericParams) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a GenericParams> for TypeGenerics<'a> {
    fn from(value: &'a GenericParams) -> Self {
        Self(value)
    }
}

impl ToTokens for ImplGenerics<'_> {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.0.start_token.to_tokens(tokens);
        // generate lifetimes first
        self.0
            .generics
            .iter()
            .filter(|generic| matches!(generic, GenericParam::Lifetime(_)))
            .for_each(|generic| {
                generic.to_tokens(tokens);
                Comma::new(Span::call_site()).to_tokens(tokens);
            });
        // and then other generics
        self.0
            .generics
            .iter()
            .filter(|generic| !matches!(generic, GenericParam::Lifetime(_)))
            .for_each(|generic| {
                generic.to_tokens(tokens);
                Comma::new(Span::call_site()).to_tokens(tokens);
            });
        self.0.last_token.to_tokens(tokens);
    }
}

impl ToTokens for TypeGenerics<'_> {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.0.start_token.to_tokens(tokens);
        self.0.generics.iter().for_each(|generic| {
            match generic {
                GenericParam::Type(type_param) => type_param.ident.to_tokens(tokens),
                GenericParam::Lifetime(lifetime_param) => lifetime_param.to_tokens(tokens),
                GenericParam::Const(const_param) => const_param.ident.to_tokens(tokens),
            }
            Comma::new(Span::call_site()).to_tokens(tokens);
        });
        self.0.last_token.to_tokens(tokens);
    }
}
