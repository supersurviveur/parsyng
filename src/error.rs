use crate::proc_macro::{Span, TokenStream};
use parsyng_quote::{ToTokens, parsyng_spanned};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    content: String,
    span: Span,
}

#[derive(Debug, Clone)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostic {
    #[must_use]
    pub fn new<T: Into<String>>(content: T, span: Span) -> Self {
        Self {
            content: content.into(),
            span,
        }
    }
}
impl Diagnostics {
    #[must_use]
    pub fn empty() -> Self {
        Self(vec![])
    }
    #[must_use]
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self(vec![diagnostic])
    }
    #[must_use]
    pub fn new_error<T: Into<String>>(error: T) -> Self {
        Self::new(Diagnostic::new(error, Span::call_site()))
    }
    #[must_use]
    pub fn new_error_spanned<T: Into<String>>(error: T, span: Span) -> Self {
        Self::new(Diagnostic::new(error, span))
    }
    pub fn join(&mut self, other: Self) {
        self.0.extend(other.0)
    }
}

pub type Result<T> = core::result::Result<T, Diagnostics>;

impl ToTokens for Diagnostic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(parsyng_spanned! { self.span =>
            compile_error!{ #{ self.content } }
        });
    }
}
impl ToTokens for Diagnostics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.iter().for_each(|diagnostic| {
            diagnostic.to_tokens(tokens);
        });
    }
}
